use clap::Command;

use crate::command_outcome::CommandOutcome;

pub fn print_help(command: &mut Command) -> CommandOutcome {
    println!("{}", command.render_help());

    CommandOutcome::Failed
}

#[cfg(test)]
mod tests {
    use clap::Command;

    use super::print_help;
    use crate::command_outcome::CommandOutcome;

    #[test]
    fn prints_help_and_reports_failure() {
        let mut command = Command::new("app").subcommand(Command::new("greet"));

        assert_eq!(print_help(&mut command), CommandOutcome::Failed);
    }
}
