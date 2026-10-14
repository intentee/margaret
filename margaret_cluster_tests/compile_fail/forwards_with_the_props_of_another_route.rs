use margaret_cluster_fixture::margaret::routes::Routes;
use margaret_cluster_fixture::margaret::routes::props::GetNoteViewRouteProps;

fn main() {
    let routes = Routes::from_origins("https://identity.test", "https://public.test")
        .expect("the routes build");

    drop(routes.public.get_note.forward_to(GetNoteViewRouteProps {
        note: "viewed".to_string(),
    }));
}
