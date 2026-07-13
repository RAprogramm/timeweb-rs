#!/usr/bin/env python3
"""Keep the README statistics in sync with the generated SDK.

The README quotes the number of API operations and areas and lists every
generated ``apis`` module. Those figures are derived facts: one module per
API area lives in ``src/apis/<area>_api.rs`` and every operation is a
``pub async fn`` inside one of those modules. This script recomputes both
from the working tree and rewrites the two README fragments that quote
them, so a regeneration commit can never leave the documentation stale.

Usage:

    python3 openapi/update_readme.py [--check]

Without flags the README is rewritten in place (a no-op when it is already
current). With ``--check`` nothing is written and the exit status is 1 when
the README is out of date, which makes the script usable as a CI guard.

The script is intentionally dependency-free and deterministic: the module
list is sorted and wrapped at 79 columns, so consecutive runs produce
byte-identical output.
"""

from __future__ import annotations

import argparse
import re
import sys
import textwrap
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
README = REPO_ROOT / "README.md"
APIS_DIR = REPO_ROOT / "src" / "apis"

OPERATION_RE = re.compile(r"^pub async fn ", re.MULTILINE)
STATS_RE = re.compile(r"\*\*\d+( operations across\s+)\d+( areas\*\*)")
COVERAGE_RE = re.compile(
    r"(## API coverage\n\n)One module per API area:.*?\.\n", re.DOTALL
)


def collect_modules() -> list[str]:
    """Return the sorted generated API module names, e.g. ``servers_api``."""
    modules = sorted(path.stem for path in APIS_DIR.glob("*_api.rs"))
    if not modules:
        raise SystemExit(f"no *_api.rs modules found under {APIS_DIR}")
    return modules


def count_operations(modules: list[str]) -> int:
    """Count every generated operation across the API modules."""
    return sum(
        len(OPERATION_RE.findall((APIS_DIR / f"{module}.rs").read_text()))
        for module in modules
    )


def render_coverage(modules: list[str]) -> str:
    """Render the wrapped ``## API coverage`` paragraph body."""
    names = ", ".join(f"`{module}`" for module in modules)
    paragraph = f"One module per API area: {names}."
    return textwrap.fill(paragraph, width=79) + "\n"


def updated_readme(text: str) -> str:
    """Return the README text with both derived fragments recomputed."""
    modules = collect_modules()
    operations = count_operations(modules)

    text, stats_hits = STATS_RE.subn(
        lambda match: f"**{operations}{match.group(1)}{len(modules)}{match.group(2)}",
        text,
    )
    if stats_hits != 1:
        raise SystemExit(
            f"expected exactly one operations-count fragment, found {stats_hits}"
        )

    coverage = render_coverage(modules)
    text, coverage_hits = COVERAGE_RE.subn(
        lambda match: match.group(1) + coverage, text
    )
    if coverage_hits != 1:
        raise SystemExit(
            f"expected exactly one API-coverage paragraph, found {coverage_hits}"
        )

    return text


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--check",
        action="store_true",
        help="exit 1 if the README is stale instead of rewriting it",
    )
    args = parser.parse_args()

    current = README.read_text()
    fresh = updated_readme(current)

    if fresh == current:
        print("README is up to date.")
        return 0
    if args.check:
        print("README statistics are stale; run openapi/update_readme.py.")
        return 1
    README.write_text(fresh)
    print("README statistics updated.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
