use margaret_jwks_key_gen::jwks_secret::JwksSecret;

pub enum LoadedSecret {
    Absent,
    Present(Box<JwksSecret>),
}
