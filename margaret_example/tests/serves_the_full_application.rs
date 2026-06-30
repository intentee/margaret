use bytes::Bytes;
use http::Method;
use http_body_util::BodyExt;
use http_body_util::Full;
use hyper::client::conn::http1::SendRequest;
use hyper_util::rt::TokioIo;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use margaret_example::margaret::container::Container;
use margaret_example::margaret::http::server_internal;
use margaret_example::margaret::http::server_public;

const FORM: &str = "application/x-www-form-urlencoded";

struct Reply {
    body: String,
    set_cookie: Option<String>,
    status: u16,
    trace: Option<String>,
}

struct TestServer {
    cancellation_token: CancellationToken,
    driving: JoinHandle<Result<(), hyper::Error>>,
    sender: SendRequest<Full<Bytes>>,
    serving: JoinHandle<()>,
}

impl TestServer {
    async fn start() -> Self {
        let container = Container::build();

        Self::serve(server_public(&container).await).await
    }

    async fn start_internal() -> Self {
        let container = Container::build();

        Self::serve(server_internal(&container).await).await
    }

    async fn serve(server: margaret_http::server::Server) -> Self {
        let bound = server
            .bind("127.0.0.1:0")
            .await
            .expect("the server binds to an ephemeral port");
        let address = bound.local_addr();
        let cancellation_token = CancellationToken::new();
        let serving = tokio::spawn(bound.serve(cancellation_token.clone()));
        let stream = TcpStream::connect(address)
            .await
            .expect("the client connects to the server");
        let (sender, connection) = hyper::client::conn::http1::handshake(TokioIo::new(stream))
            .await
            .expect("the client handshake succeeds");
        let driving = tokio::spawn(connection);

        Self {
            cancellation_token,
            driving,
            sender,
            serving,
        }
    }

    async fn send(
        &mut self,
        method: Method,
        path: &str,
        headers: &[(&str, &str)],
        body: &str,
    ) -> Reply {
        let mut builder = http::Request::builder()
            .method(method)
            .uri(path)
            .header("host", "example");

        for (name, value) in headers {
            builder = builder.header(*name, *value);
        }

        let request = builder
            .body(Full::new(Bytes::from(body.to_string())))
            .expect("a well-formed request");
        let response = self
            .sender
            .send_request(request)
            .await
            .expect("the server responds");
        let status = response.status().as_u16();
        let header = |name: &str| {
            response
                .headers()
                .get(name)
                .map(|value| value.to_str().expect("an ascii header value").to_string())
        };
        let trace = header("x-traced");
        let set_cookie = header("set-cookie");
        let body = response
            .into_body()
            .collect()
            .await
            .expect("the response body is read")
            .to_bytes();

        Reply {
            body: String::from_utf8(body.to_vec()).expect("a utf-8 response body"),
            set_cookie,
            status,
            trace,
        }
    }

    async fn login_as(&mut self, username: &str) -> String {
        let login = self
            .send(
                Method::POST,
                "/login",
                &[("content-type", FORM)],
                &format!("username={username}&password=resonance"),
            )
            .await;

        assert_eq!(login.status, 303);

        cookie_header(
            &login
                .set_cookie
                .expect("the login response sets a session cookie"),
        )
    }

    async fn shutdown(self) {
        let Self {
            cancellation_token,
            driving,
            sender,
            serving,
        } = self;

        drop(sender);
        driving
            .await
            .expect("the client connection task joins")
            .expect("the client connection drains cleanly");
        cancellation_token.cancel();
        serving
            .await
            .expect("the serving task joins and the server drains cleanly");
    }
}

fn cookie_header(set_cookie: &str) -> String {
    let parsed = margaret_http::cookie::Cookie::parse(set_cookie.to_string())
        .expect("a valid set-cookie header");

    format!("{}={}", parsed.name(), parsed.value())
}

#[tokio::test]
async fn isolates_routes_between_the_public_and_internal_servers() {
    let mut internal = TestServer::start_internal().await;

    let metrics = internal.send(Method::GET, "/metrics", &[], "").await;

    assert_eq!(metrics.status, 200);
    assert!(metrics.body.starts_with("sweeps="));

    let public_route_on_internal = internal.send(Method::GET, "/health", &[], "").await;

    assert_eq!(public_route_on_internal.status, 404);

    internal.shutdown().await;

    let mut public = TestServer::start().await;

    let internal_route_on_public = public.send(Method::GET, "/metrics", &[], "").await;

    assert_eq!(internal_route_on_public.status, 404);

    public.shutdown().await;
}

