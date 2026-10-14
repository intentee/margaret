use crate::redirect::Redirect;

pub struct GetRouteReference {
    url: String,
}

impl GetRouteReference {
    #[must_use]
    pub fn new(url: String) -> Self {
        Self { url }
    }

    #[must_use]
    pub fn permanent_redirect(&self) -> Redirect {
        Redirect::permanent(self.url.clone())
    }

    #[must_use]
    pub fn see_other(&self) -> Redirect {
        Redirect::see_other(self.url.clone())
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
    use super::GetRouteReference;

    fn article_route() -> GetRouteReference {
        GetRouteReference::new("http://localhost/articles/rust".to_string())
    }

    #[test]
    fn sees_other_to_the_route_url_with_303_so_the_client_follows_up_with_a_get() {
        let response = article_route().see_other().into_response().into_http();

        assert_eq!(response.status().as_u16(), 303);
        assert_eq!(
            response
                .headers()
                .get("location")
                .expect("the location header is present"),
            "http://localhost/articles/rust"
        );
    }

    #[test]
    fn temporarily_redirects_to_the_route_url_with_307_keeping_the_request_method() {
        let response = article_route()
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
    fn permanently_redirects_to_the_route_url_with_308_so_clients_cache_the_move() {
        let response = article_route()
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
