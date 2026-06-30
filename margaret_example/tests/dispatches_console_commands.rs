use margaret_console::command_outcome::CommandOutcome;
use margaret_example::margaret::console::run;
use margaret_example::margaret::container::Container;

#[tokio::test]
async fn dispatches_console_commands() {
    let container = Container::build();

    assert_eq!(
        run(
            &container,
            ["app", "greet", "Margaret", "--salutation", "Hi", "--loud"]
        )
        .await,
        CommandOutcome::Succeeded
    );

    assert_eq!(
        run(
            &container,
            [
                "app",
                "serve",
                "--public-addr",
                "this is not an address",
                "--internal-addr",
                "127.0.0.1:0",
            ]
        )
        .await,
        CommandOutcome::Failed
    );

    assert_eq!(
        run(&container, ["app", "--unknown-flag"]).await,
        CommandOutcome::Failed
    );

    assert_eq!(run(&container, ["app"]).await, CommandOutcome::Failed);
}
