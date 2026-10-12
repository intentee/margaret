use std::collections::BTreeSet;
use std::ops::ControlFlow;

use margaret_http::response::Response;
use margaret_oauth_vocabulary::declared_scope::DeclaredScope;
use margaret_oauth_vocabulary::openid_scope::OpenidScope;
use margaret_registered_claims::audience_claim::AudienceClaim;
use margaret_token_issuance::token_issuance::TokenIssuance;

use crate::invalid_target::invalid_target;
use crate::resource_target::ResourceTarget;

pub(crate) fn token_audience(
    TokenIssuance { issuer }: TokenIssuance,
    target: ResourceTarget,
    scopes: &BTreeSet<String>,
) -> ControlFlow<Response, AudienceClaim> {
    match (target, scopes.contains(OpenidScope::NAME)) {
        (ResourceTarget::Named(resource), true) => {
            ControlFlow::Continue(AudienceClaim::Multiple(vec![
                resource.to_string(),
                issuer.to_string(),
            ]))
        }
        (ResourceTarget::Named(resource), false) => {
            ControlFlow::Continue(AudienceClaim::Single(resource.to_string()))
        }
        (ResourceTarget::Unnamed, true) => {
            ControlFlow::Continue(AudienceClaim::Single(issuer.to_string()))
        }
        (ResourceTarget::Unnamed, false) => invalid_target(
            "the client admits no resource and is not granted openid, so the token would reach no audience",
        ),
    }
}
