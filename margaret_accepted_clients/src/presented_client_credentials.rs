use zeroize::Zeroizing;

use margaret_http::request_authorization::RequestAuthorization;
use margaret_oauth_vocabulary::client_secret_basic::ClientSecretBasic;

pub enum PresentedClientCredentials {
    Absent,
    Basic {
        client_id: String,
        secret: Zeroizing<String>,
    },
    ClientId(String),
    ConflictingClientIds,
    Malformed,
}

impl PresentedClientCredentials {
    #[must_use]
    pub fn of(authorization: &RequestAuthorization, form_client_id: Option<&str>) -> Self {
        match authorization {
            RequestAuthorization::Absent => match form_client_id {
                Some(client_id) => Self::ClientId(client_id.to_string()),
                None => Self::Absent,
            },
            RequestAuthorization::Basic(credentials) => {
                match ClientSecretBasic::decode(credentials.user_id(), credentials.password()) {
                    Ok(ClientSecretBasic { client_id, .. })
                        if form_client_id
                            .is_some_and(|form_client_id| form_client_id != client_id) =>
                    {
                        Self::ConflictingClientIds
                    }
                    Ok(ClientSecretBasic { client_id, secret }) => {
                        Self::Basic { client_id, secret }
                    }
                    Err(_) => Self::Malformed,
                }
            }
            RequestAuthorization::Bearer(_)
            | RequestAuthorization::Malformed
            | RequestAuthorization::OtherScheme => Self::Malformed,
        }
    }
}
