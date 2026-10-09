use margaret_accepted_clients::assertion_signing::AssertionSigning;
use margaret_oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EndpointAuthentication {
    pub methods: &'static [ClientAuthenticationMethod],
    pub signing: &'static [AssertionSigning],
}
