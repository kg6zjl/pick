.PHONY: build
build:
	cargo build --release

.PHONY: deps
deps:
	cargo update

.PHONY: setup
setup:
	pip3 install ziglang
	cargo install cargo-zigbuild
	cargo install cross --git https://github.com/cross-rs/cross
	rustup target add x86_64-apple-darwin || echo "Already added"
	rustup target add aarch64-apple-darwin || echo "Already added"
	rustup target add x86_64-unknown-linux-musl || echo "Already added"
	rustup target add x86_64-unknown-linux-gnu || echo "Already added"
