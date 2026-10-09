use tokio_postgres::Client;
use tokio_postgres::NoTls;
use url::Url;

pub(crate) async fn connected_client(url: &Url) -> Client {
    let (client, connection) = tokio_postgres::connect(url.as_str(), NoTls)
        .await
        .expect("the shared test cluster accepts the connection");

    tokio::spawn(async move {
        connection.await.expect("the connection closes cleanly");
    });

    client
}
