use crate::literal_route_path::LiteralRoutePath;
use crate::route_url_template::route_url_template;
use crate::url_segment::UrlSegment;

#[derive(Clone)]
pub struct RoutePath {
    pattern: String,
    segments: Vec<UrlSegment>,
}

impl RoutePath {
    #[must_use]
    pub fn parse(pattern: &str) -> Self {
        Self {
            segments: route_url_template(pattern),
            pattern: pattern.to_owned(),
        }
    }

    #[must_use]
    pub fn literal(&self) -> LiteralRoutePath {
        let mut literal = String::new();

        for segment in &self.segments {
            match segment {
                UrlSegment::Literal(text) => literal.push_str(text),
                UrlSegment::CatchAllParameter(_) | UrlSegment::Parameter(_) => {
                    return LiteralRoutePath::Parameterized;
                }
            }
        }

        LiteralRoutePath::Literal(literal)
    }

    pub fn parameters(&self) -> impl Iterator<Item = &str> {
        self.segments.iter().filter_map(|segment| match segment {
            UrlSegment::CatchAllParameter(name) | UrlSegment::Parameter(name) => {
                Some(name.as_str())
            }
            UrlSegment::Literal(_) => None,
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
    use crate::literal_route_path::LiteralRoutePath;

    #[test]
    fn unescapes_the_literal_of_a_path_without_parameters() {
        assert!(matches!(
            RoutePath::parse("/oauth/{{token}}").literal(),
            LiteralRoutePath::Literal(literal) if literal == "/oauth/{token}"
        ));
    }

    #[test]
    fn finds_no_literal_in_a_path_with_parameters() {
        assert_eq!(
            RoutePath::parse("/articles/{article}").literal(),
            LiteralRoutePath::Parameterized
        );
    }

    #[test]
    fn lists_named_and_catch_all_parameters_and_skips_literals() {
        let route_path = RoutePath::parse("/files/{bucket}/{*rest}");

        assert_eq!(
            route_path.parameters().collect::<Vec<_>>(),
            vec!["bucket", "rest"]
        );
    }
}
