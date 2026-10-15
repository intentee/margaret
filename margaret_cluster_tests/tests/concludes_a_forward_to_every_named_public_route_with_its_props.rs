use std::sync::Arc;

use async_trait::async_trait;
use reqwest::Method;
use reqwest::header::HeaderName;

use margaret_cluster_fixture::margaret::framework::handler_error::handler_error::HandlerError;
use margaret_cluster_fixture::margaret::framework::http::entered_responders::EnteredResponders;
use margaret_cluster_fixture::margaret::framework::http::forward::Forward;
use margaret_cluster_fixture::margaret::framework::http::forward_scope::ForwardScope;
use margaret_cluster_fixture::margaret::framework::http::head_handler::HeadHandler;
use margaret_cluster_fixture::margaret::framework::http::request::Request;
use margaret_cluster_fixture::margaret::framework::http::response::Response;
use margaret_cluster_fixture::margaret::framework::http::response_continuation::ResponseContinuation;
use margaret_cluster_fixture::margaret::routes::Routes;
use margaret_cluster_fixture::margaret::routes::forward_targets::PublicForwardTarget;
use margaret_cluster_fixture::margaret::routes::forward_targets::PublicForwardTargets;
use margaret_cluster_fixture::margaret::routes::props::GetAssetRouteProps;
use margaret_cluster_fixture::margaret::routes::props::GetExternalRouteProps;
use margaret_cluster_fixture::margaret::routes::props::GetForwardedNoteRouteProps;
use margaret_cluster_fixture::margaret::routes::props::GetHealthRouteProps;
use margaret_cluster_fixture::margaret::routes::props::GetMessagesRouteProps;
use margaret_cluster_fixture::margaret::routes::props::GetNoteRouteProps;
use margaret_cluster_fixture::margaret::routes::props::GetNoteViewRouteProps;
use margaret_cluster_fixture::margaret::routes::props::GetNotesCallerRouteProps;
use margaret_cluster_fixture::margaret::routes::props::GetNotesRouteProps;
use margaret_cluster_fixture::margaret::routes::props::GetProfileRouteProps;
use margaret_cluster_fixture::margaret::routes::props::GetSignInCallbackRouteProps;
use margaret_cluster_fixture::margaret::routes::props::GetSignInRouteProps;
use margaret_cluster_fixture::margaret::routes::props::GetUploadRouteProps;
use margaret_cluster_tests::path_parameter::path_parameter;
use margaret_cluster_tests::path_tail::path_tail;
use margaret_http_tests::fixture_request::FixtureRequest;

const ROUTE: HeaderName = HeaderName::from_static("x-route");
const PARAMETERS: HeaderName = HeaderName::from_static("x-parameters");

struct EchoesItsRoute {
    placeholders: &'static [&'static str],
    route: &'static str,
}

#[async_trait]
impl HeadHandler<PublicForwardTarget> for EchoesItsRoute {
    async fn handle(
        &self,
        request: &Request,
    ) -> Result<ResponseContinuation<PublicForwardTarget>, HandlerError> {
        let parameters = self
            .placeholders
            .iter()
            .map(|placeholder| {
                format!(
                    "{placeholder}={}",
                    request
                        .path_param(placeholder)
                        .expect("the forward hands over every placeholder")
                )
            })
            .collect::<Vec<String>>()
            .join(",");

        Ok(ResponseContinuation::from(
            Response::text(200, "")
                .header(ROUTE.as_str(), self.route)
                .header(PARAMETERS.as_str(), parameters),
        ))
    }
}

fn echoes(
    route: &'static str,
    placeholders: &'static [&'static str],
) -> Arc<dyn HeadHandler<PublicForwardTarget>> {
    Arc::new(EchoesItsRoute {
        placeholders,
        route,
    })
}

fn targets() -> PublicForwardTargets {
    PublicForwardTargets {
        get_asset: echoes("get_asset", &["asset_path"]),
        get_external: echoes("get_external", &[]),
        get_forwarded_note: echoes("get_forwarded_note", &["note_id"]),
        get_health: echoes("get_health", &[]),
        get_messages: echoes("get_messages", &[]),
        get_note: echoes("get_note", &["note"]),
        get_note_view: echoes("get_note_view", &["note"]),
        get_notes: echoes("get_notes", &[]),
        get_notes_caller: echoes("get_notes_caller", &[]),
        get_profile: echoes("get_profile", &[]),
        get_sign_in: echoes("get_sign_in", &[]),
        get_sign_in_callback: echoes("get_sign_in_callback", &[]),
        get_upload: echoes("get_upload", &["upload"]),
    }
}

async fn concluded(
    targets: &PublicForwardTargets,
    forward: Forward<PublicForwardTarget>,
) -> String {
    let response = PublicForwardTarget::conclude(
        targets,
        EnteredResponders::default(),
        FixtureRequest::new(Method::GET, "/").into_request(),
        ResponseContinuation::from(forward),
    )
    .await
    .expect("the forward concludes");

    format!(
        "{} {}",
        response
            .header_value(&ROUTE)
            .expect("the target names its route"),
        response
            .header_value(&PARAMETERS)
            .expect("the target names its parameters")
    )
}

#[tokio::test]
async fn concludes_a_forward_to_every_named_public_route_with_its_props() {
    let routes = Routes::from_origins("https://identity.test", "https://public.test")
        .expect("the routes of the cluster build");
    let public = &routes.public;
    let targets = targets();
    let mut concluded_forwards = Vec::new();

    for forward in [
        public.get_asset.forward_to(GetAssetRouteProps {
            asset_path: path_tail("cluster.css".to_string()),
        }),
        public.get_external.forward_to(GetExternalRouteProps),
        public
            .get_forwarded_note
            .forward_to(GetForwardedNoteRouteProps {
                note_id: path_parameter("forwarded".to_string()),
            }),
        public.get_health.forward_to(GetHealthRouteProps),
        public.get_messages.forward_to(GetMessagesRouteProps),
        public.get_note.forward_to(GetNoteRouteProps {
            note: path_parameter("note".to_string()),
        }),
        public.get_note_view.forward_to(GetNoteViewRouteProps {
            note: path_parameter("viewed".to_string()),
        }),
        public.get_notes.forward_to(GetNotesRouteProps),
        public.get_notes_caller.forward_to(GetNotesCallerRouteProps),
        public.get_profile.forward_to(GetProfileRouteProps),
        public.get_sign_in.forward_to(GetSignInRouteProps),
        public
            .get_sign_in_callback
            .forward_to(GetSignInCallbackRouteProps),
        public.get_upload.forward_to(GetUploadRouteProps {
            upload: path_parameter("upload".to_string()),
        }),
    ] {
        concluded_forwards.push(concluded(&targets, forward).await);
    }

    assert_eq!(
        concluded_forwards,
        [
            "get_asset asset_path=cluster.css",
            "get_external ",
            "get_forwarded_note note_id=forwarded",
            "get_health ",
            "get_messages ",
            "get_note note=note",
            "get_note_view note=viewed",
            "get_notes ",
            "get_notes_caller ",
            "get_profile ",
            "get_sign_in ",
            "get_sign_in_callback ",
            "get_upload upload=upload",
        ]
    );
}
