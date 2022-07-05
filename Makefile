TAG ?= latest
PLATFORM ?= linux/amd64,linux/arm64
VERSION ?= latest

CARGO_TARGET_DIR ?= $(CURDIR)/target

lint:
	cargo fmt --all
	cargo clippy --workspace --all-targets -- -D warnings

build:
	cargo build --bin=mars --release