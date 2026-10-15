use http::header::LOCATION;

use margaret_example::constants::FEATURED_ARTICLE_ID;
use margaret_example::margaret::routes::Routes;
use margaret_example::routes::public::get_featured::GetFeatured;

#[test]
fn sees_the_visitor_other_to_the_featured_article() {
    let routes = Routes::from_origins("http://identity", "http://internal", "http://public")
        .expect("the routes build");
    let response = GetFeatured
        .respond(&routes)
        .expect("the responder succeeds")
        .into_response();

    assert_eq!(response.status(), 303);
    assert_eq!(
        response.header_value(&LOCATION),
        Some(format!("http://public/articles/{FEATURED_ARTICLE_ID}").as_str())
    );
}
