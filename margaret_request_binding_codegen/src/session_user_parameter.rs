use crate::authenticated_user_application::AuthenticatedUserApplication;
use crate::authenticated_user_challenge::AuthenticatedUserChallenge;
use crate::bound_parameter::BoundParameter;
use crate::request_binding::RequestBinding;

#[must_use]
pub fn session_user_parameter(parameters: &[BoundParameter]) -> Option<&BoundParameter> {
    parameters.iter().find(|parameter| {
        matches!(
            &parameter.binding,
            RequestBinding::AuthenticatedUser {
                application: AuthenticatedUserApplication {
                    challenge: AuthenticatedUserChallenge::Session { .. },
                    ..
                },
                ..
            }
        )
    })
}
