use http::StatusCode;
use http::header::LOCATION;

use crate::response::Response;

pub struct Redirect {
    location: String,
    status: StatusCode,
}

impl Redirect {
    pub(crate) fn permanent(location: String) -> Self {
        Self {
            location,
            status: StatusCode::PERMANENT_REDIRECT,
        }
    }

    pub(crate) fn see_other(location: String) -> Self {
        Self {
            location,
            status: StatusCode::SEE_OTHER,
        }
    }

    pub(crate) fn temporary(location: String) -> Self {
        Self {
            location,
            status: StatusCode::TEMPORARY_REDIRECT,
        }
    }

    pub(crate) fn into_response(self) -> Response {
        Response::text(self.status.as_u16(), "").reserved_header(LOCATION, self.location)
    }
}

#[cfg(test)]
mod tests {
    use super::Redirect;

    #[test]
    fn see_other_redirects_with_303() {
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
    fn temporary_redirects_with_307() {
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
    fn permanent_redirects_with_308() {
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
