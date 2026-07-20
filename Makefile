COVERAGE_PACKAGES := \
	-p margaret_attributes \
	-p margaret_codegen \
	-p margaret_codegen_tokens \
	-p margaret_console \
	-p margaret_console_argument_codegen \
	-p margaret_console_codegen \
	-p margaret_container \
	-p margaret_container_tests \
	-p margaret_generated_module \
	-p margaret_http \
	-p margaret_http_codegen \
	-p margaret_http_tests \
	-p margaret_http_validation \
	-p margaret_identity_session \
	-p margaret_injection_codegen \
	-p margaret_jwks_client \
	-p margaret_jwks_client_tests \
	-p margaret_jwks_key_gen \
	-p margaret_jwks_key_gen_tests \
	-p margaret_jwks_roller \
	-p margaret_jwks_roller_server \
	-p margaret_jwks_roller_tests \
	-p margaret_macros \
	-p margaret_model \
	-p margaret_model_codegen \
	-p margaret_peer_identity \
	-p margaret_peer_identity_tests \
	-p margaret_service \
	-p margaret_service_codegen \
	-p margaret_service_tests \
	-p margaret_spiffe_svid \
	-p margaret_spiffe_svid_client \
	-p margaret_spiffe_svid_client_tests \
	-p margaret_spiffe_svid_integration_tests \
	-p margaret_spiffe_svid_server \
	-p margaret_spiffe_svid_server_tests \
	-p margaret_spiffe_svid_tests \
	-p margaret_sync_holder \
	-p margaret_token_signer \
	-p margaret_token_signer_tests \
	-p margaret_validation \
	-p margaret_views \
	-p margaret_views_codegen

SPIRE_FEATURES := \
	--features margaret_spiffe_svid_tests/tests_that_use_spire \
	--features margaret_spiffe_svid_integration_tests/tests_that_use_spire

node_modules: package.json
	npm install
	touch node_modules

.PHONY: clippy
clippy:
	cargo clippy --workspace --all-targets $(SPIRE_FEATURES) -- -D warnings

.PHONY: coverage
coverage: node_modules
	cargo llvm-cov clean --workspace
	cargo llvm-cov nextest $(COVERAGE_PACKAGES) $(SPIRE_FEATURES) --no-report
	cargo llvm-cov report --json --output-path target/llvm-cov.json
	cargo llvm-cov report --lcov --output-path target/lcov.info
	cargo llvm-cov report
	npx rust-coverage-check target/llvm-cov.json \
		--workspace-root $(CURDIR) \
		--gated margaret_attributes=100 \
		--gated margaret_codegen=100 \
		--gated margaret_codegen_tokens=100 \
		--gated margaret_console=100 \
		--gated margaret_console_argument_codegen=100 \
		--gated margaret_console_codegen=100 \
		--gated margaret_container=100 \
		--gated margaret_container_tests=100 \
		--gated margaret_generated_module=100 \
		--gated margaret_http=100 \
		--gated margaret_http_codegen=100 \
		--gated margaret_http_tests=100 \
		--gated margaret_http_validation=100 \
		--gated margaret_identity_session=100 \
		--gated margaret_injection_codegen=100 \
		--gated margaret_jwks_client=100 \
		--gated margaret_jwks_client_tests=100 \
		--gated margaret_jwks_key_gen=100 \
		--gated margaret_jwks_key_gen_tests=100 \
		--gated margaret_jwks_roller=100 \
		--gated margaret_jwks_roller_server=100 \
		--gated margaret_jwks_roller_tests=100 \
		--gated margaret_macros=100 \
		--gated margaret_model=100 \
		--gated margaret_model_codegen=100 \
		--gated margaret_peer_identity=100 \
		--gated margaret_peer_identity_tests=100 \
		--gated margaret_service=100 \
		--gated margaret_service_codegen=100 \
		--gated margaret_service_tests=100 \
		--gated margaret_spiffe_svid=100 \
		--gated margaret_spiffe_svid_client=100 \
		--gated margaret_spiffe_svid_client_tests=100 \
		--gated margaret_spiffe_svid_integration_tests=100 \
		--gated margaret_spiffe_svid_server=100 \
		--gated margaret_spiffe_svid_server_tests=100 \
		--gated margaret_spiffe_svid_tests=100 \
		--gated margaret_sync_holder=100 \
		--gated margaret_token_signer=100 \
		--gated margaret_token_signer_tests=100 \
		--gated margaret_validation=100 \
		--gated margaret_views=100 \
		--gated margaret_views_codegen=100

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: test.unit
test.unit:
	cargo nextest run --workspace

.PHONY: test.integration
test.integration:
	cargo nextest run --workspace $(SPIRE_FEATURES)

.PHONY: test
test: test.integration
