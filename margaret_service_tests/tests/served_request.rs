use std::sync::Arc;

use async_trait::async_trait;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio_util::sync::CancellationToken;

use margaret_http::bound_server::BoundServer;
use margaret_http::handler::Handler;
use margaret_http::method::Method;
use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_http::router_builder::RouterBuilder;
use margaret_http::server::Server;
use margaret_http::servers::Servers;
use margaret_http::upload_config::UploadConfig;

struct Accepts;

#[async_trait]
impl Handler for Accepts {
    async fn handle(&self, _request: &Request) -> ResponseContinuation {
        ResponseContinuation::Done(Response::text(200, "accepted"))
    }
}

async fn exchange(request: &[u8], upload_config: UploadConfig, close_write: bool) -> String {
    let router = RouterBuilder::empty()
        .route(Method::Post, "/submit", Arc::new(Accepts))
        .build();
    let servers = Arc::new(Servers::new(
        vec![Server::new(
            "public",
            "127.0.0.1:0".to_string(),
            "http://127.0.0.1",
            upload_config,
            router,
        )],
        Vec::new(),
    ));
    let bound = BoundServer::bind(servers, Arc::from("public"))
        .await
        .expect("the server binds");
    let address = bound.local_addr();
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
        UploadConfig::Enabled {
            directory: std::env::temp_dir(),
            max_size: 4,
        },
        false,
    )
    .await;

    assert!(response.contains(" 413 "));
}

#[tokio::test]
async fn rejects_a_truncated_form_body() {
    let response = exchange(
        b"POST /submit HTTP/1.1\r\nHost: test\r\nContent-Type: application/x-www-form-urlencoded\r\nContent-Length: 64\r\nConnection: close\r\n\r\nshort",
        UploadConfig::Disabled,
        true,
    )
    .await;

    assert!(response.contains(" 400 "));
}
