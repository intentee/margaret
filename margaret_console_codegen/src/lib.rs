pub mod console_codegen_error;
mod console_command;
mod console_command_arguments;
mod console_commands;
pub mod has_commands;
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
    use margaret_console_argument_codegen::scan::scan;
    use margaret_container::container_bindings::ContainerBindings;
    use margaret_container::render_container::render_container;
    use margaret_http_codegen::http_server::HttpServer;
    use margaret_http_codegen::server_transport_policy::ServerTransportPolicy;

    use crate::has_commands::has_commands;
    use crate::render_console::render_console;

    const COMMANDS: &str = r#"
use std::sync::Arc;

#[singleton]
struct EnglishGreeter;

impl EnglishGreeter {
    #[constructor]
    fn create() -> Self {}
}

#[singleton]
#[console_command(name = "demo", description = "Demonstrates arguments")]
struct Demo {
    greeter: Arc<EnglishGreeter>,
    name: String,
    salutation: Option<String>,
    loud: bool,
}

impl Demo {
    #[constructor]
    fn create(
        greeter: Arc<EnglishGreeter>,
        #[console_argument(positional)] name: String,
        #[console_argument(from = "salutation")] salutation: Option<String>,
        #[console_argument(from = "loud")] loud: bool,
    ) -> Self {}

    #[process]
    fn run(&self) -> CommandOutcome {}
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

    fn bindings(index: &AttributeIndex) -> ContainerBindings {
        let registry = scan(index).expect("the console arguments are scanned");

        render_container(index, &registry, &[])
            .expect("the container renders")
            .bindings
    }

    fn source_for(lib_source: &str, has_http: bool) -> String {
        let index = index_for(lib_source);
        let servers = if has_http {
            vec![HttpServer::new(
                "public".to_string(),
                ServerTransportPolicy::Negotiable,
            )]
        } else {
            Vec::new()
        };

        render_console(&index, has_http, false, &servers, &[], &bindings(&index))
            .expect("the console source is generated")
            .format()
            .expect("the module formats")
            .source()
            .split_whitespace()
            .collect()
    }

    fn error_for(lib_source: &str) -> String {
        let index = index_for(lib_source);

        render_console(&index, false, false, &[], &[], &bindings(&index))
            .expect_err("the console source fails to generate")
            .to_string()
    }

