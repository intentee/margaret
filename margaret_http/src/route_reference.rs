use std::sync::Arc;

use crate::build_url::build_url;
use crate::redirect::Redirect;
use crate::url_segment::UrlSegment;

pub struct RouteReference {
    origin: Arc<str>,
    segments: &'static [UrlSegment],
    values: Vec<String>,
}

impl RouteReference {
    #[must_use]
    pub fn new(origin: Arc<str>, segments: &'static [UrlSegment], values: Vec<String>) -> Self {
        Self {
            origin,
            segments,
            values,
        }
    }

    #[must_use]
    pub fn permanent_redirect(&self) -> Redirect {
        Redirect::permanent(self.url())
    }

    #[must_use]
    pub fn temporary_redirect(&self) -> Redirect {
        Redirect::temporary(self.url())
    }

    #[must_use]
    pub fn url(&self) -> String {
        build_url(&self.origin, self.segments, &self.values)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::RouteReference;
    use crate::url_segment::UrlSegment;

    fn article_reference() -> RouteReference {
        RouteReference::new(
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
        assert_eq!(article_reference().url(), "http://localhost/articles/rust");
    }

    #[test]
    fn temporarily_redirects_to_the_reference_url() {
        let response = article_reference()
            .temporary_redirect()
            .into_response()
            .into_http();

        assert_eq!(response.status().as_u16(), 307);
        assert_eq!(
            response
                .headers()
                .get("location")
                .expect("the location header is present"),
            "http://localhost/articles/rust"
        );
    }

    #[test]
    fn permanently_redirects_to_the_reference_url() {
        let response = article_reference()
            .permanent_redirect()
            .into_response()
            .into_http();

        assert_eq!(response.status().as_u16(), 308);
        assert_eq!(
            response
                .headers()
                .get("location")
                .expect("the location header is present"),
            "http://localhost/articles/rust"
        );
    }
}
