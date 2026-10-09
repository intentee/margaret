use serde::Deserialize;
use url::form_urlencoded::Serializer;
use validator::Validate;

use margaret_oauth_vocabulary::optional_parameter::optional_parameter;

use crate::named_parameter::NamedParameter;

#[derive(Debug, Deserialize, Validate)]
pub struct AuthorizationRequest {
    #[serde(default, deserialize_with = "optional_parameter")]
    pub client_id: Option<String>,
    #[serde(default, deserialize_with = "optional_parameter")]
    pub code_challenge: Option<String>,
    #[serde(default, deserialize_with = "optional_parameter")]
    pub code_challenge_method: Option<String>,
    #[serde(default, deserialize_with = "optional_parameter")]
    pub max_age: Option<String>,
    #[serde(default, deserialize_with = "optional_parameter")]
    pub nonce: Option<String>,
    #[serde(default, deserialize_with = "optional_parameter")]
    pub prompt: Option<String>,
    #[serde(default, deserialize_with = "optional_parameter")]
    pub redirect_uri: Option<String>,
    #[serde(default, deserialize_with = "optional_parameter")]
    pub request: Option<String>,
    #[serde(default, deserialize_with = "optional_parameter")]
    pub request_uri: Option<String>,
    #[serde(default, deserialize_with = "optional_parameter")]
    pub response_type: Option<String>,
    #[serde(default, deserialize_with = "optional_parameter")]
    pub scope: Option<String>,
    #[serde(default, deserialize_with = "optional_parameter")]
    pub state: Option<String>,
}

impl AuthorizationRequest {
    pub(crate) fn continued_at(
        &self,
        endpoint: &str,
        max_age: Option<&str>,
        prompt: Option<&str>,
    ) -> String {
        let mut pairs = Serializer::new(String::new());

        {
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
                    value: max_age,
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

        format!("{endpoint}?{}", pairs.finish())
    }
}
