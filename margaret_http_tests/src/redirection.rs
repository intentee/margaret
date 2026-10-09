use std::collections::HashMap;

use http::header::LOCATION;
use url::Url;

use margaret_http::response::Response;

pub struct Redirection {
    pub location: Url,
    pub parameters: HashMap<String, String>,
}

impl Redirection {
    /// # Panics
    ///
    /// Panics when the response is not a see-other redirection to a url.
    #[must_use]
    pub fn of(response: &Response) -> Self {
        assert_eq!(response.status(), 303);

        let location = Url::parse(
            response
                .header_value(&LOCATION)
                .expect("the redirection names its location"),
        )
        .expect("the location is a url");
        let parameters = location.query_pairs().into_owned().collect();

        Self {
            location,
            parameters,
        }
    }

    /// # Panics
    ///
    /// Panics when the redirection lacks the parameter.
    #[must_use]
    pub fn parameter(&self, name: &str) -> &str {
        self.parameters
            .get(name)
            .expect("the redirection carries the parameter")
    }
}
