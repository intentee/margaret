use http::header::LOCATION;

use margaret_example::margaret::routes::Routes;
use margaret_example::routes::public::get_welcome::GetWelcome;

#[test]
fn sees_the_signed_out_visitor_other_to_the_greeting() {
    let routes = Routes::from_origins("http://identity", "http://internal", "http://public")
        .expect("the routes build");
    let response = GetWelcome
        .respond(&routes)
        .expect("the responder succeeds")
        .into_response();

    assert_eq!(response.status(), 303);
    assert_eq!(
        response.header_value(&LOCATION),
        Some("http://public/greeting")
    );
}
