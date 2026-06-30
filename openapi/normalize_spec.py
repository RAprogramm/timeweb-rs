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

3. ``response_id`` nullability. The shared ``response_id`` schema is a required
   ``uuid`` in the response wrappers, but the live API returns ``null`` on some
   endpoints (e.g. ``GET /account/finances``). It is marked ``nullable`` so the
   generated client deserializes the null to ``None`` instead of failing with
   "invalid type: null, expected a formatted UUID string".

Request and response schemas are otherwise left exactly as upstream published
them.

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


def nullable_response_id(spec):
    """Mark ``response_id`` nullable so a ``null`` from the API deserializes.

    The upstream spec types ``response_id`` as a required ``uuid`` and the
    response wrappers list it in ``required``, so the generator emits
    ``response_id: uuid::Uuid``. The live API does not honour this — some
    endpoints (e.g. ``GET /account/finances``) return ``response_id: null``,
    which then fails deserialization with "invalid type: null, expected a
    formatted UUID string". Marking the shared schema (and any inline
    occurrence) nullable yields ``Option<uuid::Uuid>``, so ``null`` maps to
    ``None`` instead of erroring.
    """
    schemas = spec.get("components", {}).get("schemas", {})
    shared = schemas.get("response_id")
    if isinstance(shared, dict):
        shared["nullable"] = True

    def walk(node):
        if isinstance(node, dict):
            props = node.get("properties")
            if isinstance(props, dict):
                inline = props.get("response_id")
                if isinstance(inline, dict) and "$ref" not in inline:
                    inline["nullable"] = True
            for value in node.values():
                walk(value)
        elif isinstance(node, list):
            for value in node:
                walk(value)

    walk(spec)


def add_undocumented_account_status_fields(spec):
    """Add ``status`` fields the live API returns but the spec omits.

    ``GET /account/status`` returns ``login`` (the account identifier shown to
    the user), ``registered_at``, ``two_factor_method`` and ``is_password_set``,
    but the upstream ``status`` schema documents none of them — it stops at
    ``ym_client_id``. Without ``login`` a client is left with only the
    hoster's ``company_info`` (the same "ООО ТАЙМВЭБ.КЛАУД" for every account),
    which is useless for identifying the account.

    The fields are added here, defensively (not marked required), so the
    generator emits ``Option`` types and a missing field never breaks
    deserialization. Run on every regeneration, so it survives upstream spec
    syncs automatically.
    """
    status = spec.get("components", {}).get("schemas", {}).get("status")
    if not isinstance(status, dict):
        return
    props = status.setdefault("properties", {})
    additions = {
        "login":           {"type": "string"},
        "registered_at":   {"type": "string"},
        "is_password_set": {"type": "boolean"},
        "two_factor_method": {"type": "string", "nullable": True},
    }
    for name, schema in additions.items():
        props.setdefault(name, schema)


_RENAMED_PROPERTIES = {
    "ssh-keys": "ssh_keys",
    "knowledgebases": "knowledge_bases",
}


def rename_mismatched_properties(spec):
    """Rename response properties whose spec name differs from the API's.

    Some list responses name their collection property differently from what
    the live API sends, so the generated client deserializes an empty list (or
    fails on a missing field):

    * ``GET /api/v1/ssh-keys`` — spec ``ssh-keys`` vs API ``ssh_keys``;
    * ``GET /api/v1/cloud-ai/knowledge-bases`` — spec ``knowledgebases`` vs API
      ``knowledge_bases``.

    Rename the property (and any matching ``required`` entry) to the spelling
    the API actually sends, wherever it appears.
    """

    def walk(node):
        if isinstance(node, dict):
            props = node.get("properties")
            if isinstance(props, dict):
                for old, new in _RENAMED_PROPERTIES.items():
                    if old in props:
                        props[new] = props.pop(old)
                        required = node.get("required")
                        if isinstance(required, list):
                            node["required"] = [
                                new if name == old else name for name in required
                            ]
            for value in node.values():
                walk(value)
        elif isinstance(node, list):
            for value in node:
                walk(value)

    walk(spec)


_MINIMAL_REQUIRED = {
    "app": ["id", "name", "status"],
}


def relax_overstrict_required(spec):
    """Trim ``required`` lists that demand fields the live API omits.

    Some list-item schemas mark many fields ``required`` that the live API
    does not actually send. The ``app`` schema, for example, requires
    ``framework``, ``branch_name`` and ``server_id`` — none of which appear in
    a ``GET /api/v1/apps`` item (the API sends ``branch``, not ``branch_name``,
    and no ``framework``). One missing required field fails deserialization of
    the whole collection, so the dashboard shows zero apps despite the account
    having several.

    Reduce each listed schema's ``required`` to the minimal identifying fields
    the API reliably returns; the rest become ``Option`` and tolerate absence.
    """
    schemas = spec.get("components", {}).get("schemas", {})
    for name, minimal in _MINIMAL_REQUIRED.items():
        sch = schemas.get(name)
        if not isinstance(sch, dict) or not isinstance(sch.get("required"), list):
            continue
        props = sch.get("properties", {})
        sch["required"] = [field for field in minimal if field in props]


def relax_open_enums(spec):
    """Drop closed enums on fields the API extends with new values over time.

    Several string enums in the spec are effectively open-ended and lag the
    live API, so a value the spec has not caught up to fails deserialization
    and the whole collection comes back empty:

    * availability zones (``ru-1``/``pl-1``/``nl-1``/``de-1``/...) — the shared
      ``Location``/``location`` schemas plus ~17 inline copies;
    * database engines (``mysql``/``postgres17``/``postgres18``/...);
    * floating-ip ``resource_type`` (``server``/``balancer``/``database``/
      ``network`` — the API also returns ``dbaas``).

    Strip the enum (keeping ``type: string``) wherever one of these appears, so
    any present or future value deserializes as a plain string. Detected by a
    distinctive member so unrelated enums (statuses, etc.) are left intact.
    """
    resource_kinds = {"server", "balancer", "database", "network"}

    def is_open(values):
        members = set(values)
        return (
            "ru-1" in members
            or "postgres14" in members
            or members == resource_kinds
        )

    def walk(node):
        if isinstance(node, dict):
            enum = node.get("enum")
            if isinstance(enum, list) and is_open(enum):
                node.pop("enum", None)
                node.setdefault("type", "string")
            for value in node.values():
                walk(value)
        elif isinstance(node, list):
            for value in node:
                walk(value)

    walk(spec)


def nullable_vpc_optional_fields(spec):
    """Mark VPC string fields nullable that the live API returns as ``null``.

    The ``vpc`` schema types ``description`` and ``public_ip`` as required
    strings, but ``GET /api/v2/vpcs`` returns ``null`` for them, failing with
    "invalid type: null, expected a string". Mark them nullable so they map to
    ``Option<String>`` instead.
    """
    vpc = spec.get("components", {}).get("schemas", {}).get("vpc")
    if not isinstance(vpc, dict):
        return
    props = vpc.get("properties", {})
    for name in ("description", "public_ip"):
        field = props.get(name)
        if isinstance(field, dict) and "$ref" not in field:
            field["nullable"] = True


def normalize(spec):
    """Apply all normalization passes to ``spec`` in place and return it."""
    fix_path_parameters(spec)
    localize_tags(spec)
    nullable_response_id(spec)
    add_undocumented_account_status_fields(spec)
    rename_mismatched_properties(spec)
    relax_overstrict_required(spec)
    relax_open_enums(spec)
    nullable_vpc_optional_fields(spec)
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
