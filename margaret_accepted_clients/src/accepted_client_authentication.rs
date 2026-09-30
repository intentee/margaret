use margaret_oauth_vocabulary::client_secret::ClientSecret;

pub enum AcceptedClientAuthentication {
    ClientSecretBasic(ClientSecret),
    Public,
}
