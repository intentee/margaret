use margaret_endpoint::provides_endpoint::ProvidesEndpoint;

#[provides_endpoint(jwks)]
struct FirstEndpoint;

impl FirstEndpoint {
    #[constructor]
    fn new() -> Self {}
}

impl ProvidesEndpoint for FirstEndpoint {}

#[provides_endpoint(jwks)]
struct SecondEndpoint;

impl SecondEndpoint {
    #[constructor]
    fn new() -> Self {}
}

impl ProvidesEndpoint for SecondEndpoint {}
