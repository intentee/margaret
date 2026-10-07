use margaret_attributes::canonical_path::CanonicalPath;

pub struct ServedOriginCheck {
    pub endpoints: CanonicalPath,
    pub server: String,
}