#[tokio::test]
async fn serves_public_and_dependency_injected_routes() {
    let mut server = TestServer::start().await;

    let greeting = server.send(Method::GET, "/greeting", &[], "").await;

    assert_eq!(greeting.status, 200);
    assert_eq!(greeting.body, "hello, margaret");

    let health = server.send(Method::GET, "/health", &[], "").await;

    assert_eq!(health.status, 200);
    assert_eq!(health.body, "margaret");

    let revision = server.send(Method::GET, "/revision", &[], "").await;

    assert_eq!(revision.status, 200);
    assert_eq!(revision.body, "1.0.0");

    let missing = server.send(Method::GET, "/nowhere", &[], "").await;

    assert_eq!(missing.status, 404);

    let unsupported = server.send(Method::OPTIONS, "/health", &[], "").await;

    assert_eq!(unsupported.status, 405);

    server.shutdown().await;
}

#[tokio::test]
async fn establishes_and_clears_a_session_through_login_and_logout() {
    let mut server = TestServer::start().await;

    let login_page = server.send(Method::GET, "/login", &[], "").await;

    assert_eq!(login_page.status, 200);
    assert!(login_page.body.contains("<form"));

    let wrong_password = server
        .send(
            Method::POST,
            "/login",
            &[("content-type", FORM)],
            "username=admin&password=wrong",
        )
        .await;

    assert_eq!(wrong_password.status, 401);

    let missing_field = server
        .send(
            Method::POST,
            "/login",
            &[("content-type", FORM)],
            "username=admin",
        )
        .await;

    assert_eq!(missing_field.status, 401);

    let unknown_user = server
        .send(
            Method::POST,
            "/login",
            &[("content-type", FORM)],
            "username=nobody&password=resonance",
        )
        .await;

    assert_eq!(unknown_user.status, 401);

    let cookie = server.login_as("admin").await;

    let account = server
        .send(Method::GET, "/account", &[("cookie", &cookie)], "")
        .await;

    assert_eq!(account.status, 200);
    assert_eq!(account.body, "account: Ada #1");

    let anonymous_account = server.send(Method::GET, "/account", &[], "").await;

    assert_eq!(anonymous_account.status, 403);

    let forged_account = server
        .send(
            Method::GET,
            "/account",
            &[("cookie", "access_token=not-a-valid-token")],
            "",
        )
        .await;

    assert_eq!(forged_account.status, 403);

    let relogin = server
        .send(
            Method::POST,
            "/login",
            &[("content-type", FORM), ("cookie", &cookie)],
            "username=admin&password=resonance",
        )
        .await;

    assert_eq!(relogin.status, 303);

    let logout = server
        .send(Method::POST, "/logout", &[("cookie", &cookie)], "")
        .await;

    assert_eq!(logout.status, 303);
    assert!(
        logout
            .set_cookie
            .expect("logout returns a set-cookie that clears the access token")
            .contains("Max-Age=0"),
        "stateless logout clears the access token cookie on the client"
    );

    let anonymous_logout = server.send(Method::POST, "/logout", &[], "").await;

    assert_eq!(anonymous_logout.status, 303);

    server.shutdown().await;
}

#[tokio::test]
async fn authorizes_anonymous_article_access() {
    let mut server = TestServer::start().await;

    let published = server.send(Method::GET, "/articles/100", &[], "").await;

    assert_eq!(published.status, 200);
    assert!(published.body.contains("a guest reads"));

    let draft = server.send(Method::GET, "/articles/101", &[], "").await;

    assert_eq!(draft.status, 403);

    let unknown = server.send(Method::GET, "/articles/999", &[], "").await;

    assert_eq!(unknown.status, 404);

    let mutation = server.send(Method::DELETE, "/articles/100", &[], "").await;

    assert_eq!(mutation.status, 403);

    server.shutdown().await;
}

