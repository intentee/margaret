COVERAGE_PACKAGES := \
	-p margaret \
	-p margaret_accepted_clients \
	-p margaret_accepted_clients_codegen \
	-p margaret_accepted_clients_tests \
	-p margaret_access_token_minter \
	-p margaret_asset_bag \
	-p margaret_asset_bag_codegen \
	-p margaret_attribute_arguments \
	-p margaret_attributes \
	-p margaret_attributes_tests \
	-p margaret_authorization_grants \
	-p margaret_authorization_grants_database \
	-p margaret_authorization_grants_schema \
	-p margaret_authorization_server_client \
	-p margaret_authorization_server_client_tests \
	-p margaret_bearer_token_verification \
	-p margaret_bearer_token_verification_tests \
	-p margaret_client_assertions_database \
	-p margaret_client_assertions_schema \
	-p margaret_client_credentials \
	-p margaret_client_credentials_tests \
	-p margaret_codegen \
	-p margaret_codegen_tests \
	-p margaret_codegen_tokens \
	-p margaret_console \
	-p margaret_console_argument_codegen \
	-p margaret_console_codegen \
	-p margaret_construction \
	-p margaret_container \
	-p margaret_container_tests \
	-p margaret_database \
	-p margaret_database_codegen \
	-p margaret_database_tests \
	-p margaret_deadline \
	-p margaret_declaration_anchor \
	-p margaret_environment_variable \
	-p margaret_environment_variable_codegen \
	-p margaret_generated_module \
	-p margaret_http \
	-p margaret_http_codegen \
	-p margaret_http_tests \
	-p margaret_http_uploaded_file \
	-p margaret_http_validation \
	-p margaret_https_url \
	-p margaret_identity \
	-p margaret_identity_session \
	-p margaret_injection_codegen \
	-p margaret_input_weaving \
	-p margaret_issuer_directory \
	-p margaret_issuer_directory_tests \
	-p margaret_issuer_key_set \
	-p margaret_issuer_key_set_tests \
	-p margaret_issuer_metadata \
	-p margaret_issuer_request \
	-p margaret_item_naming_argument \
	-p margaret_jose_parameters \
	-p margaret_jwks_codegen \
	-p margaret_jwks_keygen \
	-p margaret_jwks_keygen_tests \
	-p margaret_jwks_roller \
	-p margaret_jwks_roller_server \
	-p margaret_jwks_roller_tests \
	-p margaret_jwks_secret_store \
	-p margaret_jwks_secret_store_tests \
	-p margaret_jws_verification \
	-p margaret_jws_verification_tests \
	-p margaret_jwt_verification \
	-p margaret_jwt_verification_tests \
	-p margaret_macros \
	-p margaret_middleware_codegen \
	-p margaret_model \
	-p margaret_model_codegen \
	-p margaret_oauth_client \
	-p margaret_oauth_client_codegen \
	-p margaret_oauth_vocabulary \
	-p margaret_oauth_vocabulary_codegen \
	-p margaret_oidc_discovery \
	-p margaret_oidc_provider \
	-p margaret_oidc_provider_codegen \
	-p margaret_oidc_provider_tests \
	-p margaret_oidc_sign_in \
	-p margaret_oidc_sign_in_tests \
	-p margaret_openid_conformance_tests \
	-p margaret_peer_identity \
	-p margaret_peer_identity_tests \
	-p margaret_registered_claims \
	-p margaret_request_binding_codegen \
	-p margaret_route_method \
	-p margaret_route_parameter_binding \
	-p margaret_route_parameter_codegen \
	-p margaret_schema_codegen \
	-p margaret_schema_identifier_naming \
	-p margaret_schema_postgres_tests \
	-p margaret_serve_input_codegen \
	-p margaret_service \
	-p margaret_service_codegen \
	-p margaret_service_tests \
	-p margaret_signing_keys_database \
	-p margaret_signing_keys_schema \
	-p margaret_spiffe_svid \
	-p margaret_spiffe_svid_bundle \
	-p margaret_spiffe_svid_bundle_tests \
	-p margaret_spiffe_svid_client \
	-p margaret_spiffe_svid_client_tests \
	-p margaret_spiffe_svid_integration_tests \
	-p margaret_spiffe_svid_server \
	-p margaret_spiffe_svid_server_tests \
	-p margaret_spiffe_svid_tests \
	-p margaret_store_contract_tests \
	-p margaret_subject_token_exchange \
	-p margaret_subject_token_exchange_tests \
	-p margaret_syn_type_peeling \
	-p margaret_sync_holder \
	-p margaret_tag_codegen \
	-p margaret_tag_codegen_tests \
	-p margaret_token_digest \
	-p margaret_token_exchange_client \
	-p margaret_token_exchange_client_tests \
	-p margaret_token_introspection \
	-p margaret_token_introspection_tests \
	-p margaret_token_issuance \
	-p margaret_token_issuance_codegen \
	-p margaret_token_signer \
	-p margaret_token_signer_tests \
	-p margaret_token_trust \
	-p margaret_toposort \
	-p margaret_trusted_issuer \
	-p margaret_trusted_issuer_codegen \
	-p margaret_umbrella_path \
	-p margaret_validation \
	-p margaret_views \
	-p margaret_views_codegen \
	-p margaret_websocket \
	-p margaret_websocket_codegen \
	-p margaret_websocket_session \
	-p margaret_websocket_tests
