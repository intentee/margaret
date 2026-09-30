#[rustfmt::skip]
#[path = "../margaret/mod.rs"]
pub mod margaret;

pub mod ci_claims;
pub mod ci_exchanger;
pub mod ci_issuer;
pub mod get_authorize;
pub mod get_discovery;
pub mod get_jwks;
pub mod get_userinfo;
pub mod portal_client;
pub mod post_authorize;
pub mod post_consent;
pub mod post_introspect;
pub mod post_revoke;
pub mod post_token;
pub mod provider_issuer;
