use crate::client_credentials::ClientCredentials;
use crate::service_secret::SERVICE_SECRET;

pub const SERVICE_CREDENTIALS: ClientCredentials = ClientCredentials::Basic {
    client_id: "service",
    secret: SERVICE_SECRET,
};
