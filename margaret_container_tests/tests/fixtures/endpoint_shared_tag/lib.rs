use margaret::framework::jwks_endpoint::provides_endpoint::ProvidesEndpoint;
use margaret::framework::token_trust::declares_token_trust::DeclaresTokenTrust;

#[provides_jwks_endpoint(jwks)]
#[singleton]
struct PrimaryEndpoint;

impl ProvidesEndpoint for PrimaryEndpoint {}

impl DeclaresTokenTrust for PrimaryEndpoint {}

#[provides_jwks_endpoint(jwks)]
#[singleton]
struct SecondaryEndpoint;

impl ProvidesEndpoint for SecondaryEndpoint {}

impl DeclaresTokenTrust for SecondaryEndpoint {}
