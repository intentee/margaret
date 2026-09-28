use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) enum DeclaredTokenIssuance {
    Absent,
    Declared(CanonicalPath),
}
