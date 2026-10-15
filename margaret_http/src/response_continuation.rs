use crate::redirect::Redirect;
use crate::response::Response;

pub enum ResponseContinuation {
    Done(Response),
    Redirect(Redirect),
}

impl ResponseContinuation {
    pub(crate) fn into_response(self) -> Response {
        match self {
            Self::Done(response) => response,
            Self::Redirect(redirect) => redirect.into_response(),
        }
    }
}

impl From<Redirect> for ResponseContinuation {
    fn from(redirect: Redirect) -> Self {
        Self::Redirect(redirect)
    }
}

impl From<Response> for ResponseContinuation {
    fn from(response: Response) -> Self {
        Self::Done(response)
    }
}

#[cfg(test)]
mod tests {
    use http::HeaderName;
    use http::header::LOCATION;

    use super::ResponseContinuation;
    use crate::redirect::Redirect;
    use crate::response::Response;

    #[test]
    fn answers_with_the_finished_response_unchanged() {
        let response =
            ResponseContinuation::from(Response::text(418, "teapot").header("x-fixture", "kept"))
                .into_response();

        assert_eq!(response.status(), 418);
        assert_eq!(
            response.header_value(&HeaderName::from_static("x-fixture")),
            Some("kept")
        );
    }

    #[test]
    fn answers_a_redirect_with_its_status_and_location() {
        let response = ResponseContinuation::from(Redirect::temporary(
            "https://fixture.test/elsewhere".to_string(),
        ))
        .into_response();

        assert_eq!(response.status(), 307);
        assert_eq!(
            response.header_value(&LOCATION),
            Some("https://fixture.test/elsewhere")
        );
    }
}
