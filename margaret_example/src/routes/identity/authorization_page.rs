use margaret::framework::http::response::Response;
use margaret::framework::oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret::framework::oidc_provider::authorization_request::AuthorizationRequest;
use margaret::framework::oidc_provider::end_user_authentication::EndUserAuthentication;
use margaret::framework::validation::validation_result::ValidationResult;
use margaret::framework::views::renders_view::RendersView;

use crate::margaret::oidc_provider::AuthorizationEndpoint;
use crate::margaret::routes::Routes;
use crate::margaret::views::Views;
use crate::models::user::User;
use crate::views::consent_view::ConsentViewProps;

/// # Errors
///
/// Returns an error when the authorization cannot be stored or the consent page cannot be rendered.
pub async fn authorization_page(
    authorization_endpoint: &AuthorizationEndpoint,
    request: ValidationResult<AuthorizationRequest>,
    user: Option<User>,
    routes: &Routes,
    views: &Views,
) -> anyhow::Result<Response> {
    let end_user = match user {
        Some(user) => EndUserAuthentication::Authenticated(user.end_user()),
        None => EndUserAuthentication::Anonymous,
    };

    Ok(
        match authorization_endpoint.authorize(request, &end_user).await? {
            AuthorizationOutcome::AuthenticationRequired { return_to } => Response::text(
                401,
                format!("Sign in to the identity server, then continue at {return_to}"),
            ),
            AuthorizationOutcome::ConsentRequired(consent) => Response::html(
                200,
                views.consent_view.render(ConsentViewProps {
                    consent: &consent,
                    routes,
                })?,
            ),
            AuthorizationOutcome::Redirected(response)
            | AuthorizationOutcome::Rejected(response) => response,
        },
    )
}
