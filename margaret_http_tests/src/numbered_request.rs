use std::collections::HashMap;

use http::Method;

use margaret_http::request::Request;

use crate::fixture_request::FixtureRequest;

#[must_use]
pub fn numbered_request(number: &str) -> Request {
    FixtureRequest::new(Method::GET, "/numbers/42")
        .into_request()
        .with_path_params(HashMap::from([("number".to_string(), number.to_string())]))
}
