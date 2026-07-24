use margaret_codegen::codegen_error::CodegenError;
use margaret_codegen_tests::generate_fixture::generate_fixture;
use margaret_codegen_tests::generated_module_source::generated_module_source;

#[test]
fn generates_the_jwks_poll_service_for_an_injected_verifier() {
    let generated = generate_fixture("jwks_client").expect("the fixture generates");
    let source: String = generated_module_source(&generated, "serve")
        .expect("the serve module is generated")
        .split_whitespace()
        .collect();

    assert!(
        source.contains(
            "margaret_jwks_client::public_jwks_poll_service::PublicJwksPollService::new("
        )
    );
    assert!(source.contains("margaret_jwks_client::default_http_client::default_http_client()"));
}

#[test]
fn rejects_a_verifier_with_an_ambiguous_jwks_endpoint() {
    let error = generate_fixture("jwks_client_ambiguous_endpoint")
        .expect_err("more than one jwks endpoint is ambiguous");

    let CodegenError::Services { source } = error else {
        panic!("the ambiguity surfaces as a service codegen error");
    };

    assert!(
        source
            .to_string()
            .contains("the jwks endpoint to poll is ambiguous")
    );
}
