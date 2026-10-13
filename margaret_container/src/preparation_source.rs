use margaret_attributes::canonical_path::CanonicalPath;

pub enum PreparationSource {
    Constant(CanonicalPath),
    TokenIssuance,
}
