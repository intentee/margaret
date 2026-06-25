use crate::canonical_path::CanonicalPath;

pub enum Resolution {
    Ambiguous(Vec<CanonicalPath>),
    NotFound,
    Resolved(CanonicalPath),
}
