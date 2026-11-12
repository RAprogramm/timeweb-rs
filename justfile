# SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
# SPDX-License-Identifier: MIT

# Development gates mirroring the CI workflows.

# Reformat the workspace.
fmt:
    cargo +nightly fmt --all

# Verify formatting.
fmt-check:
    cargo +nightly fmt --all --check

# Lint the whole workspace with pedantic and nursery lints.
lint:
    cargo clippy --workspace --all-targets --all-features -- -D warnings -W clippy::pedantic -W clippy::nursery

# Run the test suite including doc tests.
test:
    cargo test --all-features

# Build the examples.
examples:
    cargo build --all-features --examples

# Verify the generated book pages, test and build the book.
# Since Cargo 1.100 the intermediate artifact layout is not a stable interface,
# so the doctest library search path is collected from Cargo's JSON messages.
docs-check:
    #!/usr/bin/env bash
    set -euo pipefail
    cargo xtask api-index --check
    cargo build --tests
    lib_dirs="$(cargo build --tests --message-format=json \
      | jq -r 'select(.reason == "compiler-artifact") | .filenames[]?' \
      | xargs -r -n1 dirname | sort -u | paste -sd, -)"
    mdbook test docs/en -L "$lib_dirs"
    mdbook test docs/ru -L "$lib_dirs"
    mdbook build docs/en -d target/book/en
    mdbook build docs/ru -d target/book/ru

# Check that every feature combination compiles.
features-check:
    cargo hack --each-feature --no-dev-deps check

# Full gate mirroring CI: formatting, lints, tests, examples and the book.
check: fmt-check lint test examples docs-check
