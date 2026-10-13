use margaret_attributes::canonical_path::CanonicalPath;

#[derive(Clone)]
pub enum ServerOriginSource {
    Argument,
    Issuer { endpoints: CanonicalPath },
}
