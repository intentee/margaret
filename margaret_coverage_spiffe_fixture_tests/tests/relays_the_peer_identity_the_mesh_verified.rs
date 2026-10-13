use margaret_coverage_spiffe_fixture::margaret::framework::console::command_outcome::CommandOutcome;
use margaret_coverage_spiffe_fixture_tests::served_mesh::ServedMesh;
use margaret_coverage_spiffe_fixture_tests::visit_mesh::visit_mesh;

#[tokio::test]
async fn relays_the_peer_identity_the_mesh_verified() {
    let outcome = visit_mesh(
        async |ServedMesh {
                   public,
                   public_client,
                   ..
               }: &ServedMesh| {
            assert_eq!(
                public_client
                    .get(format!("http://{public}/relay"))
                    .send()
                    .await
                    .expect("the relay answers")
                    .text()
                    .await
                    .expect("the relayed identity is read"),
                "spiffe://spiffe.test/workload"
            );
        },
    )
    .await;

    assert_eq!(outcome, CommandOutcome::Succeeded);
}
