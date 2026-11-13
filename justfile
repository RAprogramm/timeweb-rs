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

# Build the documentation of the crate without dependencies.
doc-check:
    cargo doc --no-deps --all-features

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

# Regenerate the derived files and verify the committed code matches the pipeline.
generated-check:
    cargo xtask layout
    cargo xtask feature-gates
    cargo xtask patch-models
    cargo xtask example-tests
    cargo xtask readme
    cargo +nightly fmt --all
    git diff --exit-code

# Audit dependencies, licenses and sources of the lockfile.
deny:
    cargo deny check

# Check REUSE compliance of the repository.
reuse:
    reuse lint

# Lint the workflow and the YAML files.
yaml-check:
    actionlint
    yamllint .

# Verify the MSRV badge and compile with the minimum supported toolchain.
# CI pins the exact toolchain from Cargo.toml; the local stable must match it.
msrv:
    #!/usr/bin/env bash
    set -euo pipefail
    expected=$(jq -r '.message' .github/msrv-badge.json)
    actual=$(sed -nE 's/^rust-version = "([0-9]+\.[0-9]+).*$$/\1/p' Cargo.toml | head -1)
    test "$expected" = "$actual" || { echo "badge says $expected, Cargo.toml says $actual"; exit 1; }
    cargo check --all-features --all-targets

# Check that every feature combination compiles.
features-check:
    cargo hack --each-feature --no-dev-deps check

# Run the test suite under coverage and enforce 95% line coverage of the
# hand-written code.
cov:
    cargo llvm-cov --workspace --all-features nextest --ignore-filename-regex 'src/(apis|models)\.rs|src/(apis|models)/' --fail-under-lines 95

# Full gate mirroring CI: formatting, lints, tests, examples, documentation,
# generated code and the repository audits.
check: fmt-check lint test examples doc-check docs-check generated-check deny reuse yaml-check
