use std::collections::HashMap;

use cookie::Cookie;
use url::Url;

use margaret_http::response::Response;

fn header<'response>(response: &'response Response, name: &str) -> &'response str {
    response
        .headers()
        .iter()
        .find(|header| header.name == name)
        .map(|header| header.value.as_str())
        .expect("the redirect carries the header")
}

pub struct BegunSignIn {
    pub authorization: HashMap<String, String>,
    pub cookie: Cookie<'static>,
    pub status: u16,
}

impl BegunSignIn {
    #[must_use]
    /// # Panics
    ///
    /// Panics when the fixture it builds cannot be prepared.
    pub fn of(response: &Response) -> Self {
        Self {
            authorization: Url::parse(header(response, "location"))
                .expect("the location is a url")
                .query_pairs()
                .into_owned()
                .collect(),
            cookie: Cookie::parse(header(response, "set-cookie").to_string())
                .expect("the transaction cookie parses"),
            status: response.status(),
        }
    }

    /// # Panics
    ///
    /// Panics when the fixture it builds cannot be prepared.
    pub fn authorization_parameter(&self, name: &str) -> &str {
        self.authorization
            .get(name)
            .map(String::as_str)
            .expect("the authorization request carries the parameter")
    }

    #[must_use]
    pub fn cookie_pair(&self) -> String {
        format!("{}={}", self.cookie.name(), self.cookie.value())
    }
}
