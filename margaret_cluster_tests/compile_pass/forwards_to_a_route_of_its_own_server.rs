use margaret::framework::http::forward::Forward;
use margaret::framework::http::responded::responded;
use margaret_cluster_fixture::margaret::routes::Routes;
use margaret_cluster_fixture::margaret::routes::forward_targets::PublicForwardTarget;
use margaret_cluster_fixture::margaret::routes::props::GetNoteRouteProps;
use margaret_cluster_fixture::margaret::framework::url_path::path_parameter_value::PathParameterValue;

fn forwarded(routes: &Routes) -> Forward<PublicForwardTarget> {
    routes.public.get_note.forward_to(GetNoteRouteProps {
        note: PathParameterValue::new("note".to_string()).expect("the note fills a segment"),
    })
}

fn main() {
    let routes = Routes::from_origins("https://identity.test", "https://public.test")
        .expect("the routes build");

    drop(responded::<PublicForwardTarget, _>(Ok(forwarded(&routes))));
}
