use crate::route_path_error::RoutePathError;
use crate::route_url_template::route_url_template;
use crate::url_segment::UrlSegment;

pub struct RoutePath {
    pattern: String,
    segments: Vec<UrlSegment>,
}

impl RoutePath {
    pub fn parse(pattern: &str) -> Result<Self, RoutePathError> {
        if !pattern.starts_with('/') {
            return Err(RoutePathError::MissingLeadingSlash);
        }
        if pattern.contains("//") {
            return Err(RoutePathError::EmptySegment);
        }
        if pattern
            .split('/')
            .any(|segment| segment == "." || segment == "..")
        {
            return Err(RoutePathError::DotSegment);
        }
        if pattern.contains('%') {
            return Err(RoutePathError::PercentEncoding);
        }
        if pattern.contains('\\') {
            return Err(RoutePathError::Backslash);
        }
        if pattern.contains(['?', '#']) {
            return Err(RoutePathError::QueryOrFragment);
        }
        if pattern.chars().any(char::is_control) {
            return Err(RoutePathError::ControlCharacter);
        }

        Ok(Self {
            segments: route_url_template(pattern),
            pattern: pattern.to_owned(),
        })
    }

    pub fn parameters(&self) -> impl Iterator<Item = &str> {
        self.segments.iter().filter_map(|segment| match segment {
            UrlSegment::Literal(_) => None,
            UrlSegment::Parameter(name) => Some(name.as_str()),
        })
    }

    #[must_use]
    pub fn pattern(&self) -> &str {
        &self.pattern
    }

    #[must_use]
    pub fn segments(&self) -> &[UrlSegment] {
        &self.segments
    }
}

#[cfg(test)]
mod tests {
    use super::RoutePath;

    #[test]
    fn accepts_a_canonical_route() {
        let path = RoutePath::parse("/articles/{article}").expect("the route is canonical");

        assert_eq!(path.pattern(), "/articles/{article}");
        assert_eq!(path.parameters().collect::<Vec<_>>(), vec!["article"]);
        assert!(!path.segments().is_empty());
    }

    #[test]
    fn rejects_a_route_without_a_leading_slash() {
        assert!(RoutePath::parse("relative").is_err());
    }

    #[test]
    fn rejects_an_empty_segment() {
        assert!(RoutePath::parse("/articles//draft").is_err());
    }

    #[test]
    fn rejects_dot_segments() {
        assert!(RoutePath::parse("/articles/../secret").is_err());
    }

    #[test]
    fn rejects_percent_encoding() {
        assert!(RoutePath::parse("/articles/%2e%2e/secret").is_err());
    }

    #[test]
    fn rejects_backslashes() {
        assert!(RoutePath::parse("/articles\\..\\secret").is_err());
    }

    #[test]
    fn rejects_query_and_fragment_delimiters() {
        assert!(RoutePath::parse("/articles?draft=true").is_err());
        assert!(RoutePath::parse("/articles#draft").is_err());
    }

    #[test]
    fn rejects_control_characters() {
        assert!(RoutePath::parse("/articles/\nsecret").is_err());
    }
}
