#[singleton]
#[provides_endpoint(jwks)]
struct First;

impl First {
    #[constructor]
    fn new() -> Self {}
}

#[singleton]
#[provides_endpoint(jwks)]
struct Second;

impl Second {
    #[constructor]
    fn new() -> Self {}
}
