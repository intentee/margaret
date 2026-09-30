use margaret_container::container_error::ContainerError;
use margaret_container_tests::render_with_framework_providers::render_with_framework_providers;

#[test]
fn rejects_an_accepted_client_without_the_declaration_trait() {
    assert!(matches!(
        render_with_framework_providers("accepted_client_missing_trait", &[]),
        Err(ContainerError::DeclarationMissingTrait {
            attribute: "accepts_oauth_client",
            ref required,
            ..
        }) if required == "margaret::framework::accepted_clients::declares_accepted_client::DeclaresAcceptedClient"
    ));
}

#[test]
fn rejects_a_subject_token_exchanger_without_the_exchange_trait() {
    assert!(matches!(
        render_with_framework_providers("subject_token_exchanger_missing_trait", &[]),
        Err(ContainerError::DeclarationMissingTrait {
            attribute: "exchanges_subject_tokens",
            ref required,
            ..
        }) if required == "margaret::framework::subject_token_exchange::exchanges_subject_tokens::ExchangesSubjectTokens"
    ));
}
