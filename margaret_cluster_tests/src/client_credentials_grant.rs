use margaret::framework::oauth_vocabulary::grant_type::GrantType;

#[must_use]
pub fn client_credentials_grant() -> [[&'static str; 2]; 1] {
    [["grant_type", GrantType::ClientCredentials.wire_name()]]
}
