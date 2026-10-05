use std::ops::ControlFlow;

use chrono::Utc;
use log::error;
use log::warn;
use serde::de::DeserializeOwned;

use margaret_authorization_server_client::authorization_server_client::AuthorizationServerClient;
use margaret_authorization_server_client::endpoint_outcome::EndpointOutcome;
use margaret_http::bearer_challenge::BearerChallenge;
use margaret_http::request_authorization::RequestAuthorization;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_http::token_admission::TokenAdmission;

use crate::extension_members::ExtensionMembers;
use crate::introspected_token::IntrospectedToken;
use crate::introspection_verdict::IntrospectionVerdict;
use crate::judge_introspection::judge_introspection;

fn refused<TToken>(response: Response) -> TokenAdmission<TToken> {
    TokenAdmission::Refused(ResponseContinuation::from(response))
}

fn unavailable<TToken>() -> TokenAdmission<TToken> {
    refused(Response::text(
        503,
        "The authorization server cannot introspect the token",
    ))
}

pub async fn introspect_bearer_token<TClaims: DeserializeOwned>(
    authorization: &RequestAuthorization,
    server: &AuthorizationServerClient,
) -> TokenAdmission<IntrospectedToken<TClaims>> {
    let token = match authorization.bearer() {
        ControlFlow::Continue(token) => token,
        ControlFlow::Break(BearerChallenge::MissingCredentials) => {
            return TokenAdmission::Unaddressed;
        }
        ControlFlow::Break(challenge) => return refused(challenge.response()),
    };

    match server.introspect::<ExtensionMembers>(token.as_str()).await {
        EndpointOutcome::Answered(introspection) => match judge_introspection(
            &introspection,
            server.trusted_issuer.trust.token_trust(),
            Utc::now(),
        ) {
            IntrospectionVerdict::Accepted(token) => TokenAdmission::Admitted(token),
            IntrospectionVerdict::Rejected(rejection) => {
                warn!("Refusing an introspected bearer token: {rejection}");

                refused(BearerChallenge::InvalidToken.response())
            }
        },
        EndpointOutcome::Refused(refusal) => {
            error!("The introspection endpoint refused to introspect a bearer token: {refusal}");

            unavailable()
        }
        EndpointOutcome::Unavailable(unavailability) => {
            error!("Unable to introspect a bearer token: {unavailability}");

            unavailable()
        }
    }
}
