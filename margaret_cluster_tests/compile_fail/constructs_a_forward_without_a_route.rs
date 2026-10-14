use margaret::framework::http::forward::Forward;
use margaret_cluster_fixture::margaret::routes::forward_targets::PublicForwardTarget;
use margaret_cluster_fixture::margaret::routes::props::GetNotesRouteProps;

fn main() {
    drop(Forward {
        cookies: Vec::new(),
        target: PublicForwardTarget::GetNotes(GetNotesRouteProps),
    });
}
