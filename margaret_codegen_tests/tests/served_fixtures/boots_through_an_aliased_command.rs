use margaret_codegen_fallible_roles_fixture::margaret::framework::console::command_outcome::CommandOutcome;
use margaret_codegen_fallible_roles_fixture::margaret::run::run;

#[tokio::test]
async fn boots_through_an_aliased_command() {
    assert_eq!(
        run(["margaret_codegen_fallible_roles_fixture", "boot"]).await,
        CommandOutcome::Succeeded
    );
}
