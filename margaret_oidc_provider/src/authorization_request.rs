use serde::Deserialize;
use url::Url;
use validator::Validate;
use validator::ValidationErrors;

use crate::named_parameter::NamedParameter;

#[derive(Debug, Deserialize)]
pub struct AuthorizationRequest {
    pub client_id: Option<String>,
    pub code_challenge: Option<String>,
    pub code_challenge_method: Option<String>,
    pub max_age: Option<String>,
    pub nonce: Option<String>,
    pub prompt: Option<String>,
    pub redirect_uri: Option<String>,
    pub response_type: Option<String>,
    pub scope: Option<String>,
    pub state: Option<String>,
}

impl AuthorizationRequest {
    pub(crate) fn continued_at(&self, endpoint: &Url, prompt: Option<&str>) -> Url {
        let mut continuation = Url::clone(endpoint);

        {
            let mut pairs = continuation.query_pairs_mut();
            let parameters = [
                NamedParameter {
                    name: "client_id",
                    value: self.client_id.as_deref(),
                },
                NamedParameter {
                    name: "code_challenge",
                    value: self.code_challenge.as_deref(),
                },
                NamedParameter {
                    name: "code_challenge_method",
                    value: self.code_challenge_method.as_deref(),
                },
                NamedParameter {
                    name: "max_age",
                    value: self.max_age.as_deref(),
                },
                NamedParameter {
                    name: "nonce",
                    value: self.nonce.as_deref(),
                },
                NamedParameter {
                    name: "prompt",
                    value: prompt,
                },
                NamedParameter {
                    name: "redirect_uri",
                    value: self.redirect_uri.as_deref(),
                },
                NamedParameter {
                    name: "response_type",
                    value: self.response_type.as_deref(),
                },
                NamedParameter {
                    name: "scope",
                    value: self.scope.as_deref(),
                },
                NamedParameter {
                    name: "state",
                    value: self.state.as_deref(),
                },
            ];

            for NamedParameter { name, value } in parameters {
                if let Some(value) = value {
                    pairs.append_pair(name, value);
                }
            }
        }

        continuation
    }
}

impl Validate for AuthorizationRequest {
    fn validate(&self) -> Result<(), ValidationErrors> {
        Ok(())
    }
}
