.PHONY: build
build:
	cargo build --release
	cp target/release/pick ~/bin/

.PHONY: deps
deps:
	cargo update

.PHONY: test
test:
	cargo test

.PHONY: setup
setup:
	# use asdf's .tool-versions and asdf-plugin-manager to setup env
	asdf plugin add asdf-plugin-manager https://github.com/asdf-community/asdf-plugin-manager.git
	asdf plugin update asdf-plugin-manager v1.3.1
	asdf install asdf-plugin-manager 1.3.1
	asdf global asdf-plugin-manager 1.3.1

	asdf install
	cargo update