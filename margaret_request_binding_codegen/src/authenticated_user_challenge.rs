use margaret_container::injected_dependency::InjectedDependency;

use crate::bound_parameter::BoundParameter;
use crate::request_binding::RequestBinding;

#[derive(Clone)]
pub enum AuthenticatedUserChallenge {
    Bearer {
        trusted_issuers: Vec<InjectedDependency>,
    },
    Introspection {
        authorization_server: InjectedDependency,
    },
    Unchallenged,
}

impl AuthenticatedUserChallenge {
    #[must_use]
    pub fn required_by(parameters: &[BoundParameter]) -> Self {
        let mut trusted_issuers = Vec::new();

        for parameter in parameters {
            match &parameter.binding {
                RequestBinding::BearerToken { trusted_issuer, .. } => {
                    trusted_issuers.push(trusted_issuer.clone());
                }
                RequestBinding::IntrospectedBearerToken {
                    authorization_server,
                    ..
                } => {
                    return Self::Introspection {
                        authorization_server: authorization_server.clone(),
                    };
                }
                _ => {}
            }
        }

        if trusted_issuers.is_empty() {
            Self::Unchallenged
        } else {
            Self::Bearer { trusted_issuers }
        }
    }

    #[must_use]
    pub fn dependencies(&self) -> Vec<&InjectedDependency> {
        match self {
            Self::Bearer { trusted_issuers } => trusted_issuers.iter().collect(),
            Self::Introspection {
                authorization_server,
            } => vec![authorization_server],
            Self::Unchallenged => Vec::new(),
        }
    }
}
