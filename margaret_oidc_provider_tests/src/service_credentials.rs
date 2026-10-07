use crate::client_credentials::ClientCredentials;

pub const SERVICE_CREDENTIALS: ClientCredentials = ClientCredentials::Asserted {
    client_id: "service",
};
