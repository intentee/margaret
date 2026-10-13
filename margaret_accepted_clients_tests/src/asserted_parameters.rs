use margaret_accepted_clients::client_authentication_parameters::ClientAuthenticationParameters;
use margaret_oauth_vocabulary::constants::JWT_BEARER_CLIENT_ASSERTION_TYPE;

#[must_use]
pub fn asserted_parameters(assertion: String) -> ClientAuthenticationParameters {
    ClientAuthenticationParameters {
        client_assertion: Some(assertion),
        client_assertion_type: Some(JWT_BEARER_CLIENT_ASSERTION_TYPE.to_string()),
        client_id: None,
    }
}
