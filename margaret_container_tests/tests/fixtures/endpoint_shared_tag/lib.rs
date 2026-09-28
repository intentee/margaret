use margaret::framework::jwks_endpoint::provides_endpoint::ProvidesEndpoint;

#[provides_jwks_endpoint(jwks)]
#[singleton]
struct PrimaryEndpoint;

impl ProvidesEndpoint for PrimaryEndpoint {}

#[provides_jwks_endpoint(jwks)]
#[singleton]
struct SecondaryEndpoint;

impl ProvidesEndpoint for SecondaryEndpoint {}
