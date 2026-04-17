set shell := ["bash", "-euo", "pipefail", "-c"]

default:
  @just --list --unsorted

build:
  cargo build

run *args:
  cargo run -- {{args}}

check:
  cargo check --all-targets

fmt:
  cargo fmt --all

fmt-check:
  cargo fmt --all -- --check

clippy:
  cargo clippy --all-targets --all-features -- -D warnings

test:
  cargo test --all-targets --all-features

quality: fmt-check clippy
