use margaret::framework::macros::provides_userinfo_claims;
use margaret::framework::macros::singleton;

#[singleton]
#[provides_userinfo_claims]
pub struct ProfileClaims;
