use crate::bound_parameter::BoundParameter;
use crate::request_binding::RequestBinding;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AuthenticatedUserChallenge {
    Bearer,
    Unchallenged,
}

impl AuthenticatedUserChallenge {
    #[must_use]
    pub fn required_by(parameters: &[BoundParameter]) -> Self {
        if parameters
            .iter()
            .any(|parameter| matches!(parameter.binding, RequestBinding::OidcToken { .. }))
        {
            Self::Bearer
        } else {
            Self::Unchallenged
        }
    }
}
