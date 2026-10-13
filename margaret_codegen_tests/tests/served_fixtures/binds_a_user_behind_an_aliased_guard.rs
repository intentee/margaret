use margaret_codegen_fallible_roles_fixture::margaret::framework::console::command_outcome::CommandOutcome;
use margaret_codegen_tests::served_fallible_roles::ServedFallibleRoles;
use margaret_codegen_tests::visit_fallible_roles::visit_fallible_roles;

#[tokio::test]
async fn binds_a_user_behind_an_aliased_guard() {
    let outcome = visit_fallible_roles(
        async |ServedFallibleRoles { address, client }: &ServedFallibleRoles| {
            assert_eq!(
                client
                    .get(format!("http://{address}/users/ada"))
                    .send()
                    .await
                    .expect("the user answers")
                    .text()
                    .await
                    .expect("the user is read"),
                "ada"
            );
        },
    )
    .await;

    assert_eq!(outcome, CommandOutcome::Succeeded);
}
