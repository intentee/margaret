use std::ffi::OsString;
use std::path::Path;

use clap::Parser as _;

use margaret_console::command_outcome::CommandOutcome;
use margaret_console::outcome_for_clap_error::outcome_for_clap_error;
use margaret_console::report_failure::report_failure;

use crate::margaret_arguments::MargaretArguments;
use crate::margaret_command::MargaretCommand;
use crate::parse_project_name::parse_project_name;
use crate::project_name::ProjectName;
use crate::project_name_outcome::ProjectNameOutcome;
use crate::render_project::render_project;
use crate::scaffold_error::ScaffoldError;
use crate::scaffold_operation::ScaffoldOperation;
use crate::workspace_manifest::WORKSPACE_MANIFEST;
use crate::write_scaffolded_files::write_scaffolded_files;

fn scaffold_project(
    margaret_revision: &str,
    project_name: &ProjectName,
    workspace_manifest: &str,
) -> Result<(), ScaffoldError> {
    let files = render_project(margaret_revision, project_name, workspace_manifest)?;

    write_scaffolded_files(Path::new(project_name.target_directory()), &files)
}

pub fn run<Arguments, Argument>(args: Arguments) -> CommandOutcome
where
    Arguments: IntoIterator<Item = Argument>,
    Argument: Into<OsString> + Clone,
{
    let arguments = match MargaretArguments::try_parse_from(args) {
        Ok(arguments) => arguments,
        Err(error) => return outcome_for_clap_error(&error),
    };
    let MargaretCommand::Scaffold(scaffold) = arguments.command;
    let ScaffoldOperation::Init(init) = scaffold.operation;

    match parse_project_name(&init.project_name) {
        ProjectNameOutcome::Accepted(project_name) => {
            match scaffold_project(&init.margaret_rev, &project_name, WORKSPACE_MANIFEST) {
                Ok(()) => CommandOutcome::Succeeded,
                Err(error) => report_failure(error),
            }
        }
        ProjectNameOutcome::Rejected(rejection) => report_failure(rejection),
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs;

    use tempfile::tempdir;

    use margaret_console::command_outcome::CommandOutcome;

    use super::run;
    use super::scaffold_project;
    use crate::project_name::ProjectName;

    #[test]
    fn scaffolds_the_project_into_a_directory_named_after_it() {
        let workspace = tempdir().expect("a temporary workspace directory");

        env::set_current_dir(workspace.path()).expect("the working directory is switched");

        let outcome = run([
            "margaret",
            "scaffold",
            "init",
            "acme",
            "--margaret-rev",
            "83a27bf",
        ]);

        assert_eq!(outcome, CommandOutcome::Succeeded);
        assert!(
            fs::read_to_string(workspace.path().join("acme").join("Cargo.toml"))
                .expect("the workspace manifest is readable")
                .contains("acme_identity")
        );
        assert!(
            workspace
                .path()
                .join("acme")
                .join("acme_base")
                .join("src")
                .join("main.rs")
                .is_file()
        );
    }

    #[test]
    fn reports_a_directory_that_is_already_taken() {
        let workspace = tempdir().expect("a temporary workspace directory");

        env::set_current_dir(workspace.path()).expect("the working directory is switched");
        fs::create_dir(workspace.path().join("acme")).expect("the directory is taken up front");

        let outcome = run([
            "margaret",
            "scaffold",
            "init",
            "acme",
            "--margaret-rev",
            "83a27bf",
        ]);

        assert_eq!(outcome, CommandOutcome::Failed);
    }

    #[test]
    fn reports_a_project_name_that_is_not_a_crate_name() {
        let outcome = run([
            "margaret",
            "scaffold",
            "init",
            "Acme-Corp",
            "--margaret-rev",
            "83a27bf",
        ]);

        assert_eq!(outcome, CommandOutcome::Failed);
    }

    #[test]
    fn reports_an_invocation_without_the_revision_to_pin() {
        assert_eq!(
            run(["margaret", "scaffold", "init", "acme"]),
            CommandOutcome::Failed
        );
    }

    #[test]
    fn reports_an_invocation_without_a_subcommand() {
        assert_eq!(run(["margaret"]), CommandOutcome::Failed);
    }

    #[test]
    fn reports_an_invocation_without_a_scaffold_operation() {
        assert_eq!(run(["margaret", "scaffold"]), CommandOutcome::Failed);
    }

    #[test]
    fn renders_the_help_of_the_scaffolder_as_a_success() {
        assert_eq!(run(["margaret", "--help"]), CommandOutcome::Succeeded);
    }

    #[test]
    fn propagates_a_workspace_manifest_that_cannot_be_read() {
        let error = scaffold_project(
            "83a27bf",
            &ProjectName::new("acme".to_string()),
            "not = = toml",
        )
        .expect_err("an invalid workspace manifest is reported");

        assert!(
            error
                .to_string()
                .contains("the workspace manifest is not valid TOML")
        );
    }
}
