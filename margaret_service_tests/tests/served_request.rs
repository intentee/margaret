use std::sync::Arc;

use async_trait::async_trait;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio_util::sync::CancellationToken;

use margaret_cookie_jar::cookie_jar::CookieJar;
use margaret_http::body_limit::BodyLimit;
use margaret_http::bound_server::BoundServer;
use margaret_http::forward_targets::ForwardTargets;
use margaret_http::handler::Handler;
use margaret_http::method_handler::MethodHandler;
use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_http::route_entry::RouteEntry;
use margaret_http::router::Router;
use margaret_http::server::Server;
use margaret_http::server_registry::ServerRegistry;
use margaret_http::transport_config::TransportConfig;
use margaret_http::upload_config::UploadConfig;

struct Accepts;

#[async_trait]
impl Handler for Accepts {
    async fn handle(&self, _request: &Request, _cookie_jar: &CookieJar) -> ResponseContinuation {
        ResponseContinuation::Done(Response::text(200, "accepted"))
    }
}

async fn exchange(
    request: &[u8],
    body_limit: BodyLimit,
    upload_config: UploadConfig,
    close_write: bool,
) -> String {
    let router = Router::build(vec![
        RouteEntry::new(
            "/submit",
            vec![MethodHandler::new("POST", Arc::new(Accepts))],
        ),
        RouteEntry::new(
            "/search",
            vec![MethodHandler::new("QUERY", Arc::new(Accepts))],
        ),
    ])
    .expect("the route entries register cleanly");
    let server_registry = Arc::new(ServerRegistry::new(vec![Server::new(
        "public",
        "127.0.0.1:0".to_string(),
        TransportConfig::Plain,
        upload_config,
        body_limit,
        router,
    )]));
    let forward_targets = Arc::new(ForwardTargets::new(Vec::new()));
    let bound = BoundServer::bind(server_registry, forward_targets, Arc::from("public"))
        .await
        .expect("the server binds");
    let address = bound
        .local_addr()
        .expect("the bound listener reports its address");
    let cancellation_token = CancellationToken::new();
    let serving = tokio::spawn(bound.serve(cancellation_token.clone()));

    let mut stream = TcpStream::connect(address)
        .await
        .expect("the client connects");

    stream
        .write_all(request)
        .await
        .expect("the request is sent");

    if close_write {
        stream.shutdown().await.expect("the write half closes");
    }

    let mut response = Vec::new();

    stream
        .read_to_end(&mut response)
        .await
        .expect("the response is read");

    cancellation_token.cancel();
    serving.await.expect("the server task finishes");

    String::from_utf8_lossy(&response).into_owned()
}

#[tokio::test]
async fn accepts_a_form_body_within_the_limit() {
    let response = exchange(
        b"POST /submit HTTP/1.1\r\nHost: test\r\nContent-Type: application/x-www-form-urlencoded\r\nContent-Length: 7\r\nConnection: close\r\n\r\nname=hi",
        BodyLimit::default(),
        UploadConfig::Disabled,
        false,
    )
    .await;

    assert!(response.contains(" 200 "));
}

#[tokio::test]
async fn rejects_a_form_body_that_exceeds_the_limit() {
    let response = exchange(
        b"POST /submit HTTP/1.1\r\nHost: test\r\nContent-Type: application/x-www-form-urlencoded\r\nContent-Length: 7\r\nConnection: close\r\n\r\nname=hi",
        BodyLimit::new(4),
        UploadConfig::Disabled,
        false,
    )
    .await;

    assert!(response.contains(" 413 "));
}

#[tokio::test]
async fn rejects_a_truncated_form_body() {
    let response = exchange(
        b"POST /submit HTTP/1.1\r\nHost: test\r\nContent-Type: application/x-www-form-urlencoded\r\nContent-Length: 64\r\nConnection: close\r\n\r\nshort",
        BodyLimit::default(),
        UploadConfig::Disabled,
        true,
    )
    .await;

    assert!(response.contains(" 400 "));
}

#[tokio::test]
async fn dispatches_a_request_using_an_extension_verb() {
    let response = exchange(
        b"QUERY /search HTTP/1.1\r\nHost: test\r\nConnection: close\r\n\r\n",
        BodyLimit::default(),
        UploadConfig::Disabled,
        false,
    )
    .await;

    assert!(response.contains(" 200 "));
}

#[tokio::test]
async fn rejects_a_known_path_reached_with_an_unhandled_method() {
    let response = exchange(
        b"GET /submit HTTP/1.1\r\nHost: test\r\nConnection: close\r\n\r\n",
        BodyLimit::default(),
        UploadConfig::Disabled,
        false,
    )
    .await;

    assert!(response.contains(" 405 "));
}
