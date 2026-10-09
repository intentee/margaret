use cookie::Cookie;

use crate::response::Response;
use crate::response_continuation::ResponseContinuation;

#[derive(Clone)]
pub struct CookieChanges {
    pub cookies: Vec<Cookie<'static>>,
}

impl CookieChanges {
    #[must_use]
    pub fn apply(&self, continuation: ResponseContinuation) -> ResponseContinuation {
        match continuation {
            ResponseContinuation::Done(response) => {
                ResponseContinuation::Done(self.applied_to(response))
            }
            ResponseContinuation::Forward(forward) => {
                ResponseContinuation::Forward(forward.carrying(&self.cookies))
            }
            ResponseContinuation::Redirect(redirect) => {
                ResponseContinuation::Done(self.applied_to(redirect.into_response()))
            }
        }
    }

    fn applied_to(&self, response: Response) -> Response {
        self.cookies
            .iter()
            .fold(response, |response, cookie| response.set_cookie(cookie))
    }
}

#[cfg(test)]
mod tests {
    use cookie::Cookie;
    use http::header::LOCATION;
    use http::header::SET_COOKIE;

    use super::CookieChanges;
    use crate::redirect::Redirect;
    use crate::response::Response;
    use crate::response_continuation::ResponseContinuation;

    fn set_cookies(response: &Response) -> Vec<&str> {
        response
            .headers()
            .iter()
            .filter(|header| header.name == SET_COOKIE.as_str())
            .map(|header| header.value.as_str())
            .collect()
    }

    #[test]
    fn sets_every_changed_cookie_on_a_finished_response() {
        assert!(matches!(
            CookieChanges {
                cookies: vec![Cookie::new("access", "token"), Cookie::new("secret", "value")],
            }
            .apply(ResponseContinuation::from(Response::text(200, ""))),
            ResponseContinuation::Done(response)
                if set_cookies(&response) == ["access=token", "secret=value"]
        ));
    }

    #[test]
    fn finishes_a_redirect_with_the_changed_cookies() {
        assert!(matches!(
            CookieChanges {
                cookies: vec![Cookie::new("access", "token")],
            }
            .apply(
                ResponseContinuation::from(Redirect::see_other("https://fixture.test/".to_string())),
            ),
            ResponseContinuation::Done(response)
                if response.status() == 303
                    && response.header_value(&LOCATION) == Some("https://fixture.test/")
                    && set_cookies(&response) == ["access=token"]
        ));
    }
}
