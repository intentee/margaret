use margaret_cluster_fixture::margaret::routes::Routes;
use margaret_cluster_fixture::margaret::routes::props::GetNoteViewRouteProps;
use margaret_cluster_fixture::margaret::framework::url_path::path_parameter_value::PathParameterValue;

fn main() {
    let routes = Routes::from_origins("https://identity.test", "https://public.test")
        .expect("the routes build");

    drop(routes.public.get_note.forward_to(GetNoteViewRouteProps {
        note: PathParameterValue::new("viewed".to_string()).expect("the note fills a segment"),
    }));
}
