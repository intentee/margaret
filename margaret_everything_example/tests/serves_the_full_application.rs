use std::net::SocketAddr;

use bytes::Bytes;
use http::Method;
use http_body_util::BodyExt;
use http_body_util::Empty;
use hyper_util::rt::TokioIo;
use margaret_everything_example::margaret::http::server;
use tokio::net::TcpStream;

struct Reply {
    body: String,
    status: u16,
    trace: Option<String>,
}

async fn request(address: SocketAddr, method: Method, path: &str) -> Reply {
    let stream = TcpStream::connect(address)
        .await
        .expect("the client connects to the server");
    let (mut sender, connection) = hyper::client::conn::http1::handshake(TokioIo::new(stream))
        .await
        .expect("the client handshake succeeds");

    tokio::spawn(connection);

    let request = http::Request::builder()
        .method(method)
        .uri(path)
        .header("host", "example")
        .body(Empty::<Bytes>::new())
        .expect("a well-formed request");
    let response = sender
        .send_request(request)
        .await
        .expect("the server responds");
    let status = response.status().as_u16();
    let trace = response
        .headers()
        .get("x-traced")
        .map(|value| value.to_str().expect("an ascii trace header").to_string());
    let body = response
        .into_body()
        .collect()
        .await
        .expect("the response body is read")
        .to_bytes();

    Reply {
        body: String::from_utf8(body.to_vec()).expect("a utf-8 response body"),
        status,
        trace,
    }
}

#[tokio::test]
async fn serves_the_full_application() {
    let bound = server()
        .bind("127.0.0.1:0")
        .await
        .expect("the server binds to an ephemeral port");
    let address = bound.local_addr();

    tokio::spawn(bound.serve());

    let greeting = request(address, Method::GET, "/greeting").await;

    assert_eq!(greeting.status, 200);
    assert_eq!(greeting.body, "hello, margaret");

    let user = request(address, Method::GET, "/users/7").await;

    assert_eq!(user.status, 200);
    assert_eq!(user.body, "7");

    let health = request(address, Method::GET, "/health").await;

    assert_eq!(health.status, 200);
    assert_eq!(health.body, "ok");

    let report = request(address, Method::GET, "/admin/report").await;

    assert_eq!(report.status, 200);
    assert_eq!(report.body, "report");
    assert_eq!(report.trace.as_deref(), Some("logging,metrics"));

    let mutation = request(address, Method::POST, "/admin/report").await;

    assert_eq!(mutation.status, 403);

    let missing = request(address, Method::GET, "/nowhere").await;

    assert_eq!(missing.status, 404);

    let unsupported = request(address, Method::DELETE, "/health").await;

    assert_eq!(unsupported.status, 405);
}
