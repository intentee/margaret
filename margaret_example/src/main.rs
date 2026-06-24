use margaret_console::command_outcome::CommandOutcome;
use margaret_example::margaret::container::Container;

#[tokio::main(flavor = "current_thread")]
async fn main() -> CommandOutcome {
    let container = Container::build();

    margaret_example::margaret::console::run(&container, std::env::args_os()).await
}

#[cfg(test)]
mod tests {
    use margaret_console::command_outcome::CommandOutcome;

    #[test]
    fn returns_failed_without_a_recognized_command() {
        assert_eq!(super::main(), CommandOutcome::Failed);
    }
}