COVERAGE_EXCLUDED_PACKAGES := \
	--exclude-from-report margaret_cluster_fixture \
	--exclude-from-report margaret_codegen_collisions_and_diamonds_fixture \
	--exclude-from-report margaret_codegen_copy_console_arguments_fixture \
	--exclude-from-report margaret_codegen_environment_variable_inputs_fixture \
	--exclude-from-report margaret_codegen_fallible_roles_fixture \
	--exclude-from-report margaret_codegen_jwks_token_trust_fixture \
	--exclude-from-report margaret_codegen_linear_construction_future_fixture \
	--exclude-from-report margaret_codegen_many_serve_inputs_fixture \
	--exclude-from-report margaret_codegen_oauth_clients_fixture \
	--exclude-from-report margaret_codegen_oidc_bearer_identity_fixture \
	--exclude-from-report margaret_codegen_oidc_issuers_fixture \
	--exclude-from-report margaret_codegen_oidc_provider_fixture \
	--exclude-from-report margaret_codegen_route_parameter_values_fixture \
	--exclude-from-report margaret_codegen_websocket_fixture \
	--exclude-from-report margaret_codegen_websocket_peer_identity_fixture \
	--exclude-from-report margaret_composite_foreign_key_model_fixture \
	--exclude-from-report margaret_example \
	--exclude-from-report margaret_schema_postgres_fixture \
	--exclude-from-report margaret_self_referential_model_fixture \
	--exclude-from-report margaret_spiffe_http_client_fixture
GENERATED_CODE_PACKAGES := \
	-p margaret_cluster_fixture \
	-p margaret_codegen_collisions_and_diamonds_fixture \
	-p margaret_codegen_copy_console_arguments_fixture \
	-p margaret_codegen_environment_variable_inputs_fixture \
	-p margaret_codegen_fallible_roles_fixture \
	-p margaret_codegen_jwks_token_trust_fixture \
	-p margaret_codegen_linear_construction_future_fixture \
	-p margaret_codegen_many_serve_inputs_fixture \
	-p margaret_codegen_oauth_clients_fixture \
	-p margaret_codegen_oidc_bearer_identity_fixture \
	-p margaret_codegen_oidc_issuers_fixture \
	-p margaret_codegen_oidc_provider_fixture \
	-p margaret_codegen_route_parameter_values_fixture \
	-p margaret_codegen_websocket_fixture \
	-p margaret_codegen_websocket_peer_identity_fixture \
	-p margaret_composite_foreign_key_model_fixture \
	-p margaret_example \
	-p margaret_schema_postgres_fixture \
	-p margaret_self_referential_model_fixture \
	-p margaret_spiffe_http_client_fixture
