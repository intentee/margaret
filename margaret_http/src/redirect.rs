use http::StatusCode;

use crate::response::Response;

pub struct Redirect {
    location: String,
    status: StatusCode,
}

impl Redirect {
    #[must_use]
    pub fn permanent(location: String) -> Self {
        Self {
            location,
            status: StatusCode::PERMANENT_REDIRECT,
        }
    }

    #[must_use]
    pub fn see_other(location: String) -> Self {
        Self {
            location,
            status: StatusCode::SEE_OTHER,
        }
    }

    #[must_use]
    pub fn temporary(location: String) -> Self {
        Self {
            location,
            status: StatusCode::TEMPORARY_REDIRECT,
        }
    }

    #[must_use]
    pub fn into_response(self) -> Response {
        Response::text(self.status.as_u16(), "").header("location", self.location)
    }
}

#[cfg(test)]
mod tests {
    use super::Redirect;

    #[test]
    fn see_other_redirects_with_303_so_the_client_follows_with_a_get() {
        let response = Redirect::see_other("http://localhost/greeting".to_string())
            .into_response()
            .into_http();

        assert_eq!(response.status().as_u16(), 303);
        assert_eq!(
            response
                .headers()
                .get("location")
                .expect("the location header is present"),
            "http://localhost/greeting"
        );
    }

    #[test]
    fn temporary_redirects_with_307_so_the_client_repeats_the_method_at_the_location() {
        let response = Redirect::temporary("http://localhost/greeting".to_string())
            .into_response()
            .into_http();

        assert_eq!(response.status().as_u16(), 307);
        assert_eq!(
            response
                .headers()
                .get("location")
                .expect("the location header is present"),
            "http://localhost/greeting"
        );
    }

    #[test]
    fn permanent_redirects_with_308_so_the_client_keeps_the_method_and_may_cache_the_move() {
        let response = Redirect::permanent("http://localhost/greeting".to_string())
            .into_response()
            .into_http();

        assert_eq!(response.status().as_u16(), 308);
        assert_eq!(
            response
                .headers()
                .get("location")
                .expect("the location header is present"),
            "http://localhost/greeting"
        );
    }
}
