use chrono::Utc;
use log::error;
use log::warn;
use serde::de::DeserializeOwned;

use margaret_authorization_server_client::authorization_server_client::AuthorizationServerClient;
use margaret_authorization_server_client::authorization_server_client_error::AuthorizationServerClientError;
use margaret_authorization_server_client::endpoint_outcome::EndpointOutcome;
use margaret_http::bearer_challenge::BearerChallenge;
use margaret_http::request_authorization::RequestAuthorization;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;

use crate::extension_members::ExtensionMembers;
use crate::introspection_admission::IntrospectionAdmission;
use crate::introspection_verdict::IntrospectionVerdict;
use crate::judge_introspection::judge_introspection;

fn refused<TClaims>(response: Response) -> IntrospectionAdmission<TClaims> {
    IntrospectionAdmission::Refused(ResponseContinuation::from(response))
}

fn unavailable<TClaims>() -> IntrospectionAdmission<TClaims> {
    refused(Response::text(
        503,
        "The authorization server cannot introspect the token",
    ))
}

/// # Errors
///
/// Returns `AuthorizationServerClientError::AssertionSigning` when the client assertion of a
/// `private_key_jwt` client cannot be signed.
pub async fn introspect_bearer_token<TClaims: DeserializeOwned>(
    authorization: &RequestAuthorization,
    server: &AuthorizationServerClient,
) -> Result<IntrospectionAdmission<TClaims>, AuthorizationServerClientError> {
    let token = match authorization {
        RequestAuthorization::Absent | RequestAuthorization::OtherScheme => {
            return Ok(IntrospectionAdmission::Unaddressed);
        }
        RequestAuthorization::Bearer(token) => token,
        RequestAuthorization::Malformed => {
            return Ok(refused(BearerChallenge::InvalidRequest.response()));
        }
    };

    Ok(
        match server
            .introspect::<ExtensionMembers>(token.as_str())
            .await?
        {
            EndpointOutcome::Answered(introspection) => match judge_introspection(
                &introspection,
                server.trusted_issuer.trust.token_trust(),
                Utc::now(),
            ) {
                IntrospectionVerdict::Accepted(token) => IntrospectionAdmission::Admitted(token),
                IntrospectionVerdict::Rejected(rejection) => {
                    warn!("Refusing an introspected bearer token: {rejection}");

                    refused(BearerChallenge::InvalidToken.response())
                }
            },
            EndpointOutcome::Refused(refusal) => {
                error!(
                    "The introspection endpoint refused to introspect a bearer token: {refusal}"
                );

                unavailable()
            }
            EndpointOutcome::Unavailable(unavailability) => {
                error!("Unable to introspect a bearer token: {unavailability}");

                unavailable()
            }
        },
    )
}
