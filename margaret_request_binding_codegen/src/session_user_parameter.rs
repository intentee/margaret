use crate::authenticated_user_application::AuthenticatedUserApplication;
use crate::authenticated_user_challenge::AuthenticatedUserChallenge;
use crate::bound_parameter::BoundParameter;
use crate::request_binding::RequestBinding;
use crate::session_user::SessionUser;

#[must_use]
pub fn session_user_parameter(parameters: &[BoundParameter]) -> Option<SessionUser<'_>> {
    parameters
        .iter()
        .find_map(|parameter| match &parameter.binding {
            RequestBinding::AuthenticatedUser {
                application:
                    application @ AuthenticatedUserApplication {
                        challenge: AuthenticatedUserChallenge::Session { .. },
                        ..
                    },
                ..
            } => Some(SessionUser {
                application,
                parameter,
            }),
            _ => None,
        })
}