RUNTIME_PACKAGES := \
	-p margaret \
	-p margaret_accepted_clients \
	-p margaret_access_token_minter \
	-p margaret_asset_bag \
	-p margaret_asset_bag_codegen \
	-p margaret_attribute_arguments \
	-p margaret_attributes \
	-p margaret_authorization_grants \
	-p margaret_authorization_grants_database \
	-p margaret_authorization_grants_schema \
	-p margaret_authorization_server_client \
	-p margaret_bearer_token_verification \
	-p margaret_client_assertions_database \
	-p margaret_client_assertions_schema \
	-p margaret_client_credentials \
	-p margaret_codegen \
	-p margaret_codegen_tokens \
	-p margaret_console \
	-p margaret_console_argument_codegen \
	-p margaret_console_codegen \
	-p margaret_construction \
	-p margaret_container \
	-p margaret_database \
	-p margaret_database_codegen \
	-p margaret_deadline \
	-p margaret_declaration_anchor \
	-p margaret_environment_variable \
	-p margaret_environment_variable_codegen \
	-p margaret_generated_module \
	-p margaret_http \
	-p margaret_http_codegen \
	-p margaret_http_uploaded_file \
	-p margaret_http_validation \
	-p margaret_https_url \
	-p margaret_identity \
	-p margaret_identity_session \
	-p margaret_injection_codegen \
	-p margaret_input_weaving \
	-p margaret_issuer_directory \
	-p margaret_issuer_key_set \
	-p margaret_issuer_metadata \
	-p margaret_issuer_request \
	-p margaret_item_naming_argument \
	-p margaret_jose_parameters \
	-p margaret_jwks_codegen \
	-p margaret_jwks_keygen \
	-p margaret_jwks_roller \
	-p margaret_jwks_roller_server \
	-p margaret_jwks_secret_store \
	-p margaret_jws_verification \
	-p margaret_jwt_verification \
	-p margaret_macros \
	-p margaret_middleware_codegen \
	-p margaret_model \
	-p margaret_model_codegen \
	-p margaret_oauth_client \
	-p margaret_oauth_client_codegen \
	-p margaret_oauth_vocabulary \
	-p margaret_oauth_vocabulary_codegen \
	-p margaret_oidc_discovery \
	-p margaret_oidc_provider \
	-p margaret_oidc_provider_codegen \
	-p margaret_oidc_sign_in \
	-p margaret_peer_identity \
	-p margaret_registered_claims \
	-p margaret_request_binding_codegen \
	-p margaret_route_method \
	-p margaret_route_parameter_binding \
	-p margaret_route_parameter_codegen \
	-p margaret_schema_codegen \
	-p margaret_schema_identifier_naming \
	-p margaret_serve_input_codegen \
	-p margaret_service \
	-p margaret_service_codegen \
	-p margaret_signing_keys_database \
	-p margaret_signing_keys_schema \
	-p margaret_spiffe_svid \
	-p margaret_spiffe_svid_bundle \
	-p margaret_spiffe_svid_client \
	-p margaret_spiffe_svid_server \
	-p margaret_subject_token_exchange \
	-p margaret_syn_type_peeling \
	-p margaret_sync_holder \
	-p margaret_tag_codegen \
	-p margaret_token_digest \
	-p margaret_token_exchange_client \
	-p margaret_token_introspection \
	-p margaret_token_issuance \
	-p margaret_token_issuance_codegen \
	-p margaret_token_signer \
	-p margaret_token_trust \
	-p margaret_toposort \
	-p margaret_trusted_issuer \
	-p margaret_trusted_issuer_codegen \
	-p margaret_umbrella_path \
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

POSTGRES_FEATURES := \
	--features margaret_schema_postgres_tests/tests_that_use_postgres \
	--features margaret_store_contract_tests/tests_that_use_postgres

POSTGRES_TESTS := package(margaret_schema_postgres_tests) | binary_id(margaret_store_contract_tests::database)

POSTGRES_IMAGE_NAME := postgres
POSTGRES_IMAGE_TAG := 18@sha256:3a82e1f56c8f0f5616a11103ac3d47e632c3938698946a7ad26da0df1334744a

export POSTGRES_IMAGE_NAME
export POSTGRES_IMAGE_TAG

EXAMPLE_POSTGRES_FEATURES := --features margaret_example_tests/tests_that_use_postgres

EXAMPLE_COMPOSE := docker compose --file margaret_example/compose.yml

