#!/usr/bin/env bash
# Writes the README.md of each crate from the `//!` docs in its lib.rs.
# The root README.md comes from the docs of the pico-sdk crate.
# Only READMEs with cargo-rdme markers change.
# With --check, it changes nothing and fails when a README is out of date.
set -euo pipefail

cd "$(dirname "$0")/.."

args="--force"
if [ "${1:-}" = "--check" ]; then
  args="--check"
fi

for readme in */README.md; do
  grep -q '<!-- cargo-rdme start -->' "$readme" || continue
  echo "$readme"
  (cd "$(dirname "$readme")" && cargo rdme $args)
done

echo README.md
(cd sdk && cargo rdme $args --readme-path ../README.md)
