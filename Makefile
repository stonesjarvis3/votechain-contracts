.PHONY: build test fmt fmt-check lint clean openapi-validate docs-serve
build:
	stellar contract build
test:
	cargo test
fmt:
	cargo fmt
fmt-check:
	cargo fmt --check
lint:
	cargo clippy --all-targets -- -D warnings
clean:
	cargo clean
openapi-validate:
	npx @redocly/cli@1.21.0 lint api/openapi.yml
docs-serve:
	python3 -m http.server 8080 --directory docs/api
