use crate::client_credentials::ClientCredentials;

pub const LEDGER_CREDENTIALS: ClientCredentials = ClientCredentials::Asserted {
    client_id: "ledger",
};
