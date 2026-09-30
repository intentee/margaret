use std::sync::Arc;

use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
use margaret_oauth_vocabulary::client_secret::ClientSecret;

pub enum ClientAuthentication {
    ClientSecretBasic(ClientSecret),
    PrivateKeyJwt(Arc<JwksSecretStore>),
}
