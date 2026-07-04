use std::sync::Arc;

use crate::build_url::build_url;
use crate::url_segment::UrlSegment;

pub struct UrlReference {
    origin: Arc<str>,
    segments: &'static [UrlSegment],
    values: Vec<String>,
}

impl UrlReference {
    pub fn new(origin: Arc<str>, segments: &'static [UrlSegment], values: Vec<String>) -> Self {
        Self {
            origin,
            segments,
            values,
        }
    }

    pub fn url(&self) -> String {
        build_url(&self.origin, self.segments, &self.values)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::UrlReference;
    use crate::url_segment::UrlSegment;

    #[test]
    fn renders_the_url_from_its_origin_and_parameters() {
        let reference = UrlReference::new(
            Arc::from("http://localhost"),
            &[
                UrlSegment::Literal("/articles/"),
                UrlSegment::Parameter("article"),
            ],
            vec!["rust".to_string()],
        );

        assert_eq!(reference.url(), "http://localhost/articles/rust");
    }
}
