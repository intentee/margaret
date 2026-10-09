#[rustfmt::skip]
#[path = "../margaret/mod.rs"]
pub mod margaret;

pub mod artifacts_read_scope;
pub mod artifacts_resource;
pub mod browser_sessions;
pub mod ci_claims;
pub mod ci_exchanger;
pub mod ci_issuer;
pub mod consent_view;
pub mod fixture_database;
pub mod fixture_userinfo_claims;
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
pub mod reports_resource;
pub mod spa_client;
