use margaret_jwks_keygen::jwks_secret::JwksSecret;

pub enum LoadedSecret {
    Absent,
    Present(Box<JwksSecret>),
}
