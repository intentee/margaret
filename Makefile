COVERAGE_PACKAGES := \
	-p margaret \
	-p margaret_access_token_minter \
	-p margaret_asset_bag \
	-p margaret_asset_bag_codegen \
	-p margaret_attribute_arguments \
	-p margaret_attributes \
	-p margaret_attributes_tests \
	-p margaret_codegen \
	-p margaret_codegen_tests \
	-p margaret_codegen_tokens \
	-p margaret_console \
	-p margaret_console_argument_codegen \
	-p margaret_console_codegen \
	-p margaret_construction \
	-p margaret_environment_variable \
	-p margaret_environment_variable_codegen \
	-p margaret_container \
	-p margaret_container_tests \
	-p margaret_jwks_endpoint \
	-p margaret_generated_module \
	-p margaret_http \
	-p margaret_http_codegen \
	-p margaret_http_tests \
	-p margaret_http_uploaded_file \
	-p margaret_http_validation \
	-p margaret_identity \
	-p margaret_identity_session \
	-p margaret_injection_codegen \
	-p margaret_input_weaving \
	-p margaret_item_naming_argument \
	-p margaret_jwks_client \
	-p margaret_jwks_client_tests \
	-p margaret_jwks_codegen \
	-p margaret_jwks_file_secret_storage \
	-p margaret_jwks_file_secret_storage_tests \
	-p margaret_jwks_keygen \
	-p margaret_jwks_keygen_tests \
	-p margaret_jwks_roller \
	-p margaret_jwks_roller_server \
	-p margaret_jwks_roller_tests \
	-p margaret_jwks_secret_storage_selection \
	-p margaret_jwks_secret_storage_selection_tests \
	-p margaret_jwks_secret_store \
	-p margaret_macros \
	-p margaret_middleware_codegen \
	-p margaret_model \
	-p margaret_model_codegen \
	-p margaret_peer_identity \
	-p margaret_peer_identity_tests \
	-p margaret_request_binding_codegen \
	-p margaret_route_parameter_binding \
	-p margaret_route_parameter_codegen \
	-p margaret_schema_codegen \
	-p margaret_schema_identifier_naming \
	-p margaret_schema_postgres_tests \
	-p margaret_service \
	-p margaret_serve_input_codegen \
	-p margaret_service_codegen \
	-p margaret_service_tests \
	-p margaret_spiffe_svid \
	-p margaret_spiffe_svid_bundle \
	-p margaret_spiffe_svid_bundle_tests \
	-p margaret_spiffe_svid_client \
	-p margaret_spiffe_svid_client_tests \
	-p margaret_spiffe_svid_integration_tests \
	-p margaret_spiffe_svid_server \
	-p margaret_spiffe_svid_server_tests \
	-p margaret_spiffe_svid_tests \
	-p margaret_syn_type_peeling \
	-p margaret_sync_holder \
	-p margaret_tag_codegen \
	-p margaret_token_signer \
	-p margaret_token_signer_tests \
	-p margaret_toposort \
	-p margaret_validation \
	-p margaret_views \
	-p margaret_views_codegen \
	-p margaret_websocket \
	-p margaret_websocket_codegen \
	-p margaret_websocket_session \
	-p margaret_websocket_tests
COVERAGE_EXCLUDED_PACKAGES := \
	--exclude-from-report margaret_example \
	--exclude-from-report margaret_codegen_collisions_and_diamonds_fixture \
	--exclude-from-report margaret_codegen_copy_console_arguments_fixture \
	--exclude-from-report margaret_codegen_environment_variable_inputs_fixture \
	--exclude-from-report margaret_codegen_fallible_roles_fixture \
	--exclude-from-report margaret_codegen_linear_construction_future_fixture \
	--exclude-from-report margaret_codegen_route_parameter_values_fixture \
	--exclude-from-report margaret_codegen_websocket_fixture \
	--exclude-from-report margaret_codegen_websocket_peer_identity_fixture \
	--exclude-from-report margaret_composite_foreign_key_model_fixture \
	--exclude-from-report margaret_self_referential_model_fixture \
	--exclude-from-report margaret_spiffe_http_client_fixture
GENERATED_CODE_PACKAGES := \
	-p margaret_codegen_collisions_and_diamonds_fixture \
	-p margaret_codegen_copy_console_arguments_fixture \
	-p margaret_codegen_environment_variable_inputs_fixture \
	-p margaret_codegen_fallible_roles_fixture \
	-p margaret_codegen_linear_construction_future_fixture \
	-p margaret_codegen_route_parameter_values_fixture \
	-p margaret_codegen_websocket_fixture \
	-p margaret_codegen_websocket_peer_identity_fixture \
	-p margaret_example \
	-p margaret_composite_foreign_key_model_fixture \
	-p margaret_self_referential_model_fixture \
	-p margaret_spiffe_http_client_fixture
