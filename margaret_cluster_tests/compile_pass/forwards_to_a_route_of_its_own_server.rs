use margaret::framework::http::forward::Forward;
use margaret::framework::http::responded::responded;
use margaret_cluster_fixture::margaret::routes::Routes;
use margaret_cluster_fixture::margaret::routes::forward_targets::PublicForwardTarget;
use margaret_cluster_fixture::margaret::routes::props::GetNoteRouteProps;

fn forwarded(routes: &Routes) -> Forward<PublicForwardTarget> {
    routes.public.get_note.forward_to(GetNoteRouteProps {
        note: "note".to_string(),
    })
}

fn main() {
    let routes = Routes::from_origins("https://identity.test", "https://public.test")
        .expect("the routes build");

    drop(responded::<PublicForwardTarget, _>(Ok(forwarded(&routes))));
}
