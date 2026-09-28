use margaret_container::injected_dependency::InjectedDependency;

use crate::bound_parameter::BoundParameter;
use crate::request_binding::RequestBinding;

#[derive(Clone)]
pub enum AuthenticatedUserChallenge {
    Bearer { issuer_client: InjectedDependency },
    Unchallenged,
}

impl AuthenticatedUserChallenge {
    #[must_use]
    pub fn required_by(parameters: &[BoundParameter]) -> Self {
        parameters
            .iter()
            .find_map(|parameter| match &parameter.binding {
                RequestBinding::BearerToken { issuer_client, .. } => Some(Self::Bearer {
                    issuer_client: issuer_client.clone(),
                }),
                _ => None,
            })
            .unwrap_or(Self::Unchallenged)
    }
}
