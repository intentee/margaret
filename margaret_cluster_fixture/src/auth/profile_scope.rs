use margaret::framework::macros::oauth_scope;

#[oauth_scope(name = "profile")]
pub struct ProfileScope;
