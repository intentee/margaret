use margaret_attributes::tag::Tag;
use margaret_oauth_vocabulary::client_id::ClientId;

use crate::declared_client_authentication::DeclaredClientAuthentication;
use crate::declared_sign_in::DeclaredSignIn;

pub enum DeclaredClientRegistration {
    Admitted {
        admitted: Tag,
    },
    External {
        authentication: DeclaredClientAuthentication,
        client_id: ClientId,
        issuer: Tag,
        sign_in: DeclaredSignIn,
    },
}
