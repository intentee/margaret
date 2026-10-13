use margaret_codegen_fallible_roles_fixture::margaret::framework::console::command_outcome::CommandOutcome;
use margaret_codegen_tests::served_fallible_roles::ServedFallibleRoles;
use margaret_codegen_tests::visit_fallible_roles::visit_fallible_roles;

#[tokio::test]
async fn renders_a_view_through_aliased_results() {
    let outcome = visit_fallible_roles(
        async |ServedFallibleRoles { address, client }: &ServedFallibleRoles| {
            assert_eq!(
                client
                    .get(format!("http://{address}/"))
                    .send()
                    .await
                    .expect("the home answers")
                    .text()
                    .await
                    .expect("the home is read"),
                "<h1>home</h1>"
            );
        },
    )
    .await;

    assert_eq!(outcome, CommandOutcome::Succeeded);
}
