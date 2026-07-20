trait Marker {}

#[singleton(provides = Marker)]
#[provides_endpoint(jwks)]
struct BadProvider;

impl BadProvider {
    #[constructor]
    fn new() -> Self {}
}
