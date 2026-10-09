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
    oauth_client_bindings: &'declarations [OAuthClientBinding],
) -> Vec<OwnTrust<'declarations>> {
    let DeclaredTokenIssuance::Declared(TokenIssuanceDeclaration { issuer, .. }) = issuance else {
        return Vec::new();
    };

    oauth_client_bindings
        .iter()
        .filter_map(|binding| match &binding.server {
            BoundAuthorizationServer::Own { admitted, .. } => Some(OwnTrust {
                audience: admitted.client_id.as_str(),
                issuer,
                tag: &binding.client.tag,
            }),
            BoundAuthorizationServer::External { .. } => None,
        })
        .chain(
            resources
                .resources()
                .filter(|resource| tags.verifies_resource_tokens_for(&resource.tag))
                .map(|resource| OwnTrust {
                    audience: resource.audience.as_str(),
                    issuer,
                    tag: &resource.tag,
                }),
        )
        .collect()
}