EXAMPLE_DATABASE_URL = postgresql://margaret_example:margaret_example@$$($(EXAMPLE_COMPOSE) port postgres 5432)/margaret_example

SPIRE_FEATURES := \
	--features margaret_spiffe_svid_tests/tests_that_use_spire \
	--features margaret_spiffe_svid_integration_tests/tests_that_use_spire

OPENID_CONFORMANCE_FEATURES := \
	--features margaret_openid_conformance_tests/tests_that_use_openid_conformance_suite

CONFORMANCE_SUITE_MONGODB_IMAGE := mongo:6.0.13@sha256:b415b12f638e2685d06c58ab7fb5943577c50fadec6d9340ef67d21aeac72070
CONFORMANCE_SUITE_NGINX_IMAGE := registry.gitlab.com/openid/conformance-suite/nginx:release-v5.3.1@sha256:6ea3f4b8854f1f3626c81350900962d9e5f86424791d8e72b378a26ee2f4c105
CONFORMANCE_SUITE_SERVER_IMAGE := registry.gitlab.com/openid/conformance-suite:release-v5.3.1@sha256:69495f453a920c262f66e5e72abd12501c33e05ce88051cddf300c00621a4d70

export CONFORMANCE_SUITE_MONGODB_IMAGE
export CONFORMANCE_SUITE_NGINX_IMAGE
export CONFORMANCE_SUITE_SERVER_IMAGE

node_modules: package.json
	npm install
	touch node_modules

.PHONY: clippy
clippy:
	cargo clippy --workspace --all-targets $(POSTGRES_FEATURES) $(EXAMPLE_POSTGRES_FEATURES) $(SPIRE_FEATURES) $(OPENID_CONFORMANCE_FEATURES) -- -D warnings
	cargo clippy $(RUNTIME_PACKAGES) --lib -- -D warnings $(RUNTIME_LINTS)
	cargo clippy $(GENERATED_CODE_PACKAGES) --lib -- -D warnings $(RUNTIME_LINTS)
	cargo clippy -p margaret --all-targets --no-default-features -- -D warnings
	cargo clippy -p margaret --all-targets --no-default-features --features codegen -- -D warnings
	cargo clippy -p margaret --all-targets --all-features -- -D warnings
	cargo clippy -p margaret --lib --no-default-features -- -D warnings $(RUNTIME_LINTS)
	cargo clippy -p margaret --lib --no-default-features --features codegen -- -D warnings $(RUNTIME_LINTS)
	cargo clippy -p margaret --lib --all-features -- -D warnings $(RUNTIME_LINTS)

