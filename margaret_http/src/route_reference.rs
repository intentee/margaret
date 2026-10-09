use crate::redirect::Redirect;

pub struct RouteReference {
    url: String,
}

impl RouteReference {
    #[must_use]
    pub fn new(url: String) -> Self {
        Self { url }
    }

    #[must_use]
    pub fn permanent_redirect(&self) -> Redirect {
        Redirect::permanent(self.url.clone())
    }

    #[must_use]
    pub fn temporary_redirect(&self) -> Redirect {
        Redirect::temporary(self.url.clone())
    }

    #[must_use]
    pub fn url(&self) -> String {
        self.url.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::RouteReference;

    fn article_reference() -> RouteReference {
        RouteReference::new("http://localhost/articles/rust".to_string())
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
