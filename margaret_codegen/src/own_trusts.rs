use margaret_tag_codegen::bound_authorization_server::BoundAuthorizationServer;
use margaret_tag_codegen::oauth_client_binding::OAuthClientBinding;
use margaret_tag_codegen::tag_pool::TagPool;
use margaret_token_issuance_codegen::declared_resource_issuances::DeclaredResourceIssuances;
use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;
use margaret_token_issuance_codegen::token_issuance_declaration::TokenIssuanceDeclaration;
use margaret_trusted_issuer_codegen::own_trust::OwnTrust;

pub(crate) fn own_trusts<'declarations>(
    issuance: &'declarations DeclaredTokenIssuance,
    resources: &'declarations DeclaredResourceIssuances,
    tags: &TagPool,
    oauth_client_bindings: &[OAuthClientBinding],
) -> Vec<OwnTrust<'declarations>> {
    let DeclaredTokenIssuance::Declared(TokenIssuanceDeclaration {
        audience,
        issuer,
        tag,
        ..
    }) = issuance
    else {
        return Vec::new();
    };
    let own_clients_verify_the_issuance = oauth_client_bindings
        .iter()
        .any(|binding| matches!(binding.server, BoundAuthorizationServer::Own { .. }));
    let issuance_trust = (own_clients_verify_the_issuance || tags.verifies_bearer_tokens_for(tag))
        .then_some(OwnTrust {
            audience,
            issuer,
            tag,
        });

    issuance_trust
        .into_iter()
        .chain(
            resources
                .resources()
                .filter(|resource| tags.verifies_bearer_tokens_for(&resource.tag))
                .map(|resource| OwnTrust {
                    audience: &resource.audience,
                    issuer,
                    tag: &resource.tag,
                }),
        )
        .collect()
}
