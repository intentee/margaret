use std::collections::HashMap;

use http::Method;

use margaret_http::requirement::Requirement;
use margaret_http::with_route_parameters::with_route_parameters;
use margaret_http_tests::fixture_request::FixtureRequest;
use margaret_http_tests::met_route_parameters::met_route_parameters;

#[tokio::test]
async fn hands_the_declared_route_parameters_in_declared_order() {
    let request = FixtureRequest {
        path_params: HashMap::from([
            ("article".to_string(), "7".to_string()),
            ("topic".to_string(), "rust".to_string()),
        ]),
        ..FixtureRequest::new(Method::GET, "/topics/rust/articles/7")
    }
    .into_request();

    assert!(matches!(
        with_route_parameters(&request, ["topic", "article"], Requirement::Unmet, met_route_parameters)
        .await,
        Requirement::Met([topic, article]) if topic == "rust" && article == "7"
    ));
}
