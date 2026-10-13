use margaret_accepted_clients_codegen::declared_accepted_clients::DeclaredAcceptedClients;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_oauth_client_codegen::declared_oauth_clients::DeclaredOAuthClients;
use margaret_oauth_vocabulary_codegen::declared_scopes::DeclaredScopes;
use margaret_sessions_codegen::declared_sessions::DeclaredSessions;
use margaret_tag_codegen::tag_pool::TagPool;
use margaret_token_issuance_codegen::declared_resource_issuances::DeclaredResourceIssuances;
use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;
use margaret_trusted_issuer_codegen::declared_trusts::DeclaredTrusts;

/// # Panics
///
/// Panics when a declaration of the indexed crate is malformed or its tags collide.
#[must_use]
pub fn collected_tags(index: &AttributeIndex) -> TagPool<'_> {
    let issuance = DeclaredTokenIssuance::read(index).expect("the token issuance is read");
    let resources =
        DeclaredResourceIssuances::read(index, &issuance).expect("the resources are read");
    let scopes = DeclaredScopes::read(index).expect("the scopes are read");

    TagPool::collect(
        index,
        &DeclaredTrusts::read(index).expect("the trusts are read"),
        &DeclaredOAuthClients::read(index, &scopes).expect("the oauth clients are read"),
        &issuance,
        &resources,
        &DeclaredAcceptedClients::read(index, &issuance, &resources, &scopes)
            .expect("the admitted clients are read"),
        &DeclaredSessions::read(index, &issuance, &resources).expect("the sessions are read"),
    )
    .expect("the tags are collected")
}
