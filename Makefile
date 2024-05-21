.PHONY: build
build:
	cargo build --release

.PHONY: release
release: build-mac-intel build-mac-arm build-linux

.PHONY: deps
deps:
	cargo update

# TODO: this doesn't support all the different architectures yet
.PHONY: brew
brew: build
	VERSION=`grep "^version" Cargo.toml | cut -d' ' -f3 | tr -d '"'`
	cd target/release && tar -czf pick.tar.gz pick && shasum -a 256 pick.tar.gz > pick.sha
	SHA256=`cd target/release && cat pick.sha`
	sed -i '' "s/^  version .*/  version \"$${VERSION}\"/" Formula/pick.rb
	sed -i '' "s/^  sha256 .*/  sha256 \"$${SHA256}\"/" Formula/pick.rb

.PHONY: setup
setup:
	pip3 install ziglang
	cargo install cargo-zigbuild
	cargo install cross --git https://github.com/cross-rs/cross
	rustup target add x86_64-apple-darwin || echo "Already added"
	rustup target add aarch64-apple-darwin || echo "Already added"
	rustup target add x86_64-unknown-linux-musl || echo "Already added"

.PHONY: build-mac-intel
build-mac-intel:
	cargo build --target x86_64-apple-darwin --release

.PHONY: build-mac-arm
build-mac-arm:
	cargo build --target aarch64-apple-darwin --release

.PHONY: build-linux
build-linux:
	podman machine start || echo "Podman already started, ignore"
	cross build --release --target aarch64-unknown-linux-gnu
