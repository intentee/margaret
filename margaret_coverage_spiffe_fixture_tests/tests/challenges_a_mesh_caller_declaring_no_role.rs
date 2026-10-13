use reqwest::StatusCode;

use margaret_coverage_spiffe_fixture::margaret::framework::console::command_outcome::CommandOutcome;
use margaret_coverage_spiffe_fixture_tests::served_mesh::ServedMesh;
use margaret_coverage_spiffe_fixture_tests::visit_mesh::visit_mesh;

#[tokio::test]
async fn challenges_a_mesh_caller_declaring_no_role() {
    let outcome = visit_mesh(
        async |ServedMesh {
                   mesh, mesh_client, ..
               }: &ServedMesh| {
            assert_eq!(
                mesh_client
                    .get(format!("https://{mesh}/caller"))
                    .send()
                    .await
                    .expect("the mesh answers the caller")
                    .status(),
                StatusCode::UNAUTHORIZED
            );
        },
    )
    .await;

    assert_eq!(outcome, CommandOutcome::Succeeded);
}
