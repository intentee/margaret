use margaret::framework::console::command_outcome::CommandOutcome;

#[tokio::main]
async fn main() -> CommandOutcome {
    let container = margaret_example::margaret::container::build::build();

    Box::pin(margaret_example::margaret::run::run(
        &container,
        std::env::args_os(),
    ))
    .await
}

#[cfg(test)]
mod tests {
    use margaret::framework::console::command_outcome::CommandOutcome;

    #[test]
    fn returns_failed_without_a_recognized_command() {
        assert_eq!(super::main(), CommandOutcome::Failed);
    }
}
