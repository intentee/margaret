use margaret_attributes::canonical_path::CanonicalPath;

pub struct UriSelectedProvider {
    pub argument_name: String,
    pub resolver: CanonicalPath,
    pub trait_path: CanonicalPath,
    pub value_type: CanonicalPath,
}
