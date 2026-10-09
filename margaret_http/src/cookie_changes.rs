use cookie::Cookie;

use crate::response_continuation::ResponseContinuation;

#[derive(Clone)]
pub struct CookieChanges {
    pub cookies: Vec<Cookie<'static>>,
}

impl CookieChanges {
    #[must_use]
    pub fn followed_by(&self, later: CookieChanges) -> CookieChanges {
        CookieChanges {
            cookies: self.cookies.iter().cloned().chain(later.cookies).collect(),
        }
    }

    #[must_use]
    pub fn precede(&self, continuation: ResponseContinuation) -> ResponseContinuation {
        match continuation {
            ResponseContinuation::Done(response) => {
                ResponseContinuation::Done(response.preceded_by_cookies(&self.cookies))
            }
            ResponseContinuation::Forward(forward) => {
                ResponseContinuation::Forward(forward.preceded_by(&self.cookies))
            }
            ResponseContinuation::Redirect(redirect) => ResponseContinuation::Done(
                redirect.into_response().preceded_by_cookies(&self.cookies),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use cookie::Cookie;
    use http::header::LOCATION;
    use http::header::SET_COOKIE;

    use super::CookieChanges;
    use crate::forward::Forward;
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
            .precede(ResponseContinuation::from(Response::text(200, ""))),
            ResponseContinuation::Done(response)
                if set_cookies(&response) == ["access=token", "secret=value"]
        ));
    }

    #[test]
    fn lists_its_changes_before_the_later_ones() {
        assert!(
            CookieChanges {
                cookies: vec![Cookie::new("session", "removed")],
            }
            .followed_by(CookieChanges {
                cookies: vec![Cookie::new("session", "started")],
            })
            .cookies
            .iter()
            .map(Cookie::value)
            .eq(["removed", "started"])
        );
    }

    #[test]
    fn precedes_the_cookies_the_response_already_sets() {
        assert!(matches!(
            CookieChanges {
                cookies: vec![Cookie::new("session", "removed")],
            }
            .precede(ResponseContinuation::from(
                Response::text(200, "").set_cookie(&Cookie::new("session", "started"))
            )),
            ResponseContinuation::Done(response)
                if set_cookies(&response) == ["session=removed", "session=started"]
        ));
    }

    #[test]
    fn precedes_the_cookies_a_forward_already_carries() {
        let carrying_the_start = CookieChanges {
            cookies: vec![Cookie::new("session", "started")],
        }
        .precede(ResponseContinuation::from(Forward::new(
            "target",
            HashMap::new(),
        )));

        assert!(matches!(
            CookieChanges {
                cookies: vec![Cookie::new("session", "removed")],
            }
            .precede(carrying_the_start),
            ResponseContinuation::Forward(forward)
                if forward
                    .carried_cookies()
                    .iter()
                    .map(Cookie::value)
                    .eq(["removed", "started"])
        ));
    }

    #[test]
    fn finishes_a_redirect_with_the_changed_cookies() {
        assert!(matches!(
            CookieChanges {
                cookies: vec![Cookie::new("access", "token")],
            }
            .precede(
                ResponseContinuation::from(Redirect::see_other("https://fixture.test/".to_string())),
            ),
            ResponseContinuation::Done(response)
                if response.status() == 303
                    && response.header_value(&LOCATION) == Some("https://fixture.test/")
                    && set_cookies(&response) == ["access=token"]
        ));
    }
}
