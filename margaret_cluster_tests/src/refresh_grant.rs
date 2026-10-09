use margaret::framework::oauth_vocabulary::grant_type::GrantType;

#[must_use]
pub fn refresh_grant(refresh_token: &str) -> [[&str; 2]; 2] {
    [
        ["grant_type", GrantType::RefreshToken.wire_name()],
        ["refresh_token", refresh_token],
    ]
}
