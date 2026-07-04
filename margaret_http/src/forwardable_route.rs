use std::sync::Arc;

use crate::build_url::build_url;
use crate::forward::Forward;
use crate::url_segment::UrlSegment;

pub struct ForwardableRoute {
    name: &'static str,
    origin: Arc<str>,
    segments: &'static [UrlSegment],
    values: Vec<String>,
}

impl ForwardableRoute {
    pub fn new(
        name: &'static str,
        origin: Arc<str>,
        segments: &'static [UrlSegment],
        values: Vec<String>,
    ) -> Self {
        Self {
            name,
            origin,
            segments,
            values,
        }
    }

    pub fn forward_to(&self) -> Forward {
        let path_params = self
            .segments
            .iter()
            .filter_map(|segment| match segment {
                UrlSegment::Parameter(name) => Some(*name),
                UrlSegment::Literal(_) => None,
            })
            .zip(self.values.iter().cloned())
            .map(|(name, value)| (name.to_string(), value))
            .collect();

        Forward::new(self.name, path_params)
    }

    pub fn url(&self) -> String {
        build_url(&self.origin, self.segments, &self.values)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::ForwardableRoute;
    use crate::url_segment::UrlSegment;

    fn article_route() -> ForwardableRoute {
        ForwardableRoute::new(
            "get_article",
            Arc::from("http://localhost"),
            &[
                UrlSegment::Literal("/articles/"),
                UrlSegment::Parameter("article"),
            ],
            vec!["rust".to_string()],
        )
    }

    #[test]
    fn renders_the_url_from_its_origin_and_parameters() {
        assert_eq!(article_route().url(), "http://localhost/articles/rust");
    }

    #[test]
    fn forwards_to_the_named_route_with_its_path_parameters() {
        let forward = article_route().forward_to();

        assert_eq!(forward.name(), "get_article");
        assert_eq!(
            forward
                .into_path_params()
                .get("article")
                .map(String::as_str),
            Some("rust")
        );
    }
}
