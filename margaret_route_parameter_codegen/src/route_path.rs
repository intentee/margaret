use crate::route_url_template::route_url_template;
use crate::url_segment::UrlSegment;

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

    pub fn parameters(&self) -> impl Iterator<Item = &str> {
        self.segments.iter().filter_map(|segment| match segment {
            UrlSegment::CatchAll(name) => Some(name.as_str()),
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
