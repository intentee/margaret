use syn::Path;

use crate::canonical_path::CanonicalPath;
use crate::resolution::Resolution;

pub fn resolve_unique(written: &Path, candidates: &[CanonicalPath]) -> Resolution {
    let written_segments: Vec<String> = written
        .segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect();
    let mut matches: Vec<CanonicalPath> = candidates
        .iter()
        .filter(|candidate| suffix_matches(&written_segments, candidate.segments()))
        .cloned()
        .collect();

    if matches.len() > 1 {
        return Resolution::Ambiguous(matches);
    }

    match matches.pop() {
        Some(single) => Resolution::Resolved(single),
        None => Resolution::NotFound,
    }
}

fn suffix_matches(written: &[String], candidate: &[String]) -> bool {
    match written.first() {
        Some(first) if first == "crate" => {
            written.len() == candidate.len() && written[1..] == candidate[1..]
        }
        _ => {
            written.len() <= candidate.len()
                && written == &candidate[candidate.len() - written.len()..]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::suffix_matches;

    fn owned(segments: &[&str]) -> Vec<String> {
        segments.iter().map(|segment| segment.to_string()).collect()
    }

    #[test]
    fn matches_trailing_segment() {
        assert!(suffix_matches(
            &owned(&["Config"]),
            &owned(&["app", "config", "Config"])
        ));
    }

    #[test]
    fn rejects_when_segment_differs() {
        assert!(!suffix_matches(
            &owned(&["Logger"]),
            &owned(&["app", "config", "Config"])
        ));
    }

    #[test]
    fn rejects_when_written_is_longer() {
        assert!(!suffix_matches(
            &owned(&["deep", "config", "Config"]),
            &owned(&["app", "Config"])
        ));
    }

    #[test]
    fn crate_anchored_matches_full_path() {
        assert!(suffix_matches(
            &owned(&["crate", "config", "Config"]),
            &owned(&["app", "config", "Config"])
        ));
    }

    #[test]
    fn crate_anchored_rejects_partial_path() {
        assert!(!suffix_matches(
            &owned(&["crate", "Config"]),
            &owned(&["app", "config", "Config"])
        ));
    }
}
