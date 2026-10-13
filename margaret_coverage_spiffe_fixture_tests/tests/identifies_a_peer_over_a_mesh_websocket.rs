use futures_util::SinkExt;
use futures_util::StreamExt;
use rustls::pki_types::ServerName;
use serde_json::Value;
use serde_json::json;
use tokio::net::TcpStream;
use tokio_rustls::TlsConnector;
use tokio_tungstenite::client_async;
use tokio_tungstenite::tungstenite::Message;

use margaret_coverage_spiffe_fixture::margaret::framework::console::command_outcome::CommandOutcome;
use margaret_coverage_spiffe_fixture_tests::served_mesh::ServedMesh;
use margaret_coverage_spiffe_fixture_tests::visit_mesh::visit_mesh;

#[tokio::test]
async fn identifies_a_peer_over_a_mesh_websocket() {
    let outcome = visit_mesh(async |ServedMesh { mesh, mesh_tls, .. }: &ServedMesh| {
        let secured = TlsConnector::from(mesh_tls.clone())
            .connect(
                ServerName::from(mesh.ip()),
                TcpStream::connect(mesh)
                    .await
                    .expect("the mesh accepts the connection"),
            )
            .await
            .expect("the mesh connection is mutually authenticated");
        let (mut talk, _) = client_async(format!("wss://{mesh}/talk"), secured)
            .await
            .expect("the verified peer talks on the mesh");

        talk.send(Message::text(r#"{"id":7,"method":"identify","params":{}}"#))
            .await
            .expect("the identification request is sent");

        let identity = talk
            .next()
            .await
            .expect("the mesh answers")
            .expect("the answer arrives intact");

        assert_eq!(
            serde_json::from_str::<Value>(identity.to_text().expect("the answer is text"))
                .expect("the answer is json"),
            json!({
                "done": true,
                "id": 7,
                "method": "identity",
                "result": { "spiffe_id": "spiffe://spiffe.test/workload" },
            })
        );
    })
    .await;

    assert_eq!(outcome, CommandOutcome::Succeeded);
}
