pub mod console_codegen_error;
mod console_command;
mod render;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;

use crate::console_codegen_error::ConsoleCodegenError;
use crate::console_command::console_commands;
use crate::render::render;

pub fn has_commands(index: &AttributeIndex) -> bool {
    let selector = AttributeSelector::parse("console_command").expect("a valid selector");

    !index.select(&selector).is_empty()
}

pub fn render_console(
    index: &AttributeIndex,
    has_http: bool,
) -> Result<String, ConsoleCodegenError> {
    let commands = console_commands(index)?;

    Ok(render(&commands, has_http))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use margaret_attributes::attribute_index::AttributeIndex;
    use tempfile::TempDir;
    use tempfile::tempdir;

    use crate::has_commands;
    use crate::render_console;

    const COMMANDS: &str = r#"
#[console_command(name = "greet", description = "Greets a person")]
struct Greet;

#[console_command(name = "farewell")]
struct Farewell;
"#;

    fn crate_with(lib_source: &str) -> TempDir {
        let directory = tempdir().expect("a temporary crate directory is created");
        let source_directory = directory.path().join("src");

        fs::create_dir(&source_directory).expect("the src directory is created");
        fs::write(source_directory.join("lib.rs"), lib_source).expect("lib.rs is written");

        directory
    }

    fn index_for(lib_source: &str) -> AttributeIndex {
        let directory = crate_with(lib_source);

        AttributeIndex::from_crate_root("crate", &directory.path().join("src"))
            .expect("the crate is indexed")
    }

    fn source_for(lib_source: &str, has_http: bool) -> String {
        render_console(&index_for(lib_source), has_http)
            .expect("the console source is generated")
            .split_whitespace()
            .collect()
    }

    fn error_for(lib_source: &str) -> String {
        render_console(&index_for(lib_source), false)
            .expect_err("the console source fails to generate")
            .to_string()
    }

    #[test]
    fn generates_a_dispatcher_for_commands() {
        let source = source_for(COMMANDS, false);

        assert!(source.contains("pubasyncfnrun"));
        assert!(source.contains("usemargaret_console::command::Command;"));
        assert!(source.contains("container:&super::container::Container"));
        assert!(!source.contains("Container::default()"));
        assert!(source.contains(r#"clap::Command::new("greet").about("Greetsaperson")"#));
        assert!(source.contains(r#"clap::Command::new("farewell").args("#));
        assert!(source.contains(r#"Some(("greet",matches))=>container.greet.run(matches)"#));
        assert!(source.contains(r#"Some(("farewell",matches))=>container.farewell.run(matches)"#));
        assert!(source.contains("margaret_console::print_help::print_help(&mutcommand)"));
        assert!(!source.contains("serve"));
        assert!(!source.contains("super::http"));
    }

    #[test]
    fn adds_a_serve_command_when_http_exists() {
        let source = source_for("struct App;\n", true);

        assert!(source.contains(r#"clap::Command::new("serve")"#));
        assert!(source.contains(r#"clap::Arg::new("addr").long("addr").required(true)"#));
        assert!(source.contains("container:&super::container::Container"));
        assert!(source.contains("margaret_console::serve::serve(super::http::server(container)"));
        assert!(!source.contains("Container::default()"));
        assert!(!source.contains("command::Command"));
        assert!(!source.contains("greet"));
    }

    #[test]
    fn rejects_console_command_on_a_non_struct() {
        let message = error_for("#[console_command(name = \"bad\")]\nenum Bad {}\n");

        assert!(message.contains("#[console_command]"));
    }

    #[test]
    fn requires_a_command_name() {
        let message = error_for("#[console_command]\nstruct Bad;\n");

        assert!(message.contains("missing the 'name'"));
    }

    #[test]
    fn propagates_malformed_command_arguments() {
        let message = error_for("#[console_command(= 5)]\nstruct Bad;\n");

        assert!(message.contains("failed to index"));
    }

    #[test]
    fn rejects_a_non_string_name() {
        let message = error_for("#[console_command(name = 5)]\nstruct Bad;\n");

        assert!(message.contains("failed to index"));
    }

    #[test]
    fn rejects_a_non_string_description() {
        let message = error_for("#[console_command(name = \"x\", description = 5)]\nstruct Bad;\n");

        assert!(message.contains("failed to index"));
    }

    #[test]
    fn reports_commands_present() {
        assert!(has_commands(&index_for(COMMANDS)));
    }

    #[test]
    fn reports_no_commands() {
        assert!(!has_commands(&index_for("struct App;\n")));
    }
}
