use std::ops::ControlFlow;

use chrono::DateTime;
use chrono::Utc;
use oauth2::StandardTokenIntrospectionResponse;
use oauth2::TokenIntrospectionResponse;
use oauth2::basic::BasicTokenType;
use serde::de::DeserializeOwned;

use margaret_token_trust::token_trust::TokenTrust;

use crate::extension_members::ExtensionMembers;
use crate::introspected_token::IntrospectedToken;
use crate::introspection_rejection::IntrospectionRejection;
use crate::introspection_verdict::IntrospectionVerdict;

fn admitted(
    introspection: &StandardTokenIntrospectionResponse<ExtensionMembers, BasicTokenType>,
    TokenTrust { audience, issuer }: &TokenTrust,
    now: DateTime<Utc>,
) -> ControlFlow<IntrospectionRejection> {
    if !introspection.active() {
        return ControlFlow::Break(IntrospectionRejection::Inactive);
    }

    match introspection.aud() {
        Some(found) if !found.iter().any(|value| value == audience.as_str()) => {
            return ControlFlow::Break(IntrospectionRejection::AudienceMismatch {
                found: found.clone(),
            });
        }
        Some(_) => {}
        None => return ControlFlow::Break(IntrospectionRejection::AudienceMissing),
    }

    if let Some(found) = introspection.iss()
        && found != issuer.as_str()
    {
        return ControlFlow::Break(IntrospectionRejection::IssuerMismatch {
            found: found.to_string(),
        });
    }

    if let Some(exp) = introspection.exp()
        && now >= exp
    {
        return ControlFlow::Break(IntrospectionRejection::Expired { exp, now });
    }

    if let Some(nbf) = introspection.nbf()
        && now < nbf
    {
        return ControlFlow::Break(IntrospectionRejection::NotYetValid { nbf, now });
    }

    ControlFlow::Continue(())
}

fn judged<TClaims: DeserializeOwned>(
    introspection: &StandardTokenIntrospectionResponse<ExtensionMembers, BasicTokenType>,
    trust: &TokenTrust,
    now: DateTime<Utc>,
) -> ControlFlow<IntrospectionRejection, IntrospectedToken<TClaims>> {
    admitted(introspection, trust, now)?;

    match TClaims::deserialize(&introspection.extra_fields().members) {
        Ok(claims) => ControlFlow::Continue(IntrospectedToken {
            claims,
            client_id: introspection.client_id().cloned(),
            scopes: introspection.scopes().cloned(),
            subject: introspection.sub().map(str::to_string),
            username: introspection.username().map(str::to_string),
        }),
        Err(source) => ControlFlow::Break(IntrospectionRejection::UnexpectedClaims { source }),
    }
}

pub(crate) fn judge_introspection<TClaims: DeserializeOwned>(
    introspection: &StandardTokenIntrospectionResponse<ExtensionMembers, BasicTokenType>,
    trust: &TokenTrust,
    now: DateTime<Utc>,
) -> IntrospectionVerdict<TClaims> {
    match judged(introspection, trust, now) {
        ControlFlow::Continue(token) => IntrospectionVerdict::Accepted(token),
        ControlFlow::Break(rejection) => IntrospectionVerdict::Rejected(rejection),
    }
}
