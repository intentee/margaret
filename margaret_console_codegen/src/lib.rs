mod console_argument;
mod console_argument_arguments;
pub mod console_codegen_error;
mod console_command;
mod console_command_arguments;
mod console_commands;
pub mod has_commands;
mod optional_parameter;
mod render;
pub mod render_console;

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;
    use tempfile::tempdir;

    use margaret_attributes::attribute_index::AttributeIndex;
    use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
    use margaret_attributes::crate_root::CrateRoot;
    use margaret_http_codegen::http_server::HttpServer;

    use crate::has_commands::has_commands;
    use crate::render_console::render_console;

    const COMMANDS: &str = r#"
trait Greeter: Send + Sync {}

#[singleton]
#[console_command(name = "demo", description = "Demonstrates arguments")]
struct Demo {
    greeter: Arc<dyn Greeter>,
}

impl Demo {
    #[constructor]
    fn create(greeter: Arc<dyn Greeter>) -> Self {}

    #[process]
    fn run(
        &self,
        #[console_argument(from = "name")] name: String,
        #[console_argument(from = "salutation")] salutation: Option<String>,
        #[console_argument(from = "loud")] loud: bool,
    ) -> CommandOutcome {}
}

#[singleton]
#[console_command(name = "farewell")]
struct Farewell;

