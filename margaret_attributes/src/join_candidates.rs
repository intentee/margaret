use crate::canonical_path::CanonicalPath;

pub fn join_candidates(candidates: &[CanonicalPath]) -> String {
    candidates
        .iter()
        .map(CanonicalPath::to_string)
        .collect::<Vec<String>>()
        .join(", ")
}