RUNTIME_PACKAGES := \
	-p margaret \
	-p margaret_access_token_minter \
	-p margaret_asset_bag \
	-p margaret_asset_bag_codegen \
	-p margaret_attribute_arguments \
	-p margaret_attributes \
	-p margaret_codegen \
	-p margaret_codegen_tokens \
	-p margaret_console \
	-p margaret_console_argument_codegen \
	-p margaret_console_codegen \
	-p margaret_construction \
	-p margaret_environment_variable \
	-p margaret_environment_variable_codegen \
	-p margaret_container \
	-p margaret_generated_module \
	-p margaret_http \
	-p margaret_http_codegen \
	-p margaret_http_uploaded_file \
	-p margaret_http_validation \
	-p margaret_identity \
	-p margaret_identity_session \
	-p margaret_injection_codegen \
	-p margaret_input_weaving \
	-p margaret_item_naming_argument \
	-p margaret_jwks_client \
	-p margaret_jwks_codegen \
	-p margaret_jwks_endpoint \
	-p margaret_jwks_file_secret_storage \
	-p margaret_jwks_keygen \
	-p margaret_jwks_roller \
	-p margaret_jwks_roller_server \
	-p margaret_jwks_secret_storage_selection \
	-p margaret_jwks_secret_store \
	-p margaret_macros \
	-p margaret_middleware_codegen \
	-p margaret_model \
	-p margaret_model_codegen \
	-p margaret_peer_identity \
	-p margaret_request_binding_codegen \
	-p margaret_route_parameter_binding \
	-p margaret_route_parameter_codegen \
	-p margaret_schema_codegen \
	-p margaret_schema_identifier_naming \
	-p margaret_service \
	-p margaret_serve_input_codegen \
	-p margaret_service_codegen \
	-p margaret_spiffe_svid \
	-p margaret_spiffe_svid_bundle \
	-p margaret_spiffe_svid_client \
	-p margaret_spiffe_svid_server \
	-p margaret_syn_type_peeling \
	-p margaret_sync_holder \
	-p margaret_tag_codegen \
	-p margaret_token_signer \
	-p margaret_toposort \
	-p margaret_validation \
	-p margaret_views \
	-p margaret_views_codegen \
	-p margaret_websocket \
	-p margaret_websocket_codegen \
	-p margaret_websocket_session
RUNTIME_LINTS := \
	-D unsafe-code \
	-D clippy::exit \
	-D clippy::expect-used \
	-D clippy::mem-forget \
	-D clippy::panic \
	-D clippy::todo \
	-D clippy::unimplemented \
	-D clippy::unwrap-used

POSTGRES_FEATURES := --features margaret_schema_postgres_tests/tests_that_use_postgres

POSTGRES_IMAGE_NAME := postgres
POSTGRES_IMAGE_TAG := 18@sha256:3a82e1f56c8f0f5616a11103ac3d47e632c3938698946a7ad26da0df1334744a

export POSTGRES_IMAGE_NAME
export POSTGRES_IMAGE_TAG

SPIRE_FEATURES := \
	--features margaret_spiffe_svid_tests/tests_that_use_spire \
	--features margaret_spiffe_svid_integration_tests/tests_that_use_spire

node_modules: package.json
	npm install
	touch node_modules

.PHONY: clippy
clippy:
	cargo clippy --workspace --all-targets $(POSTGRES_FEATURES) $(SPIRE_FEATURES) -- -D warnings
	cargo clippy $(RUNTIME_PACKAGES) --lib -- -D warnings $(RUNTIME_LINTS)
	cargo clippy $(GENERATED_CODE_PACKAGES) --lib -- -D warnings $(RUNTIME_LINTS)
	cargo clippy -p margaret --all-targets --no-default-features -- -D warnings
	cargo clippy -p margaret --all-targets --no-default-features --features codegen -- -D warnings
	cargo clippy -p margaret --all-targets --all-features -- -D warnings
	cargo clippy -p margaret --lib --no-default-features -- -D warnings $(RUNTIME_LINTS)
	cargo clippy -p margaret --lib --no-default-features --features codegen -- -D warnings $(RUNTIME_LINTS)
	cargo clippy -p margaret --lib --all-features -- -D warnings $(RUNTIME_LINTS)

