use margaret_attributes::canonical_path::CanonicalPath;

pub struct StartupSingleton {
    pub concrete_path: CanonicalPath,
    pub field_name: String,
}
