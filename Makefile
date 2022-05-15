# The flags to pass to the `cargo build` command
BUILDFLAGS := --release

# Absolute path to project directory, required for symbolic links
# or when 'make' is run from another directory.
# - credit: https://stackoverflow.com/a/23324703/2726733
ROOT_DIR := $(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))

# default rule is to run build/test
all: build test

# builds the project
build:
	cd $(ROOT_DIR) && cargo build ${BUILDFLAGS}

# runs tests and linters
test: 
	cd $(ROOT_DIR) && cargo test --all-targets --all-features
	cd $(ROOT_DIR) && cargo fmt -- --check
	cd $(ROOT_DIR) && cargo clippy --release

demo:
	cd $(ROOT_DIR) && cargo run --package demo
