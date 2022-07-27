TAG ?= latest
PLATFORM ?= linux/amd64,linux/arm64
VERSION ?= latest

CARGO_TARGET_DIR ?= $(CURDIR)/target

lint:
	cargo fmt --all
	cargo clippy --workspace --all-targets -- -D warnings

build:
	bash ./scripts/build/build-release.sh

test:
	bash ./scripts/ci/ci-run-unit-tests.sh

clean:
	cargo clean
