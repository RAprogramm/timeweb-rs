#!/usr/bin/env python3
"""Normalize the Timeweb Cloud OpenAPI spec before client generation.

The upstream spec (https://timeweb.cloud/api-docs-data/bundle.json) needs two
adjustments before a Rust client can be generated cleanly.

1. Path-parameter defects that otherwise produce non-compiling code:

   * a parameter declared ``in: path`` whose name has no ``{placeholder}`` in
     the route — the generated client emits an unused format argument;
   * a ``{placeholder}`` segment in a route with no declared parameter — the
     generated client references an undefined variable;
   * a parameter spelled with ``-`` where the route placeholder uses ``_``.

   Orphan path parameters are moved to ``in: query``, separator-mismatched
   names are renamed to the placeholder spelling, and missing declarations are
   added (schema borrowed from ``components/parameters`` when a match exists).

2. Tag localization. Every tag in the upstream spec is written in Russian,
   which the generator cannot turn into Rust module names — all operations
   collapse into a single ``default_api`` module. Each tag carries its English
   name in ``x-name-i18n.eng`` (operations additionally in ``x-tags-i18n``);
   this script swaps the Russian tags for those English names so the generator
   emits one module per API area (``servers_api``, ``databases_api``, ...).

Nothing else is touched: request and response schemas are left exactly as
upstream published them.

Usage:
    normalize_spec.py <input.json> <output.json>
"""

import json
import re
import sys

_METHODS = ("get", "post", "put", "patch", "delete")


def placeholders(route):
    """Return the ``{name}`` placeholders embedded in a route template."""
    return re.findall(r"{([^}]+)}", route)


def resolve(param, components):
    """Resolve a ``$ref`` parameter against ``components``, else return it."""
    ref = param.get("$ref")
    if ref and ref.startswith("#/components/parameters/"):
        return components.get(ref.split("/")[-1], param)
    return param


def fix_path_parameters(spec):
    """Make every route's path parameters consistent with its template."""
    components = spec.get("components", {}).get("parameters", {})
    comp_by_norm = {}
    for comp in components.values():
        if comp.get("in") == "path" and comp.get("name"):
            comp_by_norm[comp["name"].replace("-", "_")] = comp

    for route, item in spec.get("paths", {}).items():
        names = placeholders(route)

        def fix_list(params):
            for index, param in enumerate(params):
                resolved = resolve(param, components)
                if resolved.get("in") != "path":
                    continue
                name = resolved.get("name")
                if not name or name in names:
                    continue
                inlined = dict(resolved)
                normalized = name.replace("-", "_")
                if normalized in names:
                    inlined["name"] = normalized
                else:
                    inlined["in"] = "query"
                params[index] = inlined

        shared = item.get("parameters")
        if shared:
            fix_list(shared)

        for method, operation in item.items():
            if method not in _METHODS or not isinstance(operation, dict):
                continue
            params = operation.setdefault("parameters", [])
            fix_list(params)

            declared = set()
            for param in (shared or []) + params:
                resolved = resolve(param, components)
                if resolved.get("in") == "path" and resolved.get("name"):
                    declared.add(resolved["name"])

            for name in names:
                if name in declared:
                    continue
                comp = comp_by_norm.get(name.replace("-", "_"))
                schema = comp.get("schema") if comp else None
                params.append(
                    {
                        "name": name,
                        "in": "path",
                        "required": True,
                        "schema": schema or {"type": "string"},
                    }
                )


def localize_tags(spec):
    """Replace Russian tag names with the English names the spec carries."""
    mapping = {}
    for tag in spec.get("tags", []):
        english = (tag.get("x-name-i18n") or {}).get("eng")
        if english:
            mapping[tag["name"]] = english
            tag["name"] = english

    for item in spec.get("paths", {}).values():
        for method, operation in item.items():
            if method not in _METHODS or not isinstance(operation, dict):
                continue
            english = (operation.get("x-tags-i18n") or {}).get("eng")
            if english:
                operation["tags"] = list(english)
            elif "tags" in operation:
                operation["tags"] = [
                    mapping.get(tag, tag) for tag in operation["tags"]
                ]


def normalize(spec):
    """Apply all normalization passes to ``spec`` in place and return it."""
    fix_path_parameters(spec)
    localize_tags(spec)
    return spec


def main():
    if len(sys.argv) != 3:
        sys.exit(f"usage: {sys.argv[0]} <input.json> <output.json>")
    with open(sys.argv[1], encoding="utf-8") as src:
        spec = json.load(src)
    normalize(spec)
    with open(sys.argv[2], "w", encoding="utf-8") as dst:
        json.dump(spec, dst, ensure_ascii=False, indent=1)


if __name__ == "__main__":
    main()
