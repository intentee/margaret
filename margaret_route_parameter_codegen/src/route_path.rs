use crate::route_path_error::RoutePathError;
use crate::route_url_template::RouteUrlTemplate;
use crate::url_segment::UrlSegment;

#[derive(Clone)]
pub struct RoutePath {
    pattern: String,
    template: RouteUrlTemplate,
}

impl RoutePath {
    /// # Errors
    ///
    /// Returns `RoutePathError` when the pattern cannot describe a route a request can reach.
    pub fn parse(pattern: &str) -> Result<Self, RoutePathError> {
        Ok(Self {
            pattern: pattern.to_owned(),
            template: RouteUrlTemplate::parse(pattern)?,
        })
    }

    pub fn parameters(&self) -> impl Iterator<Item = &str> {
        let segments: &[UrlSegment] = match &self.template {
            RouteUrlTemplate::Literal(_) => &[],
            RouteUrlTemplate::Parameterized(segments) => segments,
        };

        segments.iter().filter_map(|segment| match segment {
            UrlSegment::CatchAllParameter { name, .. } | UrlSegment::Parameter { name, .. } => {
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
    pub fn template(&self) -> &RouteUrlTemplate {
        &self.template
    }
}

#[cfg(test)]
mod tests {
    use super::RoutePath;
    use crate::route_url_template::RouteUrlTemplate;

    fn parsed(pattern: &str) -> RoutePath {
        RoutePath::parse(pattern).expect("the route path is routable")
    }

    #[test]
    fn keeps_the_pattern_it_was_declared_with() {
        assert_eq!(parsed("/oauth/{{token}}").pattern(), "/oauth/{{token}}");
    }

    #[test]
    fn encodes_the_literal_of_a_path_without_parameters() {
        assert_eq!(
            parsed("/oauth/{{token}}").template(),
            &RouteUrlTemplate::Literal("/oauth/%7Btoken%7D".to_string())
        );
    }

    #[test]
    fn lists_named_and_catch_all_parameters_and_skips_literals() {
        assert_eq!(
            parsed("/files/{bucket}/{*rest}")
                .parameters()
                .collect::<Vec<_>>(),
            vec!["bucket", "rest"]
        );
    }

    #[test]
    fn lists_no_parameters_of_a_literal_path() {
        assert_eq!(parsed("/greeting").parameters().count(), 0);
    }
}
