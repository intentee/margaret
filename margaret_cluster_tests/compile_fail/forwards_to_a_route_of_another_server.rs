use std::convert::Infallible;

use margaret::framework::http::responded::responded;
use margaret_cluster_fixture::margaret::routes::Routes;
use margaret_cluster_fixture::margaret::routes::props::GetNotesRouteProps;

fn main() {
    let routes = Routes::from_origins("https://identity.test", "https://public.test")
        .expect("the routes build");

    drop(responded::<Infallible, _>(Ok(routes
        .public
        .get_notes
        .forward_to(GetNotesRouteProps))));
}
