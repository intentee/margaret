use crate::client_credentials::ClientCredentials;
use crate::portal_secret::PORTAL_SECRET;

pub const PORTAL_CREDENTIALS: ClientCredentials = ClientCredentials::Basic {
    client_id: "portal",
    secret: PORTAL_SECRET,
};