impl Farewell {
    #[process]
    fn run(&self) -> CommandOutcome {}
}
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

        AttributeIndexBuilder::new()
            .index_crate(&CrateRoot::new("crate", directory.path().join("src")))
            .expect("the crate is indexed")
            .build()
    }

    fn source_for(lib_source: &str, has_http: bool) -> String {
        rendered(lib_source, has_http)
    }

    fn rendered(lib_source: &str, has_http: bool) -> String {
        let servers = if has_http {
            vec![HttpServer::new("public".to_string())]
        } else {
            Vec::new()
        };

        render_console(&index_for(lib_source), has_http, &servers)
            .expect("the console source is generated")
            .format()
            .source()
            .split_whitespace()
            .collect()
    }

    fn error_for(lib_source: &str) -> String {
        render_console(&index_for(lib_source), false, &[])
            .expect_err("the console source fails to generate")
            .to_string()
    }

    #[test]
    fn generates_a_dispatcher_for_each_argument_kind() {
        let source = source_for(COMMANDS, false);

        assert!(source.contains("pubasyncfnrun"));
        assert!(source.contains(r#"clap::Command::new("demo").about("Demonstratesarguments")"#));

        assert!(source.contains(r#"clap::Arg::new("name").required(true)"#));
        assert!(
            source.contains(r#"clap::Arg::new("salutation").long("salutation").required(false)"#)
        );
        assert!(
            source.contains(
                r#"clap::Arg::new("loud").long("loud").action(clap::ArgAction::SetTrue)"#
            )
        );
        assert!(source.contains("clap::value_parser!(String)"));

        assert!(source.contains("container.demo().await.run("));
        assert!(source.contains(r#"matches.get_one::<String>("name")"#));
        assert!(source.contains(r#"matches.get_one::<String>("salutation").cloned()"#));
        assert!(source.contains(r#"matches.get_flag("loud")"#));

        assert!(source.contains(r#"("farewell",_matches)"#));
        assert!(source.contains("container.farewell().await.run().await"));
    }

    #[test]
    fn adds_a_serve_command_when_http_exists() {
        let source = source_for("struct App;\n", true);

        assert!(source.contains(r#"clap::Command::new("serve")"#));
        assert!(
            source.contains(r#"clap::Arg::new("public-addr").long("public-addr").required(true)"#)
        );
        assert!(
            source.contains(r#"clap::Arg::new("public-url").long("public-url").required(false)"#)
        );
        assert!(source.contains(
            r#"clap::Arg::new("public-uploads").long("public-uploads").action(clap::ArgAction::SetTrue)"#
        ));
        assert!(source.contains(
            r#"clap::Arg::new("public-upload-dir").long("public-upload-dir").required(false).requires("public-uploads")"#
        ));
        assert!(source.contains("super::services::serve(container,matches,cancellation_token)"));
        assert!(source.contains("margaret_service::install::install()"));
    }

    #[test]
    fn registers_one_address_argument_per_active_server() {
        let source: String = render_console(
            &index_for("struct App;\n"),
            true,
            &[
                HttpServer::new("public".to_string()),
                HttpServer::new("internal".to_string()),
            ],
        )
        .expect("the console source is generated")
        .format()
        .source()
        .split_whitespace()
        .collect();

        assert!(
            source.contains(r#"clap::Arg::new("public-addr").long("public-addr").required(true)"#)
        );
        assert!(
            source.contains(
                r#"clap::Arg::new("internal-addr").long("internal-addr").required(true)"#
            )
        );
    }

    #[test]
    fn registers_a_serve_command_without_addr_for_a_service_only_app() {
        let source: String = render_console(&index_for("struct App;\n"), true, &[])
            .expect("the console source is generated")
            .format()
            .source()
            .split_whitespace()
            .collect();

        assert!(source.contains(r#"clap::Command::new("serve")"#));
        assert!(!source.contains(r#"clap::Arg::new("addr")"#));
        assert!(!source.contains("-addr"));
    }

    #[test]
    fn injects_the_cancellation_token_into_a_command_runner() {
        let source = source_for(
            "#[singleton]\n#[console_command(name = \"watch\")]\nstruct Watch;\n\nimpl Watch {\n    #[process]\n    fn run(&self, #[console_argument(from = \"target\")] target: String, token: CancellationToken) -> CommandOutcome {}\n}\n",
            false,
        );

        assert!(source.contains("letcancellation_token=margaret_service::install::install();"));
        assert!(source.contains("cancellation_token,).await"));
    }

    #[test]
    fn ignores_a_receiver_in_a_runner() {
        let source = source_for(
            "#[singleton]\n#[console_command(name = \"flagged\")]\nstruct Flagged;\n\nimpl Flagged {\n    #[process]\n    fn run(&self, #[console_argument(from = \"loud\")] loud: bool) -> CommandOutcome {}\n}\n",
            false,
        );

        assert!(source.contains("container.flagged().await.run("));
        assert!(source.contains(r#"matches.get_flag("loud")"#));
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
    fn rejects_a_non_string_command_name() {
        let message = error_for("#[console_command(name = 5)]\nstruct Bad;\n");

        assert!(message.contains("failed to index"));
    }

    #[test]
    fn rejects_a_non_string_command_description() {
        let message = error_for("#[console_command(name = \"x\", description = 5)]\nstruct Bad;\n");

        assert!(message.contains("failed to index"));
    }

    #[test]
    fn dispatches_a_fieldless_command_without_a_constructor() {
        let source = source_for(
            "#[console_command(name = \"bare\")]\nstruct Bare;\n\nimpl Bare {\n    #[process]\n    fn run(&self) -> CommandOutcome {}\n}\n",
            false,
        );

        assert!(source.contains(r#"("bare",_matches)"#));
        assert!(source.contains("container.bare().await.run().await"));
    }

    #[test]
    fn reports_a_missing_command_runner() {
        let message = error_for("#[console_command(name = \"bad\")]\nstruct Bad;\n");

        assert!(message.contains("no #[process] method"));
    }

    #[test]
    fn rejects_an_unmarked_runner_parameter() {
        let message = error_for(
            "#[console_command(name = \"bad\")]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn run(&self, value: String) -> CommandOutcome {}\n}\n",
        );

        assert!(message.contains("must be a console argument"));
    }

    #[test]
    fn binds_a_destructured_console_argument_named_by_from() {
        let source = source_for(
            "#[console_command(name = \"plot\")]\nstruct Plot;\n\nimpl Plot {\n    #[process]\n    fn run(&self, #[console_argument(from = \"point\")] Point { x, y }: Point) -> CommandOutcome {}\n}\n",
            false,
        );

        assert!(source.contains(r#"clap::Arg::new("point").required(true)"#));
        assert!(source.contains(r#"matches.get_one::<Point>("point")"#));
    }

    #[test]
    fn rejects_a_console_argument_without_from() {
        let message = error_for(
            "#[console_command(name = \"bad\")]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn run(&self, #[console_argument] flag: bool) -> CommandOutcome {}\n}\n",
        );

        assert!(message.contains("must name the command-line argument it binds"));
    }

    #[test]
    fn rejects_a_non_string_argument_source() {
        let message = error_for(
            "#[console_command(name = \"bad\")]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn run(&self, #[console_argument(from = 5)] value: String) -> CommandOutcome {}\n}\n",
        );

        assert!(message.contains("failed to index"));
    }

    #[test]
    fn rejects_a_malformed_argument() {
        let message = error_for(
            "#[console_command(name = \"bad\")]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn run(&self, #[console_argument(= 5)] value: String) -> CommandOutcome {}\n}\n",
        );

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
