use margaret::framework::websocket_session::web_socket_session_creation_outcome::WebSocketSessionCreationOutcome;
use margaret_cluster_fixture::margaret::routes::Routes;
use margaret_cluster_fixture::margaret::routes::props::GetNotesRouteProps;

fn main() {
    let routes = Routes::from_origins("https://identity.test", "https://public.test")
        .expect("the routes build");

    drop(WebSocketSessionCreationOutcome::<()>::Interrupted(
        routes.public.get_notes.forward_to(GetNotesRouteProps),
    ));
}
