#!/usr/bin/env bash
#
# Renders every diagram source of this directory into an SVG.
#
# The documentation embeds the SVG files, so a change to a source has to be
# rendered and committed together with it. The CI does the same render into a
# temporary directory and fails when the result differs from docs/img.
#
# Usage: docs/diagrams/render.sh [output-directory]   (default: docs/img)

set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
out="${1:-$(cd "${here}/.." && pwd)/img}"

if ! command -v d2 > /dev/null; then
  echo "d2 is not installed: see https://d2lang.com/tour/install" >&2
  exit 1
fi

mkdir -p "${out}"

for source in "${here}"/*.d2; do
  name="$(basename -- "${source}" .d2)"
  d2 --layout=elk --theme=1 --pad=16 "${source}" "${out}/${name}.svg"
  chmod 644 "${out}/${name}.svg"
done
