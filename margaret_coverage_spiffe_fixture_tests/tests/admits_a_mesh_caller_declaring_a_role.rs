use reqwest::StatusCode;

use margaret_coverage_spiffe_fixture::auth::headers::mesh_role::MESH_ROLE;
use margaret_coverage_spiffe_fixture::margaret::framework::console::command_outcome::CommandOutcome;
use margaret_coverage_spiffe_fixture_tests::served_mesh::ServedMesh;
use margaret_coverage_spiffe_fixture_tests::visit_mesh::visit_mesh;

#[tokio::test]
async fn admits_a_mesh_caller_declaring_a_role() {
    let outcome = visit_mesh(
        async |ServedMesh {
                   mesh, mesh_client, ..
               }: &ServedMesh| {
            let response = mesh_client
                .get(format!("https://{mesh}/caller"))
                .header(MESH_ROLE, "auditor")
                .send()
                .await
                .expect("the mesh answers the caller");

            assert_eq!(response.status(), StatusCode::OK);
            assert_eq!(
                response.text().await.expect("the caller is read"),
                "auditor spiffe://spiffe.test/workload"
            );
        },
    )
    .await;

    assert_eq!(outcome, CommandOutcome::Succeeded);
}
