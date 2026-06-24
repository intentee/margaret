COVERAGE_PACKAGES := \
	-p margaret_attributes \
	-p margaret_codegen \
	-p margaret_container \
	-p margaret_container_example \
	-p margaret_everything_example \
	-p margaret_http \
	-p margaret_http_codegen \
	-p margaret_http_example

node_modules: package.json
	npm install
	touch node_modules

.PHONY: clippy
clippy:
	cargo clippy --workspace --all-targets -- -D warnings

.PHONY: coverage
coverage: node_modules
	cargo llvm-cov clean --workspace
	cargo llvm-cov nextest $(COVERAGE_PACKAGES) --no-report
	cargo llvm-cov report --json --output-path target/llvm-cov.json
	cargo llvm-cov report --lcov --output-path target/lcov.info
	cargo llvm-cov report
	npx rust-coverage-check target/llvm-cov.json \
		--workspace-root $(CURDIR) \
		--gated margaret_attributes=100 \
		--gated margaret_codegen=100 \
		--gated margaret_container=100 \
		--gated margaret_container_example=100 \
		--gated margaret_everything_example=100 \
		--gated margaret_http=100 \
		--gated margaret_http_codegen=100 \
		--gated margaret_http_example=100

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: test
test:
	cargo nextest run --workspace