#[tokio::test]
async fn authorizes_a_member_against_their_own_and_foreign_articles() {
    let mut server = TestServer::start().await;
    let member = server.login_as("member").await;

    let published = server
        .send(Method::GET, "/articles/100", &[("cookie", &member)], "")
        .await;

    assert_eq!(published.status, 200);
    assert!(published.body.contains("Milo reads"));

    let own_draft = server
        .send(Method::GET, "/articles/101", &[("cookie", &member)], "")
        .await;

    assert_eq!(own_draft.status, 200);

    let foreign_draft = server
        .send(Method::GET, "/articles/102", &[("cookie", &member)], "")
        .await;

    assert_eq!(foreign_draft.status, 403);

    let own_update = server
        .send(Method::PATCH, "/articles/101", &[("cookie", &member)], "")
        .await;

    assert_eq!(own_update.status, 200);
    assert_eq!(own_update.body, "updated \"Milo's draft\"");

    let foreign_update = server
        .send(Method::PATCH, "/articles/102", &[("cookie", &member)], "")
        .await;

    assert_eq!(foreign_update.status, 403);

    let own_delete = server
        .send(Method::DELETE, "/articles/101", &[("cookie", &member)], "")
        .await;

    assert_eq!(own_delete.status, 200);

    let update_missing = server
        .send(Method::PATCH, "/articles/404", &[("cookie", &member)], "")
        .await;

    assert_eq!(update_missing.status, 404);

    let delete_missing = server
        .send(Method::DELETE, "/articles/404", &[("cookie", &member)], "")
        .await;

    assert_eq!(delete_missing.status, 404);

    server.shutdown().await;
}

#[tokio::test]
async fn authorizes_a_moderator_and_an_administrator_over_foreign_articles() {
    let mut server = TestServer::start().await;
    let moderator = server.login_as("moderator").await;
    let admin = server.login_as("admin").await;

    let moderator_reads_draft = server
        .send(Method::GET, "/articles/101", &[("cookie", &moderator)], "")
        .await;

    assert_eq!(moderator_reads_draft.status, 200);

    let moderator_updates_foreign = server
        .send(
            Method::PATCH,
            "/articles/101",
            &[("cookie", &moderator)],
            "",
        )
        .await;

    assert_eq!(moderator_updates_foreign.status, 403);

    let admin_reads_draft = server
        .send(Method::GET, "/articles/101", &[("cookie", &admin)], "")
        .await;

    assert_eq!(admin_reads_draft.status, 200);

    let admin_updates_foreign = server
        .send(Method::PATCH, "/articles/102", &[("cookie", &admin)], "")
        .await;

    assert_eq!(admin_updates_foreign.status, 200);

    let admin_deletes_foreign = server
        .send(Method::DELETE, "/articles/100", &[("cookie", &admin)], "")
        .await;

    assert_eq!(admin_deletes_foreign.status, 200);

    server.shutdown().await;
}

#[tokio::test]
async fn exercises_the_manual_gatekeeper_and_site_action_guard() {
    let mut server = TestServer::start().await;
    let member = server.login_as("member").await;
    let admin = server.login_as("admin").await;

    let anonymous_overview = server.send(Method::GET, "/articles", &[], "").await;

    assert_eq!(anonymous_overview.status, 200);
    assert_eq!(
        anonymous_overview.body,
        "manage_users=false;delete_all=false"
    );

    let member_overview = server
        .send(Method::GET, "/articles", &[("cookie", &member)], "")
        .await;

    assert_eq!(member_overview.status, 200);
    assert_eq!(member_overview.body, "manage_users=false;delete_all=false");

    let admin_overview = server
        .send(Method::GET, "/articles", &[("cookie", &admin)], "")
        .await;

    assert_eq!(admin_overview.status, 200);
    assert_eq!(admin_overview.body, "manage_users=true;delete_all=true");

    let anonymous_admin = server.send(Method::GET, "/admin/users", &[], "").await;

    assert_eq!(anonymous_admin.status, 403);

    let member_admin = server
        .send(Method::GET, "/admin/users", &[("cookie", &member)], "")
        .await;

    assert_eq!(member_admin.status, 403);

    let admin_admin = server
        .send(Method::GET, "/admin/users", &[("cookie", &admin)], "")
        .await;

    assert_eq!(admin_admin.status, 200);
    assert_eq!(
        admin_admin.body,
        "users=Ada,Mona,Milo;read=true;update=true;delete=true;read_all=true;update_all=true;delete_all=true"
    );
    assert_eq!(admin_admin.trace.as_deref(), Some("logging,metrics"));

    server.shutdown().await;
}

