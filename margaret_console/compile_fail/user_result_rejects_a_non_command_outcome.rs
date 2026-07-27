use margaret_console::command_outcome::CommandOutcome;

fn main() {
    let outcome: anyhow::Result<()> = Ok(());

    let _ = CommandOutcome::from_user_result(outcome);
}
