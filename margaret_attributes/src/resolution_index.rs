use std::collections::HashMap;
use std::collections::HashSet;

use syn::Path;

use crate::canonical_path::CanonicalPath;
use crate::resolution::Resolution;

fn normalized_segments(written: &Path, referencing_root: &str) -> Vec<String> {
    let mut segments: Vec<String> = written
        .segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect();

    if segments.first().map(String::as_str) == Some("crate") {
        segments[0] = referencing_root.to_string();
    }

    segments
}

fn segments_match(written: &[String], candidate: &[String], absolute: bool) -> bool {
    if absolute {
        written == candidate
    } else {
        written.len() <= candidate.len() && written == &candidate[candidate.len() - written.len()..]
    }
}

pub struct ResolutionIndex {
    candidates_by_leaf: HashMap<String, Vec<CanonicalPath>>,
    roots: HashSet<String>,
}

impl ResolutionIndex {
    pub fn new(candidates: impl IntoIterator<Item = CanonicalPath>) -> Self {
        let mut candidates_by_leaf: HashMap<String, Vec<CanonicalPath>> = HashMap::new();
        let mut roots: HashSet<String> = HashSet::new();

        for candidate in candidates {
            let segments = candidate.segments();

            roots.insert(
                segments
                    .first()
                    .expect("a canonical path has at least one segment")
                    .clone(),
            );

            let leaf = segments
                .last()
                .expect("a canonical path has at least one segment")
                .clone();

            candidates_by_leaf.entry(leaf).or_default().push(candidate);
        }

        Self {
            candidates_by_leaf,
            roots,
        }
    }

    pub fn resolve(&self, written: &Path, referencing_root: &str) -> Resolution {
        let written_segments = normalized_segments(written, referencing_root);
        let leaf = written_segments
            .last()
            .expect("a written path has at least one segment");
        let absolute = written_segments
            .first()
            .is_some_and(|head| self.roots.contains(head));

        let Some(candidates) = self.candidates_by_leaf.get(leaf) else {
            return Resolution::NotFound;
        };

        let mut matches: Vec<CanonicalPath> = candidates
            .iter()
            .filter(|candidate| segments_match(&written_segments, candidate.segments(), absolute))
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
}

#[cfg(test)]
mod tests {
    use syn::Path;
    use syn::parse_quote;

    use super::ResolutionIndex;
    use crate::canonical_path::CanonicalPath;
    use crate::resolution::Resolution;

    fn candidate(segments: &[&str]) -> CanonicalPath {
        CanonicalPath::new(segments.iter().map(|segment| segment.to_string()).collect())
    }

    fn outcome(written: Path, candidates: &[CanonicalPath], referencing_root: &str) -> String {
        let index = ResolutionIndex::new(candidates.iter().cloned());

        match index.resolve(&written, referencing_root) {
            Resolution::Resolved(path) => path.to_string(),
            Resolution::NotFound => "<not found>".to_string(),
            Resolution::Ambiguous(_) => "<ambiguous>".to_string(),
        }
    }

    #[test]
    fn matches_a_trailing_segment_by_suffix() {
        assert_eq!(
            outcome(
                parse_quote!(Config),
                &[candidate(&["app", "config", "Config"])],
                "app",
            ),
            "app::config::Config"
        );
    }

    #[test]
    fn does_not_match_when_the_segment_differs() {
        assert_eq!(
            outcome(
                parse_quote!(Logger),
                &[candidate(&["app", "config", "Config"])],
                "app",
            ),
            "<not found>"
        );
    }

    #[test]
    fn does_not_match_when_the_written_path_is_longer_than_the_candidate() {
        assert_eq!(
            outcome(
                parse_quote!(deep::config::Config),
                &[candidate(&["app", "Config"])],
                "app",
            ),
            "<not found>"
        );
    }

    #[test]
    fn a_crate_anchored_path_matches_the_referencing_crate_in_full() {
        assert_eq!(
            outcome(
                parse_quote!(crate::config::Config),
                &[candidate(&["app", "config", "Config"])],
                "app",
            ),
            "app::config::Config"
        );
    }

    #[test]
    fn a_crate_anchored_path_rejects_a_partial_path() {
        assert_eq!(
            outcome(
                parse_quote!(crate::Config),
                &[candidate(&["app", "config", "Config"])],
                "app",
            ),
            "<not found>"
        );
    }

    #[test]
    fn a_foreign_crate_path_resolves_across_crates() {
        assert_eq!(
            outcome(
                parse_quote!(margaret_http::Response),
                &[
                    candidate(&["margaret_http", "Response"]),
                    candidate(&["crate", "Response"]),
                ],
                "crate",
            ),
            "margaret_http::Response"
        );
    }

    #[test]
    fn a_bare_suffix_shared_across_crates_is_ambiguous() {
        assert_eq!(
            outcome(
                parse_quote!(Config),
                &[
                    candidate(&["crate", "Config"]),
                    candidate(&["margaret_plugin", "Config"]),
                ],
                "crate",
            ),
            "<ambiguous>"
        );
    }

    #[test]
    fn a_leading_crate_resolves_relative_to_the_referencing_crate() {
        assert_eq!(
            outcome(
                parse_quote!(crate::Config),
                &[
                    candidate(&["crate", "Config"]),
                    candidate(&["margaret_plugin", "Config"]),
                ],
                "margaret_plugin",
            ),
            "margaret_plugin::Config"
        );
    }
}
