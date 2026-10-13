use std::iter;

use crate::constants::PATH_SEPARATOR;
use crate::path_segment::PathSegment;

pub fn path_segments(path: &str) -> impl Iterator<Item = PathSegment<'_>> {
    let mut components = path.split(PATH_SEPARATOR).peekable();

    iter::from_fn(move || {
        components.next().map(|component| {
            if component.is_empty() && components.peek().is_none() {
                PathSegment::TrailingSeparator
            } else {
                PathSegment::Text(component)
            }
        })
    })
}

#[cfg(test)]
mod tests {
    use super::path_segments;
    use crate::path_segment::PathSegment;

    fn segments(path: &str) -> Vec<PathSegment<'_>> {
        path_segments(path).collect()
    }

    #[test]
    fn splits_a_path_at_every_separator_so_each_segment_is_admitted_on_its_own() {
        assert_eq!(
            segments("articles/42"),
            vec![PathSegment::Text("articles"), PathSegment::Text("42")]
        );
    }

    #[test]
    fn marks_a_closing_separator_so_a_trailing_slash_is_kept_instead_of_rejected() {
        assert_eq!(
            segments("articles/"),
            vec![
                PathSegment::Text("articles"),
                PathSegment::TrailingSeparator
            ]
        );
    }

    #[test]
    fn reads_the_root_path_as_a_trailing_separator_so_it_is_never_an_empty_segment() {
        assert_eq!(segments(""), vec![PathSegment::TrailingSeparator]);
    }

    #[test]
    fn keeps_an_empty_interior_segment_so_callers_can_reject_it() {
        assert_eq!(
            segments("a//b"),
            vec![
                PathSegment::Text("a"),
                PathSegment::Text(""),
                PathSegment::Text("b")
            ]
        );
    }
}
