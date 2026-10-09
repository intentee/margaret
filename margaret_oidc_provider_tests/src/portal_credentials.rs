use crate::client_credentials::ClientCredentials;

pub const PORTAL_CREDENTIALS: ClientCredentials = ClientCredentials::Asserted {
    client_id: "portal",
};
