use std::env;

use margaret::framework::console::command_outcome::CommandOutcome;
use margaret_cluster_fixture::margaret::run;

#[tokio::main]
async fn main() -> CommandOutcome {
    run::run(env::args_os()).await
}

#[cfg(test)]
mod tests {
    use margaret::framework::console::command_outcome::CommandOutcome;

    #[test]
    fn returns_failed_without_a_recognized_command() {
        assert_eq!(super::main(), CommandOutcome::Failed);
    }
}
