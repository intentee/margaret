use margaret_attributes::canonical_path::CanonicalPath;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedRustType {
    pub arguments: Vec<ResolvedRustType>,
    pub path: CanonicalPath,
}
