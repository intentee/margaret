use std::collections::HashMap;
use std::sync::Arc;

use aws_lc_rs::constant_time::verify_slices_are_equal;
use aws_lc_rs::digest::SHA256;
use aws_lc_rs::digest::SHA256_OUTPUT_LEN;
use aws_lc_rs::digest::digest;
use uuid::Uuid;

use margaret_oauth_vocabulary::client_id::ClientId;
use margaret_token_issuance::declares_token_issuance::DeclaresTokenIssuance;

use crate::accepted_client::AcceptedClient;
use crate::accepted_client_authentication::AcceptedClientAuthentication;
use crate::accepted_clients_error::AcceptedClientsError;
use crate::client_authentication_outcome::ClientAuthenticationOutcome;
use crate::client_refusal::ClientRefusal;
use crate::declares_accepted_client::DeclaresAcceptedClient;
use crate::grant_type::GrantType;
use crate::presented_client_credentials::PresentedClientCredentials;
use crate::registered_client::RegisteredClient;
use crate::registered_secret::RegisteredSecret;

fn secret_digest(secret: &str) -> [u8; SHA256_OUTPUT_LEN] {
    let mut computed = [0; SHA256_OUTPUT_LEN];

    computed.copy_from_slice(digest(&SHA256, secret.as_bytes()).as_ref());

    computed
}

fn registered_secret(
    client: &AcceptedClient,
    issuance: &dyn DeclaresTokenIssuance,
) -> Result<RegisteredSecret, AcceptedClientsError> {
    let client_id = || client.client_id.clone();

    if Uuid::try_parse(client.client_id.as_str()).is_ok() {
        return Err(AcceptedClientsError::UuidClientId {
            client_id: client_id(),
        });
    }

    if client.resources.is_empty() {
        return Err(AcceptedClientsError::WithoutResource {
            client_id: client_id(),
        });
    }

    let session_audience = &issuance.token_issuance().audience;

    if client.resources.contains(session_audience) {
        return Err(AcceptedClientsError::SessionAudienceResource {
            audience: session_audience.clone(),
            client_id: client_id(),
        });
    }

    if client.grants.contains(&GrantType::AuthorizationCode) && client.redirect_uris.is_empty() {
        return Err(AcceptedClientsError::WithoutRedirectUri {
            client_id: client_id(),
        });
    }

    match &client.authentication {
        AcceptedClientAuthentication::ClientSecretBasic(secret) => {
            Ok(RegisteredSecret::Digest(secret_digest(secret.expose())))
        }
        AcceptedClientAuthentication::Public
            if client.grants.contains(&GrantType::ClientCredentials) =>
        {
            Err(AcceptedClientsError::PublicClientCredentials {
                client_id: client_id(),
            })
        }
        AcceptedClientAuthentication::Public => Ok(RegisteredSecret::Public),
    }
}

pub struct AcceptedClients {
    clients: HashMap<ClientId, RegisteredClient>,
}

impl AcceptedClients {
    /// # Errors
    ///
    /// Returns `AcceptedClientsError` when a client is declared twice, is identified by a uuid,
    /// names no resource or the session audience as one, grants authorization codes without a
    /// redirect uri, or is public while granting client credentials.
    pub fn create(
        declarations: Vec<Arc<dyn DeclaresAcceptedClient>>,
        issuance: &dyn DeclaresTokenIssuance,
    ) -> Result<Self, AcceptedClientsError> {
        let mut clients = HashMap::new();

        for declaration in declarations {
            let client = declaration.accepted_client();
            let secret = registered_secret(client, issuance)?;
            let client_id = client.client_id.clone();

            if clients.contains_key(&client_id) {
                return Err(AcceptedClientsError::DuplicateClientId { client_id });
            }

            clients.insert(
                client_id,
                RegisteredClient {
                    declaration,
                    secret,
                },
            );
        }

        Ok(Self { clients })
    }

    #[must_use]
    pub fn authenticate(
        &self,
        presented: &PresentedClientCredentials,
    ) -> ClientAuthenticationOutcome<'_> {
        match presented {
            PresentedClientCredentials::Absent => {
                ClientAuthenticationOutcome::Refused(ClientRefusal::MissingCredentials)
            }
            PresentedClientCredentials::Basic { client_id, secret } => {
                self.with_client(client_id, |registered| match &registered.secret {
                    RegisteredSecret::Digest(expected) => {
                        if verify_slices_are_equal(&secret_digest(secret), expected).is_ok() {
                            ClientAuthenticationOutcome::Authenticated(
                                registered.declaration.accepted_client(),
                            )
                        } else {
                            ClientAuthenticationOutcome::Refused(ClientRefusal::WrongSecret)
                        }
                    }
                    RegisteredSecret::Public => {
                        ClientAuthenticationOutcome::Refused(ClientRefusal::UnexpectedSecret)
                    }
                })
            }
            PresentedClientCredentials::ClientId(client_id) => {
                self.with_client(client_id, |registered| match &registered.secret {
                    RegisteredSecret::Digest(_) => {
                        ClientAuthenticationOutcome::Refused(ClientRefusal::SecretRequired)
                    }
                    RegisteredSecret::Public => ClientAuthenticationOutcome::Authenticated(
                        registered.declaration.accepted_client(),
                    ),
                })
            }
            PresentedClientCredentials::ConflictingClientIds => {
                ClientAuthenticationOutcome::Refused(ClientRefusal::ConflictingClientIds)
            }
            PresentedClientCredentials::Malformed => {
                ClientAuthenticationOutcome::Refused(ClientRefusal::MalformedCredentials)
            }
        }
    }

    pub fn clients(&self) -> impl Iterator<Item = &AcceptedClient> {
        self.clients
            .values()
            .map(|registered| registered.declaration.accepted_client())
    }

    #[must_use]
    pub fn find(&self, client_id: &str) -> Option<&AcceptedClient> {
        client_id
            .parse::<ClientId>()
            .ok()
            .and_then(|client_id| self.clients.get(&client_id))
            .map(|registered| registered.declaration.accepted_client())
    }

    fn with_client<'clients>(
        &'clients self,
        client_id: &str,
        authenticate: impl FnOnce(&'clients RegisteredClient) -> ClientAuthenticationOutcome<'clients>,
    ) -> ClientAuthenticationOutcome<'clients> {
        match client_id
            .parse::<ClientId>()
            .ok()
            .and_then(|client_id| self.clients.get(&client_id))
        {
            Some(registered) => authenticate(registered),
            None => ClientAuthenticationOutcome::Refused(ClientRefusal::UnknownClient),
        }
    }
}
