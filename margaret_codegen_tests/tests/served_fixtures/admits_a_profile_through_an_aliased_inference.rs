use margaret_codegen_fallible_roles_fixture::margaret::framework::console::command_outcome::CommandOutcome;
use margaret_codegen_tests::served_fallible_roles::ServedFallibleRoles;
use margaret_codegen_tests::visit_fallible_roles::visit_fallible_roles;

#[tokio::test]
async fn admits_a_profile_through_an_aliased_inference() {
    let outcome = visit_fallible_roles(
        async |ServedFallibleRoles { address, client }: &ServedFallibleRoles| {
            assert_eq!(
                client
                    .get(format!("http://{address}/profile?user=ada"))
                    .send()
                    .await
                    .expect("the profile answers")
                    .text()
                    .await
                    .expect("the profile is read"),
                "ada"
            );
        },
    )
    .await;

    assert_eq!(outcome, CommandOutcome::Succeeded);
}
