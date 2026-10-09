use cookie::Cookie;
use http::header::SET_COOKIE;

use margaret_http::response::Response;
use margaret_http_tests::redirection::Redirection;

pub struct BegunSignIn {
    pub authorization: Redirection,
    pub cookie: Cookie<'static>,
}

impl BegunSignIn {
    /// # Panics
    ///
    /// Panics when the response is not a redirect carrying a transaction cookie.
    #[must_use]
    pub fn of(response: &Response) -> Self {
        Self {
            authorization: Redirection::of(response),
            cookie: Cookie::parse(
                response
                    .header_value(&SET_COOKIE)
                    .expect("the redirect carries the transaction cookie")
                    .to_string(),
            )
            .expect("the transaction cookie parses"),
        }
    }

    /// # Panics
    ///
    /// Panics when the authorization request lacks the parameter.
    #[must_use]
    pub fn authorization_parameter(&self, name: &str) -> &str {
        self.authorization.parameter(name)
    }

    #[must_use]
    pub fn cookie_pair(&self) -> String {
        format!("{}={}", self.cookie.name(), self.cookie.value())
    }
}
