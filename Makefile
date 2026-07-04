COVERAGE_PACKAGES := \
	-p margaret_attributes \
	-p margaret_codegen \
	-p margaret_console \
	-p margaret_console_codegen \
	-p margaret_container \
	-p margaret_generated_module \
	-p margaret_http \
	-p margaret_http_codegen \
	-p margaret_http_validation \
	-p margaret_injection_codegen \
	-p margaret_macros \
	-p margaret_service_codegen \
	-p margaret_service_tests \
	-p margaret_validation

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
		--gated margaret_console=100 \
		--gated margaret_console_codegen=100 \
		--gated margaret_container=100 \
		--gated margaret_generated_module=100 \
		--gated margaret_http=100 \
		--gated margaret_http_codegen=100 \
		--gated margaret_http_validation=100 \
		--gated margaret_injection_codegen=100 \
		--gated margaret_macros=100 \
		--gated margaret_service=100 \
		--gated margaret_service_codegen=100 \
		--gated margaret_service_tests=100 \
		--gated margaret_validation=100

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: test
test:
	cargo nextest run --workspace

.PHONY: bench-http
bench-http:
	cargo build --release -p margaret_example
	@addr=127.0.0.1:8079; \
	target/release/margaret_example serve --public-addr $$addr --internal-addr 127.0.0.1:8078 & \
	server=$$!; \
	until curl -sf "http://$$addr/health" >/dev/null 2>&1; do :; done; \
	wrk -t4 -c64 -d10s "http://$$addr/health"; \
	kill $$server