    #[test]
    fn generates_a_dispatcher_for_each_argument_kind() {
        let source = source_for(COMMANDS, false);

        assert!(source.contains("pubasyncfnrun"));
        assert!(source.contains("run<Arguments,Argument>(container:&super::container::Container,"));
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
        assert!(source.contains("clap::value_parser!(std::string::String)"));

        assert!(source.contains("container.demo("));
        assert!(source.contains(r#"matches.get_one::<std::string::String>("name")"#));
        assert!(
            source.contains(r#"matches.get_one::<std::string::String>("salutation").cloned()"#)
        );
        assert!(source.contains(r#"matches.get_flag("loud")"#));

        assert!(source.contains(r#"("farewell",_matches)"#));
        assert!(source.contains("container.farewell().await.run().await"));
    }

    #[test]
    fn adds_a_serve_command_when_http_exists() {
        let source = source_for("struct App;\n", true);

        assert!(source.contains("pubasyncfnrun"));
        assert!(source.contains(r#"clap::Command::new("serve")"#));
        assert!(
            source.contains(r#"clap::Arg::new("public-addr").long("public-addr").required(true)"#)
        );
        assert!(
            source.contains(r#"clap::Arg::new("public-url").long("public-url").required(true)"#)
        );
        assert!(source.contains(
            r#"clap::Arg::new("public-uploads").long("public-uploads").action(clap::ArgAction::SetTrue)"#
        ));
        assert!(source.contains(
            r#"clap::Arg::new("public-upload-dir").long("public-upload-dir").required(false).requires("public-uploads")"#
        ));
        assert!(source.contains(
            "margaret::framework::service::dispatch_serve::dispatch_serve(margaret::framework::service::install::install,|cancellation_token|super::serve::serve(container,matches,cancellation_token,),)"
        ));
    }

    #[test]
    fn emits_spiffe_transport_flags_when_a_server_is_pinned() {
        let index = index_for("struct App;\n");
        let source: String = render_console(
            &index,
            true,
            false,
            &[
                HttpServer::new(
                    "internal".to_string(),
                    ServerTransportPolicy::PinnedSpiffeMtls,
                ),
                HttpServer::new("public".to_string(), ServerTransportPolicy::Negotiable),
            ],
            &[],
            &bindings(&index),
        )
        .expect("the console source is generated")
        .format()
        .expect("the module formats")
        .source()
        .split_whitespace()
        .collect();

        assert!(source.contains(
            r#"clap::Arg::new("internal-transport").long("internal-transport").required(true).value_parser(["spiffe_mtls"])"#
        ));
        assert!(source.contains(
            r#"clap::Arg::new("public-transport").long("public-transport").required(true).value_parser(["plain","spiffe_mtls"])"#
        ));
        assert!(source.contains(
            r#"clap::Arg::new("spiffe-trust-domain").long("spiffe-trust-domain").required(true)"#
        ));
        assert!(source.contains(
            r#"clap::Arg::new("spire-agent-addr").long("spire-agent-addr").required(true)"#
        ));
    }

    #[test]
    fn omits_transport_flags_when_no_server_is_pinned() {
        let source = source_for("struct App;\n", true);

        assert!(!source.contains("public-transport"));
        assert!(!source.contains("spiffe-trust-domain"));
    }

    #[test]
    fn registers_one_address_argument_per_active_server() {
        let index = index_for("struct App;\n");
        let source: String = render_console(
            &index,
            true,
            false,
            &[
                HttpServer::new("public".to_string(), ServerTransportPolicy::Negotiable),
                HttpServer::new("internal".to_string(), ServerTransportPolicy::Negotiable),
            ],
            &[],
            &bindings(&index),
        )
        .expect("the console source is generated")
        .format()
        .expect("the module formats")
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
        let index = index_for("struct App;\n");
        let source: String = render_console(&index, true, false, &[], &[], &bindings(&index))
            .expect("the console source is generated")
            .format()
            .expect("the module formats")
            .source()
            .split_whitespace()
            .collect();

        assert!(source.contains(r#"clap::Command::new("serve")"#));
        assert!(!source.contains(r#"clap::Arg::new("addr")"#));
        assert!(!source.contains("-addr"));
    }

    #[test]
    fn adds_a_schema_command_when_models_exist() {
        let index = index_for("struct App;\n");
        let source: String = render_console(&index, false, true, &[], &[], &bindings(&index))
            .expect("the console source is generated")
            .format()
            .expect("the module formats")
            .source()
            .split_whitespace()
            .collect();

        assert!(source.contains(r#"clap::Command::new("schema")"#));
        assert!(source.contains(r#"Some(("schema",_matches))=>{"#));
        assert!(source.contains("render_postgres(&super::schema::schema())"));
        assert!(source.contains("CommandOutcome::Succeeded"));
        assert!(
            source.contains("run<Arguments,Argument>(_container:&super::container::Container,")
        );
    }

    #[test]
    fn emits_a_synchronous_run_when_only_the_schema_command_exists() {
        let index = index_for("struct App;\n");
        let source: String = render_console(&index, false, true, &[], &[], &bindings(&index))
            .expect("the console source is generated")
            .format()
            .expect("the module formats")
            .source()
            .split_whitespace()
            .collect();

        assert!(source.contains("pubfnrun<Arguments,Argument>"));
        assert!(!source.contains("pubasyncfnrun"));
    }

    #[test]
    fn wraps_a_command_run_in_a_console_outcome() {
        let source = source_for(
            "#[singleton]\n#[console_command(name = \"seed\")]\nstruct Seed;\n\nimpl Seed {\n    #[constructor]\n    fn create() -> Self {}\n\n    #[process]\n    fn run(&self) -> CommandOutcome {}\n}\n",
            false,
        );

        assert!(source.contains("matchcontainer.seed().await.run().await{"));
        assert!(source.contains(
            "Ok(())=>{margaret::framework::console::command_outcome::CommandOutcome::Succeeded}"
        ));
        assert!(source.contains(
            "Err(error)=>{margaret::framework::console::report_failure::report_failure(margaret::framework::console::console_error::ConsoleError::UserError(error,),)}"
        ));
    }

    #[test]
    fn injects_the_cancellation_token_into_a_command_runner() {
        let source = source_for(
            "use tokio_util::sync::CancellationToken;\n\n#[singleton]\n#[console_command(name = \"watch\")]\nstruct Watch {\n    target: String,\n}\n\nimpl Watch {\n    #[constructor]\n    fn create(#[console_argument(positional)] target: String) -> Self {}\n\n    #[process]\n    fn run(&self, token: CancellationToken) -> CommandOutcome {}\n}\n",
            false,
        );

        assert!(source.contains(
            "letcancellation_token=matchmargaret::framework::service::install::install()"
        ));
        assert!(source.contains(".run(cancellation_token).await"));
    }

    #[test]
    fn renders_a_command_that_takes_only_a_flag() {
        let source = source_for(
            "#[singleton]\n#[console_command(name = \"flagged\")]\nstruct Flagged {\n    loud: bool,\n}\n\nimpl Flagged {\n    #[constructor]\n    fn create(#[console_argument(from = \"loud\")] loud: bool) -> Self {}\n\n    #[process]\n    fn run(&self) -> CommandOutcome {}\n}\n",
            false,
        );

        assert!(
            source.contains(r#"container.flagged(matches.get_flag("loud")).await.run().await"#)
        );
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
    fn rejects_two_commands_registering_the_same_name() {
        let message = error_for(
            "#[console_command(name = \"greet\")]\nstruct First;\n\nimpl First {\n    #[process]\n    fn run(&self) -> CommandOutcome {}\n}\n\n#[console_command(name = \"greet\")]\nstruct Second;\n\nimpl Second {\n    #[process]\n    fn run(&self) -> CommandOutcome {}\n}\n",
        );

        assert!(message.contains("already registered"));
    }

    #[test]
    fn rejects_a_console_command_that_injects_the_spiffe_http_client() {
        let message = error_for(
            "use reqwest::Client;\n\n#[singleton]\nstruct OutboundCaller {\n    client: Client,\n}\n\nimpl OutboundCaller {\n    #[constructor]\n    fn create(#[spiffe_http_client] client: Client) -> Self {}\n}\n\n#[singleton]\n#[console_command(name = \"call\")]\nstruct Call {\n    caller: std::sync::Arc<OutboundCaller>,\n}\n\nimpl Call {\n    #[constructor]\n    fn create(caller: std::sync::Arc<OutboundCaller>) -> Self {}\n\n    #[process]\n    fn run(&self) -> CommandOutcome {}\n}\n",
        );

        assert!(message.contains("injects the #[spiffe_http_client]"));
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
            "#[singleton]\n#[console_command(name = \"bare\")]\nstruct Bare;\n\nimpl Bare {\n    #[process]\n    fn run(&self) -> CommandOutcome {}\n}\n",
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
    fn rejects_a_non_token_runner_parameter() {
        let message = error_for(
            "#[console_command(name = \"bad\")]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn run(&self, value: String) -> CommandOutcome {}\n}\n",
        );

        assert!(message.contains("may only take &self and an optional CancellationToken"));
    }

    #[test]
    fn rejects_a_request_binding_marker_on_a_runner_parameter() {
        let message = error_for(
            "use tokio_util::sync::CancellationToken;\n\n#[console_command(name = \"bad\")]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn run(&self, #[form_request(from = Query)] token: CancellationToken) -> CommandOutcome {}\n}\n",
        );

        assert!(message.contains("carries #[form_request]"));
        assert!(message.contains("only available in an HTTP responder"));
    }

    #[test]
    fn binds_a_destructured_positional_console_argument() {
        let source = source_for(
            "struct Point {\n    x: i32,\n    y: i32,\n}\n\n#[singleton]\n#[console_command(name = \"plot\")]\nstruct Plot {\n    point: Point,\n}\n\nimpl Plot {\n    #[constructor]\n    fn create(#[console_argument(positional)] Point { x, y }: Point) -> Self {}\n\n    #[process]\n    fn run(&self) -> CommandOutcome {}\n}\n",
            false,
        );

        assert!(source.contains(r#"clap::Arg::new("argument_0").required(true)"#));
        assert!(source.contains(r#"matches.get_one::<crate::Point>("argument_0")"#));
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
