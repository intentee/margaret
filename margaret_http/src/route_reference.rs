use std::marker::PhantomData;

use crate::get_target::GetTarget;
use crate::redirect::Redirect;

pub struct RouteReference<TTarget> {
    target: PhantomData<TTarget>,
    url: String,
}

impl<TTarget> RouteReference<TTarget> {
    #[must_use]
    pub fn new(url: String) -> Self {
        Self {
            target: PhantomData,
            url,
        }
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

impl RouteReference<GetTarget> {
    #[must_use]
    pub fn see_other(&self) -> Redirect {
        Redirect::see_other(self.url.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::RouteReference;
    use crate::get_target::GetTarget;
    use crate::other_target::OtherTarget;

    const ARTICLE_URL: &str = "http://localhost/articles/rust";

    #[test]
    fn sees_other_to_a_get_route_with_303_so_the_client_follows_up_with_a_get() {
        let response = RouteReference::<GetTarget>::new(ARTICLE_URL.to_string())
            .see_other()
            .into_response()
            .into_http();

        assert_eq!(response.status().as_u16(), 303);
        assert_eq!(
            response
                .headers()
                .get("location")
                .expect("the location header is present"),
            ARTICLE_URL
        );
    }

    #[test]
    fn temporarily_redirects_to_the_reference_url_with_307_keeping_the_request_method() {
        let response = RouteReference::<OtherTarget>::new(ARTICLE_URL.to_string())
            .temporary_redirect()
            .into_response()
            .into_http();

        assert_eq!(response.status().as_u16(), 307);
        assert_eq!(
            response
                .headers()
                .get("location")
                .expect("the location header is present"),
            ARTICLE_URL
        );
    }

    #[test]
    fn permanently_redirects_to_the_reference_url_with_308_so_clients_cache_the_move() {
        let response = RouteReference::<OtherTarget>::new(ARTICLE_URL.to_string())
            .permanent_redirect()
            .into_response()
            .into_http();

        assert_eq!(response.status().as_u16(), 308);
        assert_eq!(
            response
                .headers()
                .get("location")
                .expect("the location header is present"),
            ARTICLE_URL
        );
    }
}