#[tokio::test]
async fn rejects_a_request_whose_body_cannot_be_read() {
    let container = Container::build();
    let bound = server_public(&container)
        .await
        .bind("127.0.0.1:0")
        .await
        .expect("the server binds to an ephemeral port");
    let address = bound.local_addr();
    let cancellation_token = CancellationToken::new();
    let serving = tokio::spawn(bound.serve(cancellation_token.clone()));

    let mut stream = TcpStream::connect(address)
        .await
        .expect("the client connects to the server");
    stream
        .write_all(
            b"POST /login HTTP/1.1\r\nHost: example\r\nTransfer-Encoding: chunked\r\n\r\nnonsense\r\n",
        )
        .await
        .expect("the malformed request is written");

    let mut buffer = [0u8; 64];
    let read = stream
        .read(&mut buffer)
        .await
        .expect("the server responds before closing the connection");
    let response = String::from_utf8_lossy(&buffer[..read]);

    assert!(response.contains("400"));

    drop(stream);
    cancellation_token.cancel();
    serving
        .await
        .expect("the serving task joins and the server drains cleanly");
}

#[tokio::test]
async fn forwards_to_a_responder_and_re_runs_its_middleware() {
    let mut server = TestServer::start().await;

    let welcome = server.send(Method::GET, "/welcome", &[], "").await;

    assert_eq!(welcome.status, 200);
    assert_eq!(welcome.body, "hello, margaret");
    assert!(
        welcome.trace.is_some(),
        "the forwarded responder's tracing middleware re-runs after the forward"
    );

    server.shutdown().await;
}

#[tokio::test]
async fn renders_distinct_views_through_one_marker_interceptor() {
    let mut server = TestServer::start().await;

    let greeting = server.send(Method::GET, "/greeting-card", &[], "").await;

    assert_eq!(greeting.status, 200);
    assert_eq!(greeting.body, "<main>hello, margaret</main>");

    let farewell = server.send(Method::GET, "/farewell-card", &[], "").await;

    assert_eq!(farewell.status, 200);
    assert_eq!(farewell.body, "<main>goodbye, margaret</main>");

    server.shutdown().await;
}

#[tokio::test]
async fn creates_reads_updates_and_deletes_an_article_in_memory() {
    let mut server = TestServer::start().await;
    let member = server.login_as("member").await;

    let anonymous = server
        .send(
            Method::POST,
            "/articles",
            &[("content-type", FORM)],
            "title=Ghost&body=Post",
        )
        .await;

    assert_eq!(anonymous.status, 403);

    let incomplete = server
        .send(
            Method::POST,
            "/articles",
            &[("content-type", FORM), ("cookie", &member)],
            "title=Only",
        )
        .await;

    assert_eq!(incomplete.status, 422);

    let created = server
        .send(
            Method::POST,
            "/articles",
            &[("content-type", FORM), ("cookie", &member)],
            "title=My+Post&body=Hello",
        )
        .await;

    assert_eq!(created.status, 201);
    assert_eq!(created.body, "created \"My Post\"");

    let read = server
        .send(Method::GET, "/articles/103", &[("cookie", &member)], "")
        .await;

    assert_eq!(read.status, 200);
    assert!(read.body.contains("My Post"));
    assert!(read.body.contains("Hello"));

    let updated = server
        .send(
            Method::PATCH,
            "/articles/103",
            &[("content-type", FORM), ("cookie", &member)],
            "title=Edited&body=World",
        )
        .await;

    assert_eq!(updated.status, 200);
    assert_eq!(updated.body, "updated \"Edited\"");

    let reread = server
        .send(Method::GET, "/articles/103", &[("cookie", &member)], "")
        .await;

    assert!(reread.body.contains("Edited"));
    assert!(reread.body.contains("World"));

    let deleted = server
        .send(Method::DELETE, "/articles/103", &[("cookie", &member)], "")
        .await;

    assert_eq!(deleted.status, 200);

    let gone = server
        .send(Method::GET, "/articles/103", &[("cookie", &member)], "")
        .await;

    assert_eq!(gone.status, 404);

    server.shutdown().await;
}
