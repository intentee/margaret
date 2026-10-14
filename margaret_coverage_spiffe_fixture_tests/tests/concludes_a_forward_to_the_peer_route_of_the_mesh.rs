use std::sync::Arc;

use async_trait::async_trait;
use reqwest::Method;

use margaret_coverage_spiffe_fixture::margaret::framework::handler_error::handler_error::HandlerError;
use margaret_coverage_spiffe_fixture::margaret::framework::http::entered_responders::EnteredResponders;
use margaret_coverage_spiffe_fixture::margaret::framework::http::forward_scope::ForwardScope;
use margaret_coverage_spiffe_fixture::margaret::framework::http::head_handler::HeadHandler;
use margaret_coverage_spiffe_fixture::margaret::framework::http::request::Request;
use margaret_coverage_spiffe_fixture::margaret::framework::http::response::Response;
use margaret_coverage_spiffe_fixture::margaret::framework::http::response_continuation::ResponseContinuation;
use margaret_coverage_spiffe_fixture::margaret::routes::Routes;
use margaret_coverage_spiffe_fixture::margaret::routes::forward_targets::MeshForwardTarget;
use margaret_coverage_spiffe_fixture::margaret::routes::forward_targets::MeshForwardTargets;
use margaret_coverage_spiffe_fixture::margaret::routes::props::GetPeerRouteProps;
use margaret_http_tests::fixture_request::FixtureRequest;

const PEER_ANSWERED: u16 = 203;

struct AnswersThePeer;

#[async_trait]
impl HeadHandler<MeshForwardTarget> for AnswersThePeer {
    async fn handle(
        &self,
        _request: &Request,
    ) -> Result<ResponseContinuation<MeshForwardTarget>, HandlerError> {
        Ok(ResponseContinuation::from(Response::text(
            PEER_ANSWERED,
            "peer",
        )))
    }
}

#[tokio::test]
async fn concludes_a_forward_to_the_peer_route_of_the_mesh() {
    let routes = Routes::from_origins("https://mesh.test", "https://public.test")
        .expect("the routes of the mesh build");
    let response = MeshForwardTarget::conclude(
        &MeshForwardTargets {
            get_peer: Arc::new(AnswersThePeer),
        },
        EnteredResponders::default(),
        FixtureRequest::new(Method::GET, "/").into_request(),
        ResponseContinuation::from(routes.mesh.get_peer.forward_to(GetPeerRouteProps)),
    )
    .await
    .expect("the forward concludes");

    assert_eq!(response.status(), PEER_ANSWERED);
}