.PHONY: coverage
coverage: node_modules postgres-image
	cargo llvm-cov clean --workspace
	cargo llvm-cov nextest $(COVERAGE_EXCLUDED_PACKAGES) $(COVERAGE_PACKAGES) $(POSTGRES_FEATURES) $(SPIRE_FEATURES) --no-report
	cargo llvm-cov report --json --output-path target/llvm-cov.json
	cargo llvm-cov report --lcov --output-path target/lcov.info
	cargo llvm-cov report
	npx rust-coverage-check target/llvm-cov.json \
		--workspace-root $(CURDIR) \
		--gated margaret=100 \
		--gated margaret_access_token_minter=100 \
		--gated margaret_asset_bag=100 \
		--gated margaret_asset_bag_codegen=100 \
		--gated margaret_attribute_arguments=100 \
		--gated margaret_attributes=100 \
		--gated margaret_attributes_tests=100 \
		--gated margaret_codegen=100 \
		--gated margaret_codegen_tests=100 \
		--gated margaret_codegen_tokens=100 \
		--gated margaret_console=100 \
		--gated margaret_console_argument_codegen=100 \
		--gated margaret_console_codegen=100 \
		--gated margaret_construction=100 \
		--gated margaret_environment_variable=100 \
		--gated margaret_environment_variable_codegen=100 \
		--gated margaret_container=100 \
		--gated margaret_container_tests=100 \
		--gated margaret_jwks_endpoint=100 \
		--gated margaret_generated_module=100 \
		--gated margaret_http=100 \
		--gated margaret_http_codegen=100 \
		--gated margaret_http_tests=100 \
		--gated margaret_http_uploaded_file=100 \
		--gated margaret_http_validation=100 \
		--gated margaret_identity=100 \
		--gated margaret_identity_session=100 \
		--gated margaret_injection_codegen=100 \
		--gated margaret_input_weaving=100 \
		--gated margaret_item_naming_argument=100 \
		--gated margaret_jwks_client=100 \
		--gated margaret_jwks_client_tests=100 \
		--gated margaret_jwks_codegen=100 \
		--gated margaret_jwks_file_secret_storage=100 \
		--gated margaret_jwks_file_secret_storage_tests=100 \
		--gated margaret_jwks_keygen=100 \
		--gated margaret_jwks_keygen_tests=100 \
		--gated margaret_jwks_roller=100 \
		--gated margaret_jwks_roller_server=100 \
		--gated margaret_jwks_roller_tests=100 \
		--gated margaret_jwks_secret_storage_selection=100 \
		--gated margaret_jwks_secret_storage_selection_tests=100 \
		--gated margaret_jwks_secret_store=100 \
		--gated margaret_macros=100 \
		--gated margaret_middleware_codegen=100 \
		--gated margaret_model=100 \
		--gated margaret_model_codegen=100 \
		--gated margaret_peer_identity=100 \
		--gated margaret_peer_identity_tests=100 \
		--gated margaret_request_binding_codegen=100 \
		--gated margaret_route_parameter_binding=100 \
		--gated margaret_route_parameter_codegen=100 \
		--gated margaret_schema_codegen=100 \
		--gated margaret_schema_identifier_naming=100 \
		--gated margaret_schema_postgres_tests=100 \
		--gated margaret_service=100 \
		--gated margaret_serve_input_codegen=100 \
		--gated margaret_service_codegen=100 \
		--gated margaret_service_tests=100 \
		--gated margaret_spiffe_svid=100 \
		--gated margaret_spiffe_svid_bundle=100 \
		--gated margaret_spiffe_svid_bundle_tests=100 \
		--gated margaret_spiffe_svid_client=100 \
		--gated margaret_spiffe_svid_client_tests=100 \
		--gated margaret_spiffe_svid_integration_tests=100 \
		--gated margaret_spiffe_svid_server=100 \
		--gated margaret_spiffe_svid_server_tests=100 \
		--gated margaret_spiffe_svid_tests=100 \
		--gated margaret_syn_type_peeling=100 \
		--gated margaret_sync_holder=100 \
		--gated margaret_tag_codegen=100 \
		--gated margaret_token_signer=100 \
		--gated margaret_token_signer_tests=100 \
		--gated margaret_toposort=100 \
		--gated margaret_validation=100 \
		--gated margaret_views=100 \
		--gated margaret_views_codegen=100 \
		--gated margaret_websocket=100 \
		--gated margaret_websocket_codegen=100 \
		--gated margaret_websocket_session=100 \
		--gated margaret_websocket_tests=100

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: postgres-image
postgres-image:
	docker pull $(POSTGRES_IMAGE_NAME):$(POSTGRES_IMAGE_TAG)

.PHONY: test
test: test.integration

.PHONY: test.integration
test.integration: postgres-image
	cargo nextest run --workspace $(POSTGRES_FEATURES) $(SPIRE_FEATURES)

.PHONY: test.unit
test.unit:
	cargo nextest run --workspace