.PHONY: coverage
coverage: node_modules openid-conformance-images postgres-image
	cargo llvm-cov clean --workspace
	cargo llvm-cov nextest $(COVERAGE_EXCLUDED_PACKAGES) $(COVERAGE_PACKAGES) $(POSTGRES_FEATURES) $(SPIRE_FEATURES) $(OPENID_CONFORMANCE_FEATURES) --no-report --filterset 'none()' --no-tests pass
	docker run --rm --user postgres $(POSTGRES_IMAGE_NAME):$(POSTGRES_IMAGE_TAG) initdb --auth trust --no-sync --pgdata /tmp/warm
	cargo llvm-cov nextest $(COVERAGE_EXCLUDED_PACKAGES) $(COVERAGE_PACKAGES) $(POSTGRES_FEATURES) $(SPIRE_FEATURES) $(OPENID_CONFORMANCE_FEATURES) --no-report --filterset '$(POSTGRES_TESTS)'
	cargo llvm-cov nextest $(COVERAGE_EXCLUDED_PACKAGES) $(COVERAGE_PACKAGES) $(POSTGRES_FEATURES) $(SPIRE_FEATURES) $(OPENID_CONFORMANCE_FEATURES) --no-report --filterset 'not ($(POSTGRES_TESTS))'
	cargo llvm-cov report --json --output-path target/llvm-cov.json
	cargo llvm-cov report --lcov --output-path target/lcov.info
	cargo llvm-cov report
	npx rust-coverage-check target/llvm-cov.json \
		--workspace-root $(CURDIR) \
		--gated margaret=100 \
		--gated margaret_accepted_clients=100 \
		--gated margaret_accepted_clients_codegen=100 \
		--gated margaret_accepted_clients_tests=100 \
		--gated margaret_access_token_minter=100 \
		--gated margaret_asset_bag=100 \
		--gated margaret_asset_bag_codegen=100 \
		--gated margaret_attribute_arguments=100 \
		--gated margaret_attributes=100 \
		--gated margaret_attributes_tests=100 \
		--gated margaret_authorization_grants=100 \
		--gated margaret_authorization_grants_database=100 \
		--gated margaret_authorization_grants_schema=100 \
		--gated margaret_authorization_server_client=100 \
		--gated margaret_authorization_server_client_tests=100 \
		--gated margaret_bearer_token_verification=100 \
		--gated margaret_bearer_token_verification_tests=100 \
		--gated margaret_client_assertions_database=100 \
		--gated margaret_client_assertions_schema=100 \
		--gated margaret_client_credentials=100 \
		--gated margaret_client_credentials_tests=100 \
		--gated margaret_codegen=100 \
		--gated margaret_codegen_tests=100 \
		--gated margaret_codegen_tokens=100 \
		--gated margaret_console=100 \
		--gated margaret_console_argument_codegen=100 \
		--gated margaret_console_codegen=100 \
		--gated margaret_construction=100 \
		--gated margaret_container=100 \
		--gated margaret_container_tests=100 \
		--gated margaret_database=100 \
		--gated margaret_database_codegen=100 \
		--gated margaret_database_tests=100 \
		--gated margaret_deadline=100 \
		--gated margaret_declaration_anchor=100 \
		--gated margaret_environment_variable=100 \
		--gated margaret_environment_variable_codegen=100 \
		--gated margaret_generated_module=100 \
		--gated margaret_http=100 \
		--gated margaret_http_codegen=100 \
		--gated margaret_http_tests=100 \
		--gated margaret_http_uploaded_file=100 \
		--gated margaret_http_validation=100 \
		--gated margaret_https_url=100 \
		--gated margaret_identity=100 \
		--gated margaret_identity_session=100 \
		--gated margaret_injection_codegen=100 \
		--gated margaret_input_weaving=100 \
		--gated margaret_issuer_directory=100 \
		--gated margaret_issuer_directory_tests=100 \
		--gated margaret_issuer_key_set=100 \
		--gated margaret_issuer_key_set_tests=100 \
		--gated margaret_issuer_metadata=100 \
		--gated margaret_issuer_request=100 \
		--gated margaret_item_naming_argument=100 \
		--gated margaret_jose_parameters=100 \
		--gated margaret_jwks_codegen=100 \
		--gated margaret_jwks_keygen=100 \
		--gated margaret_jwks_keygen_tests=100 \
		--gated margaret_jwks_roller=100 \
		--gated margaret_jwks_roller_server=100 \
		--gated margaret_jwks_roller_tests=100 \
		--gated margaret_jwks_secret_store=100 \
		--gated margaret_jwks_secret_store_tests=100 \
		--gated margaret_jws_verification=100 \
		--gated margaret_jws_verification_tests=100 \
		--gated margaret_jwt_verification=100 \
		--gated margaret_jwt_verification_tests=100 \
		--gated margaret_macros=100 \
		--gated margaret_middleware_codegen=100 \
		--gated margaret_model=100 \
		--gated margaret_model_codegen=100 \
		--gated margaret_oauth_client=100 \
		--gated margaret_oauth_client_codegen=100 \
		--gated margaret_oauth_vocabulary=100 \
		--gated margaret_oauth_vocabulary_codegen=100 \
		--gated margaret_oidc_discovery=100 \
		--gated margaret_oidc_provider=100 \
		--gated margaret_oidc_provider_codegen=100 \
		--gated margaret_oidc_provider_tests=100 \
		--gated margaret_oidc_sign_in=100 \
		--gated margaret_oidc_sign_in_tests=100 \
		--gated margaret_openid_conformance_tests=100 \
		--gated margaret_peer_identity=100 \
		--gated margaret_peer_identity_tests=100 \
		--gated margaret_registered_claims=100 \
		--gated margaret_request_binding_codegen=100 \
		--gated margaret_route_method=100 \
		--gated margaret_route_parameter_binding=100 \
		--gated margaret_route_parameter_codegen=100 \
		--gated margaret_schema_codegen=100 \
		--gated margaret_schema_identifier_naming=100 \
		--gated margaret_schema_postgres_tests=100 \
		--gated margaret_serve_input_codegen=100 \
		--gated margaret_service=100 \
		--gated margaret_service_codegen=100 \
		--gated margaret_service_tests=100 \
		--gated margaret_signing_keys_database=100 \
		--gated margaret_signing_keys_schema=100 \
		--gated margaret_spiffe_svid=100 \
		--gated margaret_spiffe_svid_bundle=100 \
		--gated margaret_spiffe_svid_bundle_tests=100 \
		--gated margaret_spiffe_svid_client=100 \
		--gated margaret_spiffe_svid_client_tests=100 \
		--gated margaret_spiffe_svid_integration_tests=100 \
		--gated margaret_spiffe_svid_server=100 \
		--gated margaret_spiffe_svid_server_tests=100 \
		--gated margaret_spiffe_svid_tests=100 \
		--gated margaret_store_contract_tests=100 \
		--gated margaret_subject_token_exchange=100 \
		--gated margaret_subject_token_exchange_tests=100 \
		--gated margaret_syn_type_peeling=100 \
		--gated margaret_sync_holder=100 \
		--gated margaret_tag_codegen=100 \
		--gated margaret_tag_codegen_tests=100 \
		--gated margaret_token_digest=100 \
		--gated margaret_token_exchange_client=100 \
		--gated margaret_token_exchange_client_tests=100 \
		--gated margaret_token_introspection=100 \
		--gated margaret_token_introspection_tests=100 \
		--gated margaret_token_issuance=100 \
		--gated margaret_token_issuance_codegen=100 \
		--gated margaret_token_signer=100 \
		--gated margaret_token_signer_tests=100 \
		--gated margaret_token_trust=100 \
		--gated margaret_toposort=100 \
		--gated margaret_trusted_issuer=100 \
		--gated margaret_trusted_issuer_codegen=100 \
		--gated margaret_umbrella_path=100 \
		--gated margaret_validation=100 \
		--gated margaret_views=100 \
		--gated margaret_views_codegen=100 \
		--gated margaret_websocket=100 \
		--gated margaret_websocket_codegen=100 \
		--gated margaret_websocket_session=100 \
		--gated margaret_websocket_tests=100

