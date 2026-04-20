set shell := ["bash", "-euo", "pipefail", "-c"]

default:
  @just --list --unsorted

build:
  cargo build --workspace

build-server:
  cargo build --package object-server

build-frontend:
  cd frontend && npm install && npm run build

run *args:
  cargo run --package object-server -- {{args}}

run-server:
  cargo run --package object-server

run-frontend:
  cd frontend && npm install && npm run dev

openapi-gen:
  cd frontend && npm run gen:types

check:
  cargo check --all-targets

fmt:
  cargo fmt --all

fmt-check:
  cargo fmt --all -- --check

clippy:
  cargo clippy --all-targets --all-features -- -D warnings

test:
  cargo nextest run --workspace --all-features --all-targets --no-tests pass

quality: fmt-check clippy

dev:
  (cd frontend && npm install && npm run dev) & cargo run --package object-server
