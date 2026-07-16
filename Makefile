COVERAGE_PACKAGES := \
	-p margaret_attributes \
	-p margaret_codegen \
	-p margaret_console \
	-p margaret_console_argument_codegen \
	-p margaret_console_codegen \
	-p margaret_container \
	-p margaret_generated_module \
	-p margaret_http \
	-p margaret_http_codegen \
	-p margaret_http_tests \
	-p margaret_http_validation \
	-p margaret_identity \
	-p margaret_identity_session \
	-p margaret_identity_tests \
	-p margaret_injection_codegen \
	-p margaret_jwks_key_gen \
	-p margaret_jwks_key_gen_tests \
	-p margaret_jwks_roller \
	-p margaret_macros \
	-p margaret_peer_identity \
	-p margaret_peer_identity_tests \
	-p margaret_service \
	-p margaret_service_codegen \
	-p margaret_service_tests \
	-p margaret_spiffe_svid_manager \
	-p margaret_spiffe_svid_manager_tests \
	-p margaret_sync_holder \
	-p margaret_token_signer \
	-p margaret_token_signer_tests \
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
		--gated margaret_console_argument_codegen=100 \
		--gated margaret_console_codegen=100 \
		--gated margaret_container=100 \
		--gated margaret_generated_module=100 \
		--gated margaret_http=100 \
		--gated margaret_http_codegen=100 \
		--gated margaret_http_tests=100 \
		--gated margaret_http_validation=100 \
		--gated margaret_identity=100 \
		--gated margaret_identity_session=100 \
		--gated margaret_identity_tests=100 \
		--gated margaret_injection_codegen=100 \
		--gated margaret_jwks_key_gen=100 \
		--gated margaret_jwks_key_gen_tests=100 \
		--gated margaret_jwks_roller=100 \
		--gated margaret_macros=100 \
		--gated margaret_peer_identity=100 \
		--gated margaret_peer_identity_tests=100 \
		--gated margaret_service=100 \
		--gated margaret_service_codegen=100 \
		--gated margaret_service_tests=100 \
		--gated margaret_spiffe_svid_manager=100 \
		--gated margaret_spiffe_svid_manager_tests=100 \
		--gated margaret_sync_holder=100 \
		--gated margaret_token_signer=100 \
		--gated margaret_token_signer_tests=100 \
		--gated margaret_validation=100

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: test
test:
	cargo nextest run --workspace
