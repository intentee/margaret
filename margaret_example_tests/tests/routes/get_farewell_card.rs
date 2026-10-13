use std::sync::Arc;

use margaret_example::config::app_name::AppName;
use margaret_example::margaret::routes::Routes;
use margaret_example::routes::public::get_farewell_card::GetFarewellCard;
use margaret_example_tests::views::views;

#[tokio::test]
async fn responds_ok_with_the_rendered_farewell_card_so_the_demo_route_serves_a_page() {
    let routes = Routes::from_origins("http://identity", "http://internal", "http://public")
        .expect("the routes build");
    let app_name = AppName::create().expect("the app name is constructed");
    let responder = GetFarewellCard::create(Arc::new(app_name));

    assert_eq!(
        responder
            .respond(&routes, &views())
            .expect("the responder succeeds")
            .status(),
        200
    );
}
