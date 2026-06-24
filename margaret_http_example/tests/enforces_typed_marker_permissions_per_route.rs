use std::net::SocketAddr;

use bytes::Bytes;
use http::Method;
use http_body_util::BodyExt;
use http_body_util::Empty;
use hyper_util::rt::TokioIo;
use margaret_http_example::margaret::http::server;
use tokio::net::TcpStream;

struct Reply {
    body: String,
    status: u16,
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
    let body = response
        .into_body()
        .collect()
        .await
        .expect("the response body is read")
        .to_bytes();

    Reply {
        body: String::from_utf8(body.to_vec()).expect("a utf-8 response body"),
        status,
    }
}

#[tokio::test]
async fn enforces_typed_marker_permissions_per_route() {
    let bound = server()
        .bind("127.0.0.1:0")
        .await
        .expect("the server binds to an ephemeral port");
    let address = bound.local_addr();

    tokio::spawn(bound.serve());

    let allowed = request(address, Method::GET, "/resource").await;

    assert_eq!(allowed.status, 200);
    assert_eq!(allowed.body, "resource");

    let denied = request(address, Method::GET, "/restricted").await;

    assert_eq!(denied.status, 403);

    let unguarded = request(address, Method::GET, "/open").await;

    assert_eq!(unguarded.status, 200);
    assert_eq!(unguarded.body, "open");

    let unsupported = request(address, Method::OPTIONS, "/open").await;

    assert_eq!(unsupported.status, 405);
}
