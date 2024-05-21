.PHONY: build
build:
	cargo build --release

.PHONY: deps
deps:
	cargo update