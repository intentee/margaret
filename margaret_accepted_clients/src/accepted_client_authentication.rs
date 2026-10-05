use margaret_oauth_vocabulary::client_secret::ClientSecret;

use crate::confidential_privileges::ConfidentialPrivileges;

pub enum AcceptedClientAuthentication {
    ClientSecretBasic {
        privileges: ConfidentialPrivileges,
        secret: ClientSecret,
    },
    Public,
}
