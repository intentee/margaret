use margaret::framework::http::response::Response;
use margaret::framework::oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret::framework::oidc_provider::authorization_request::AuthorizationRequest;
use margaret::framework::oidc_provider::end_user_authentication::EndUserAuthentication;
use margaret::framework::validation::validation_result::ValidationResult;

use crate::margaret::oidc_provider::AuthorizationEndpoint;
use crate::models::user::User;
use crate::routes::identity::consent_required::ConsentRequired;

/// # Errors
///
/// Returns an error when the pending authorization cannot be stored.
pub async fn authorization_response(
    authorization_endpoint: &AuthorizationEndpoint,
    request: ValidationResult<AuthorizationRequest>,
    user: Option<User>,
) -> anyhow::Result<Response> {
    let end_user = match user {
        Some(user) => EndUserAuthentication::Authenticated(user.end_user()),
        None => EndUserAuthentication::Anonymous,
    };

    Ok(
        match authorization_endpoint.authorize(request, &end_user).await? {
            AuthorizationOutcome::AuthenticationRequired { return_to } => {
                Response::text(401, return_to)
            }
            AuthorizationOutcome::ConsentRequired(consent) => Response::json(
                200,
                &ConsentRequired {
                    consent: consent.id,
                },
            ),
            AuthorizationOutcome::Redirected(response)
            | AuthorizationOutcome::Rejected(response) => response,
        },
    )
}