.PHONY: example.database
example.database:
	$(EXAMPLE_COMPOSE) up --detach --force-recreate --renew-anon-volumes --wait

.PHONY: example.migrate
example.migrate:
	mkdir -p target/margaret_example
	cargo run --quiet -p margaret_example -- schema > target/margaret_example/schema.sql
	$(EXAMPLE_COMPOSE) exec --no-TTY postgres psql --dbname margaret_example --quiet --set ON_ERROR_STOP=1 --username margaret_example < target/margaret_example/schema.sql

.PHONY: example.seed
example.seed:
	MARGARET_EXAMPLE_DATABASE_URL=$(EXAMPLE_DATABASE_URL) cargo run --quiet -p margaret_example -- seed

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: openid-conformance-images
openid-conformance-images:
	docker pull $(CONFORMANCE_SUITE_MONGODB_IMAGE)
	docker pull $(CONFORMANCE_SUITE_NGINX_IMAGE)
	docker pull $(CONFORMANCE_SUITE_SERVER_IMAGE)

.PHONY: postgres-image
postgres-image:
	docker pull $(POSTGRES_IMAGE_NAME):$(POSTGRES_IMAGE_TAG)

.PHONY: test
test: test.integration

.PHONY: test.conformance
test.conformance: openid-conformance-images
	cargo nextest run -p margaret_openid_conformance_tests $(OPENID_CONFORMANCE_FEATURES)

.PHONY: test.integration
test.integration: openid-conformance-images postgres-image
	cargo nextest run --workspace $(POSTGRES_FEATURES) $(EXAMPLE_POSTGRES_FEATURES) $(SPIRE_FEATURES) $(OPENID_CONFORMANCE_FEATURES)

.PHONY: test.unit
test.unit:
	cargo nextest run --workspace
