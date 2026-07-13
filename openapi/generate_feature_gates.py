#!/usr/bin/env python3
"""Gate every API area behind its own Cargo feature.

The crate ships 22+ generated API areas and ~580 generated models; a
consumer who needs one area should not compile the rest. This script
derives the gating from the generated code itself, so a regeneration only
has to re-run it to stay correct:

1. every ``src/apis/<area>_api.rs`` becomes a Cargo feature named after
   the area (underscores become dashes);
2. the models each area actually uses are computed as the transitive
   closure of ``models::Type`` references, starting from the area's API
   file and following references between model files;
3. ``src/apis/mod.rs`` and ``src/models/mod.rs`` are rewritten in place:
   each ``pub mod``/``pub use`` line gets a ``#[cfg(feature = ...)]``
   (single area) or ``#[cfg(any(...))]`` (shared) attribute. A model used
   by every area — or, conservatively, by none — stays unconditional;
4. the marker-delimited block in ``Cargo.toml`` is regenerated with one
   empty feature per area plus a ``full`` aggregate. ``full`` is part of
   the default features, so default builds are unchanged and the gating
   is purely additive.

Usage:

    python3 openapi/generate_feature_gates.py

Run it after regenerating ``src/apis``/``src/models`` (the attributes it
wrote previously are stripped before the new ones are inserted, so it is
idempotent) and follow with ``cargo +nightly fmt``.
"""

from __future__ import annotations

import re
import sys
from collections import deque
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
APIS_DIR = REPO_ROOT / "src" / "apis"
MODELS_DIR = REPO_ROOT / "src" / "models"
CARGO_TOML = REPO_ROOT / "Cargo.toml"

BEGIN_MARK = "# BEGIN generated area features — openapi/generate_feature_gates.py"
END_MARK = "# END generated area features"

TYPE_REF_RE = re.compile(r"\bmodels::([A-Za-z0-9_]+)")
EXPORT_RE = re.compile(r"pub use self::(\w+)::(\w+);")
CFG_LINE_RE = re.compile(r"^\s*#\[cfg\((?:feature|any)[^\n]*\)\]\s*\n", re.MULTILINE)


def feature_name(area: str) -> str:
    return area.replace("_", "-")


def cfg_attribute(areas: list[str]) -> str:
    features = sorted(feature_name(area) for area in areas)
    if len(features) == 1:
        return f'#[cfg(feature = "{features[0]}")]'
    clauses = ", ".join(f'feature = "{feature}"' for feature in features)
    return f"#[cfg(any({clauses}))]"


def type_to_module() -> dict[str, str]:
    mapping: dict[str, str] = {}
    for module, exported in EXPORT_RE.findall((MODELS_DIR / "mod.rs").read_text()):
        mapping[exported] = module
    if not mapping:
        raise SystemExit("no model exports found in src/models/mod.rs")
    return mapping


def module_references(types: dict[str, str]) -> dict[str, set[str]]:
    """Modules each model module references, via `models::Type` edges."""
    edges: dict[str, set[str]] = {}
    for source in MODELS_DIR.glob("*.rs"):
        if source.name == "mod.rs":
            continue
        referenced = {
            types[name] for name in TYPE_REF_RE.findall(source.read_text()) if name in types
        }
        referenced.discard(source.stem)
        edges[source.stem] = referenced
    return edges


def area_closures(
    types: dict[str, str], edges: dict[str, set[str]]
) -> tuple[list[str], dict[str, set[str]]]:
    """Each area's transitively referenced model modules."""
    areas: list[str] = []
    closures: dict[str, set[str]] = {}
    for source in sorted(APIS_DIR.glob("*_api.rs")):
        area = source.stem.removesuffix("_api")
        areas.append(area)
        queue = deque(
            types[name] for name in TYPE_REF_RE.findall(source.read_text()) if name in types
        )
        closure: set[str] = set()
        while queue:
            module = queue.popleft()
            if module in closure:
                continue
            closure.add(module)
            queue.extend(edges.get(module, ()))
        closures[area] = closure
    if not areas:
        raise SystemExit("no *_api.rs modules found under src/apis")
    return areas, closures


def gate_apis_mod(areas: list[str]) -> None:
    path = APIS_DIR / "mod.rs"
    text = CFG_LINE_RE.sub("", path.read_text())
    for area in areas:
        text = text.replace(
            f"pub mod {area}_api;",
            f'#[cfg(feature = "{feature_name(area)}")]\npub mod {area}_api;'
        )
    path.write_text(text)


def unconditional_modules(
    edges: dict[str, set[str]], closures: dict[str, set[str]]
) -> set[str]:
    """Modules that must always compile.

    A module referenced by no area (the generator emits a few orphans)
    stays unconditional, and so must everything it transitively
    references — otherwise the orphan would fail to compile in a partial
    build.
    """
    gated = set().union(*closures.values()) if closures else set()
    queue = deque(module for module in edges if module not in gated)
    always: set[str] = set()
    while queue:
        module = queue.popleft()
        if module in always:
            continue
        always.add(module)
        queue.extend(edges.get(module, ()))
    return always


def gate_models_mod(
    areas: list[str], closures: dict[str, set[str]], always: set[str]
) -> None:
    used_by: dict[str, list[str]] = {}
    for area in areas:
        for module in closures[area]:
            used_by.setdefault(module, []).append(area)

    path = MODELS_DIR / "mod.rs"
    text = path.read_text()
    modules = list(dict.fromkeys(re.findall(r"^\s*pub mod (\w+);", text, re.MULTILINE)))
    exports: dict[str, list[str]] = {}
    for module, exported in re.findall(
        r"^\s*pub use self::(\w+)::(\w+);", text, re.MULTILINE
    ):
        exports.setdefault(module, []).append(exported)
    if len(modules) != len(exports):
        raise SystemExit(
            f"src/models/mod.rs has {len(modules)} modules but exports for {len(exports)}"
        )

    output: list[str] = []
    for module in modules:
        attribute = ""
        if module not in always:
            areas_using = used_by.get(module, [])
            if areas_using and len(areas_using) < len(areas):
                attribute = cfg_attribute(areas_using) + "\n"
        output.append(f"{attribute}pub mod {module};\n")
        for exported in exports[module]:
            output.append(f"{attribute}pub use self::{module}::{exported};\n")
    path.write_text("".join(output))


def rewrite_cargo_features(areas: list[str]) -> None:
    text = CARGO_TOML.read_text()
    if BEGIN_MARK not in text or END_MARK not in text:
        raise SystemExit(f"markers not found in Cargo.toml; add {BEGIN_MARK!r} and {END_MARK!r}")
    features = [f"{feature_name(area)} = []" for area in areas]
    full = ", ".join(f'"{feature_name(area)}"' for area in areas)
    block = "\n".join([BEGIN_MARK, *features, f"full = [{full}]", END_MARK])
    text = re.sub(
        re.escape(BEGIN_MARK) + r".*?" + re.escape(END_MARK), block, text, flags=re.DOTALL
    )
    CARGO_TOML.write_text(text)


def main() -> int:
    types = type_to_module()
    edges = module_references(types)
    areas, closures = area_closures(types, edges)
    always = unconditional_modules(edges, closures)

    gate_apis_mod(areas)
    gate_models_mod(areas, closures, always)
    rewrite_cargo_features(areas)

    total = len(edges)
    gated = {
        module
        for area in areas
        for module in closures[area]
        if module not in always
    }
    print(
        f"gated {len(areas)} areas; {total} model modules "
        f"({total - len(gated)} unconditional, {len(gated)} feature-gated)"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
