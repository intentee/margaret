use margaret_console::command_outcome::CommandOutcome;

fn main() {
    let outcome: Result<CommandOutcome, std::io::Error> = Ok(CommandOutcome::Succeeded);

    let _ = CommandOutcome::from_user_result(outcome);
}
