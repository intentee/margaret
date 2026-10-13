use std::sync::Arc;

use rustls::ClientConfig;
use rustls::crypto::aws_lc_rs;
use rustls::pki_types::ServerName;
use tokio::net::TcpStream;
use tokio_rustls::TlsConnector;
use tokio_rustls::client::TlsStream;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::client_async;
use url::Url;

use crate::chat_path::CHAT_PATH;
use crate::cluster::Cluster;

/// # Panics
///
/// Panics when the chat session cannot be opened.
pub async fn chat_socket(cluster: &Cluster, public: &Url) -> WebSocketStream<TlsStream<TcpStream>> {
    let host = public.host_str().expect("the public URL names a host");
    let stream = TcpStream::connect((
        host,
        public
            .port_or_known_default()
            .expect("the public URL names a port"),
    ))
    .await
    .expect("the chat connection opens");
    let connector = TlsConnector::from(Arc::new(
        ClientConfig::builder_with_provider(Arc::new(aws_lc_rs::default_provider()))
            .with_safe_default_protocol_versions()
            .expect("the default provider supports the safe protocol versions")
            .with_root_certificates(cluster.tls.certificate_authority.root_store())
            .with_no_client_auth(),
    ));
    let secured = connector
        .connect(
            ServerName::try_from(host.to_string()).expect("the host is a server name"),
            stream,
        )
        .await
        .expect("the chat connection is secured");
    let mut chat = public.join(CHAT_PATH).expect("the chat URL joins");

    chat.set_scheme("wss")
        .expect("the chat URL takes the secure scheme");

    client_async(chat.as_str(), secured)
        .await
        .expect("the chat session opens")
        .0
}
