pub mod console_artifacts;
pub mod console_codegen_error;
mod console_command;
mod console_command_arguments;
mod console_commands;
pub mod console_plan;
mod render;
pub mod render_console;

#[cfg(test)]
mod tests {
    use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
    use margaret_attributes::attribute_index::AttributeIndex;
    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_container::constructor_outcome::ConstructorOutcome;
    use margaret_container::container_bindings::ContainerBindings;
    use margaret_container::framework_construction::FrameworkConstruction;
    use margaret_container::framework_dependency::FrameworkDependency;
    use margaret_container::framework_enablement::FrameworkEnablement;
    use margaret_container::framework_injection_role::FrameworkInjectionRole;
    use margaret_container::framework_provider::FrameworkProvider;
    use margaret_container::render_container::render_container;
    use margaret_container::slotted_serve_input::SlottedServeInput;
    use margaret_database_codegen::declared_postgres_database::DeclaredPostgresDatabase;
    use margaret_http_codegen::http_server::HttpServer;
    use margaret_http_codegen::server_transport_policy::ServerTransportPolicy;
    use margaret_http_codegen::server_uploads::ServerUploads;
    use margaret_serve_input_codegen::route_url_input::RouteUrlInput;
    use margaret_serve_input_codegen::scan::scan;
    use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;

    use crate::console_artifacts::ConsoleArtifacts;
    use crate::console_codegen_error::ConsoleCodegenError;
    use crate::console_plan::ConsolePlan;
    use crate::render_console::render_console;

    const COMMANDS: &str = r#"
use std::sync::Arc;

#[singleton]
struct EnglishGreeter;

impl EnglishGreeter {
    #[constructor]
    fn create() -> anyhow::Result<Self> {}
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
    ) -> anyhow::Result<Self> {}

    #[process]
    fn run(&self) -> anyhow::Result<CommandOutcome> {}
}

#[singleton]
#[console_command(name = "farewell")]
struct Farewell;

impl Farewell {
    #[process]
    fn run(&self) -> anyhow::Result<CommandOutcome> {}
}
"#;

    fn bindings(index: &AttributeIndex) -> ContainerBindings {
        let registry = scan(index).expect("the console arguments are scanned");

        render_container(
            index,
            &registry,
            &[],
            &DeclaredPostgresDatabase::Absent,
            &DeclaredTokenIssuance::Absent,
        )
        .expect("the container renders")
        .bindings
    }

    fn render_planned_console(
        index: &AttributeIndex,
        serves: bool,
        has_models: bool,
        http_servers: &[HttpServer],
        serve_inputs: &[SlottedServeInput],
        bindings: &ContainerBindings,
    ) -> Result<ConsoleArtifacts, ConsoleCodegenError> {
        let plan = ConsolePlan::build(index, bindings)?;

        Ok(render_console(
            &plan,
            serves,
            has_models,
            http_servers,
            serve_inputs,
            bindings,
        ))
    }

    fn source_for(lib_source: &str, has_http: bool) -> String {
        let index = IndexedSource::new(lib_source).index;
        let http_servers = if has_http {
            vec![HttpServer::new(
                "public".to_string(),
                ServerTransportPolicy::Negotiable,
                ServerUploads::Refused,
            )]
        } else {
            Vec::new()
        };

        let bindings = bindings(&index);
        let plan = ConsolePlan::build(&index, &bindings).expect("the console is planned");

        render_console(&plan, has_http, false, &http_servers, &[], &bindings)
            .modules
            .into_iter()
            .map(|module| {
                module
                    .format()
                    .expect("the module formats")
                    .source()
                    .to_string()
            })
            .collect::<String>()
            .split_whitespace()
            .collect()
    }

    fn rejection_for(lib_source: &str) -> ConsoleCodegenError {
        let index = IndexedSource::new(lib_source).index;

        render_planned_console(&index, false, false, &[], &[], &bindings(&index))
            .expect_err("the console source fails to generate")
    }

    fn error_for(lib_source: &str) -> String {
        rejection_for(lib_source).to_string()
    }

    #[test]
    fn rejects_a_command_absent_from_the_container_plan() {
        let index = IndexedSource::new(COMMANDS).index;
        let empty_bindings = bindings(&IndexedSource::new("").index);
        let error = render_planned_console(&index, false, false, &[], &[], &empty_bindings)
            .map(drop)
            .expect_err("every command must belong to the same container plan");

        assert!(error.to_string().contains("crate::Demo"));
    }

    #[test]
    fn generates_a_dispatcher_for_each_argument_kind() {
        let source = source_for(COMMANDS, false);

        assert!(source.contains("pubfnrun"));
        assert!(source.contains("run<Arguments,Argument>(args:Arguments,)"));
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

        assert!(source.contains("super::container::build::construct_demo("));
        assert!(source.contains(r#"matches.get_one::<std::string::String>("name")"#));
        assert!(
            source.contains(r#"matches.get_one::<std::string::String>("salutation").cloned()"#)
        );
        assert!(source.contains(r#"matches.get_flag("loud")"#));

        assert!(source.contains(r#"Some(("farewell",_matches))=>{"#));
        assert!(source.contains("super::container::build::construct_farewell()"));
        assert!(!source.contains("super::container::build::construct_farewell().await"));
        assert!(source.contains("report_failure::report_failure(error"));
        assert!(source.contains(".run()"));
    }

    #[test]
    fn declares_the_package_version_on_the_generated_command() {
        let source = source_for(
            "#[singleton]\n#[console_command(name = \"boot\")]\nstruct Boot;\n\nimpl Boot {\n    #[process]\n    fn run(&self) -> anyhow::Result<CommandOutcome> {}\n}\n",
            false,
        );

        assert!(source.contains(r#".version(env!("CARGO_PKG_VERSION"))"#));
    }

    #[test]
    fn accepts_commands_named_after_the_generated_entry_point() {
        let source = source_for(
            "#[singleton]\n#[console_command(name = \"run\")]\nstruct Run;\n\nimpl Run {\n    #[process]\n    fn run(&self) -> anyhow::Result<CommandOutcome> {}\n}\n\n#[singleton]\n#[console_command(name = \"command\")]\nstruct Command;\n\nimpl Command {\n    #[process]\n    fn run(&self) -> anyhow::Result<CommandOutcome> {}\n}\n",
            false,
        );

        assert!(source.contains("pubfnrun<Arguments,Argument>"));
        assert!(source.contains(r#"Some(("run",_matches))=>{"#));
        assert!(source.contains(r#"Some(("command",_matches))=>{"#));
        assert!(source.contains("super::container::build::construct_run()"));
        assert!(source.contains("super::container::build::construct_command()"));
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
        assert!(!source.contains("public-upload-dir"));
        assert!(source.contains(
            "margaret::framework::service::dispatch_serve::dispatch_serve(margaret::framework::service::install::install,|cancellation_token|super::serve::serve(matches,cancellation_token,),)"
        ));
    }

    #[test]
    fn requires_the_upload_directory_of_a_server_that_accepts_uploads() {
        let index = IndexedSource::new("struct App;\n").index;
        let bindings = bindings(&index);
        let source: String = render_planned_console(
            &index,
            true,
            false,
            &[HttpServer::new(
                "public".to_string(),
                ServerTransportPolicy::Negotiable,
                ServerUploads::Accepted,
            )],
            &[],
            &bindings,
        )
        .expect("the console renders")
        .modules
        .into_iter()
        .map(|module| {
            module
                .format()
                .expect("the module formats")
                .source()
                .to_string()
        })
        .collect::<String>()
        .split_whitespace()
        .collect();

        assert!(source.contains(
            r#"clap::Arg::new("public-upload-dir").long("public-upload-dir").required(true).value_parser(clap::value_parser!(::std::path::PathBuf))"#
        ));
    }

    #[test]
    fn emits_spiffe_transport_flags_when_a_server_is_pinned() {
        let index = IndexedSource::new("struct App;\n").index;
        let source: String = render_planned_console(
            &index,
            true,
            false,
            &[
                HttpServer::new(
                    "internal".to_string(),
                    ServerTransportPolicy::PinnedSpiffeMtls,
                    ServerUploads::Refused,
                ),
                HttpServer::new(
                    "public".to_string(),
                    ServerTransportPolicy::Negotiable,
                    ServerUploads::Refused,
                ),
            ],
            &[],
            &bindings(&index),
        )
        .expect("the console source is generated")
        .modules
        .into_iter()
        .map(|module| {
            module
                .format()
                .expect("the module formats")
                .source()
                .to_string()
        })
        .collect::<String>()
        .split_whitespace()
        .collect();

        assert!(source.contains(
            r#"clap::Arg::new("internal-transport").long("internal-transport").required(true).value_parser(margaret::framework::service::transport_choice::TransportChoice::pinned_to_spiffe_mtls()"#
        ));
        assert!(source.contains(
            r#"clap::Arg::new("public-transport").long("public-transport").required(true).value_parser(clap::value_parser!(margaret::framework::service::transport_choice::TransportChoice)"#
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
        let index = IndexedSource::new("struct App;\n").index;
        let source: String = render_planned_console(
            &index,
            true,
            false,
            &[
                HttpServer::new(
                    "public".to_string(),
                    ServerTransportPolicy::Negotiable,
                    ServerUploads::Refused,
                ),
                HttpServer::new(
                    "internal".to_string(),
                    ServerTransportPolicy::Negotiable,
                    ServerUploads::Refused,
                ),
            ],
            &[],
            &bindings(&index),
        )
        .expect("the console source is generated")
        .modules
        .into_iter()
        .map(|module| {
            module
                .format()
                .expect("the module formats")
                .source()
                .to_string()
        })
        .collect::<String>()
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
        let index = IndexedSource::new("struct App;\n").index;
        let source: String =
            render_planned_console(&index, true, false, &[], &[], &bindings(&index))
                .expect("the console source is generated")
                .modules
                .into_iter()
                .map(|module| {
                    module
                        .format()
                        .expect("the module formats")
                        .source()
                        .to_string()
                })
                .collect::<String>()
                .split_whitespace()
                .collect();

        assert!(source.contains(r#"clap::Command::new("serve")"#));
        assert!(!source.contains(r#"clap::Arg::new("addr")"#));
        assert!(!source.contains("-addr"));
    }

    #[test]
    fn adds_a_schema_command_when_models_exist() {
        let index = IndexedSource::new("struct App;\n").index;
        let source: String =
            render_planned_console(&index, false, true, &[], &[], &bindings(&index))
                .expect("the console source is generated")
                .modules
                .into_iter()
                .map(|module| {
                    module
                        .format()
                        .expect("the module formats")
                        .source()
                        .to_string()
                })
                .collect::<String>()
                .split_whitespace()
                .collect();

        assert!(source.contains(r#"clap::Command::new("schema")"#));
        assert!(source.contains(r#"Some(("schema",_matches))=>{"#));
        assert!(source.contains("render_postgres(&super::schema::SCHEMA)"));
        assert!(source.contains("CommandOutcome::Succeeded"));
        assert!(source.contains("run<Arguments,Argument>(args:Arguments,)"));
    }

    #[test]
    fn emits_a_synchronous_run_when_only_the_schema_command_exists() {
        let index = IndexedSource::new("struct App;\n").index;
        let source: String =
            render_planned_console(&index, false, true, &[], &[], &bindings(&index))
                .expect("the console source is generated")
                .modules
                .into_iter()
                .map(|module| {
                    module
                        .format()
                        .expect("the module formats")
                        .source()
                        .to_string()
                })
                .collect::<String>()
                .split_whitespace()
                .collect();

        assert!(source.contains("pubfnrun<Arguments,Argument>"));
        assert!(!source.contains("pubasyncfnrun"));
    }

    #[test]
    fn injects_the_cancellation_token_into_a_command_runner() {
        let source = source_for(
            "use tokio_util::sync::CancellationToken;\n\n#[singleton]\n#[console_command(name = \"watch\")]\nstruct Watch {\n    target: String,\n}\n\nimpl Watch {\n    #[constructor]\n    fn create(#[console_argument(positional)] target: String) -> anyhow::Result<Self> {}\n\n    #[process]\n    fn run(&self, token: CancellationToken) -> anyhow::Result<CommandOutcome> {}\n}\n",
            false,
        );

        assert!(source.contains(
            "margaret::framework::service::dispatch_serve::dispatch_serve(margaret::framework::service::install::install,|cancellation_token|asyncmove"
        ));
        assert!(source.contains(".run(cancellation_token)"));
    }

    #[test]
    fn awaits_an_asynchronous_command_runner_that_takes_the_cancellation_token() {
        let source = source_for(
            "use tokio_util::sync::CancellationToken;\n\n#[singleton]\n#[console_command(name = \"watch\")]\nstruct Watch;\n\nimpl Watch {\n    #[process]\n    async fn run(&self, token: CancellationToken) -> anyhow::Result<CommandOutcome> {}\n}\n",
            false,
        );

        assert!(source.contains(".run(cancellation_token).await"));
    }

    #[test]
    fn awaits_a_command_that_declares_an_asynchronous_runner() {
        let source = source_for(
            "#[singleton]\n#[console_command(name = \"bare\")]\nstruct Bare;\n\nimpl Bare {\n    #[process]\n    async fn run(&self) -> anyhow::Result<CommandOutcome> {}\n}\n",
            false,
        );

        assert!(source.contains("pubasyncfnrun"));
        assert!(source.contains(".run().await"));
    }

    #[test]
    fn awaits_a_command_whose_construction_is_asynchronous() {
        let source = source_for(
            "use std::sync::Arc;\n\n#[singleton]\nstruct Slow;\n\nimpl Slow {\n    #[constructor]\n    async fn create() -> anyhow::Result<Self> {}\n}\n\n#[singleton]\n#[console_command(name = \"bare\")]\nstruct Bare {\n    slow: Arc<Slow>,\n}\n\nimpl Bare {\n    #[constructor]\n    fn create(slow: Arc<Slow>) -> anyhow::Result<Self> {}\n\n    #[process]\n    fn run(&self) -> anyhow::Result<CommandOutcome> {}\n}\n",
            false,
        );

        assert!(source.contains("pubasyncfnrun"));
        assert!(source.contains("super::container::build::construct_bare().await"));
        assert!(source.contains(".run()"));
    }

    #[test]
    fn renders_a_command_that_takes_only_a_flag() {
        let source = source_for(
            "#[singleton]\n#[console_command(name = \"flagged\")]\nstruct Flagged {\n    loud: bool,\n}\n\nimpl Flagged {\n    #[constructor]\n    fn create(#[console_argument(from = \"loud\")] loud: bool) -> anyhow::Result<Self> {}\n\n    #[process]\n    fn run(&self) -> anyhow::Result<CommandOutcome> {}\n}\n",
            false,
        );

        assert!(source.contains("super::container::build::construct_flagged("));
        assert!(source.contains(r#"matches.get_flag("loud")"#));
        assert!(source.contains("report_failure::report_failure(error"));
        assert!(source.contains(".run()"));
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
            "#[singleton]\n#[console_command(name = \"greet\")]\nstruct First;\n\nimpl First {\n    #[process]\n    fn run(&self) -> anyhow::Result<CommandOutcome> {}\n}\n\n#[singleton]\n#[console_command(name = \"greet\")]\nstruct Second;\n\nimpl Second {\n    #[process]\n    fn run(&self) -> anyhow::Result<CommandOutcome> {}\n}\n",
        );

        assert!(message.contains("already registered"));
    }

    #[test]
    fn rejects_a_console_command_that_injects_the_spiffe_http_client() {
        let message = error_for(
            "use reqwest::Client;\n\n#[singleton]\nstruct OutboundCaller {\n    client: Client,\n}\n\nimpl OutboundCaller {\n    #[constructor]\n    fn create(#[spiffe_http_client] client: Client) -> anyhow::Result<Self> {}\n}\n\n#[singleton]\n#[console_command(name = \"call\")]\nstruct Call {\n    caller: std::sync::Arc<OutboundCaller>,\n}\n\nimpl Call {\n    #[constructor]\n    fn create(caller: std::sync::Arc<OutboundCaller>) -> anyhow::Result<Self> {}\n\n    #[process]\n    fn run(&self) -> anyhow::Result<CommandOutcome> {}\n}\n",
        );

        assert!(message.contains("injects the #[spiffe_http_client]"));
    }

    #[test]
    fn rejects_a_console_command_that_depends_on_the_url_of_a_route() {
        let index = IndexedSource::new(
            "#[singleton]\n#[console_command(name = \"sign\")]\nstruct Sign {\n    flow: std::sync::Arc<margaret::framework::oidc_sign_in::sign_in_flow::SignInFlow>,\n}\n\nimpl Sign {\n    #[constructor]\n    fn create(flow: std::sync::Arc<margaret::framework::oidc_sign_in::sign_in_flow::SignInFlow>) -> anyhow::Result<Self> {}\n\n    #[process]\n    fn run(&self) -> anyhow::Result<CommandOutcome> {}\n}\n",
        )
        .index;
        let registry = scan(&index).expect("the console arguments are scanned");
        let bindings = render_container(
            &index,
            &registry,
            &[FrameworkProvider {
                construction: FrameworkConstruction::Constructor {
                    dependencies: vec![FrameworkDependency::RouteUrl(RouteUrlInput {
                        path: "/sign-in/callback".to_string(),
                        server: "public".to_string(),
                    })],
                    is_async: false,
                    method: "create".to_string(),
                    outcome: ConstructorOutcome::Infallible,
                },
                enablement: FrameworkEnablement::WhenReferenced,
                injection: FrameworkInjectionRole::Unmarked,
                provided: CanonicalPath::new(
                    [
                        "margaret",
                        "framework",
                        "oidc_sign_in",
                        "sign_in_flow",
                        "SignInFlow",
                    ]
                    .map(str::to_string)
                    .to_vec(),
                ),
            }],
            &DeclaredPostgresDatabase::Absent,
            &DeclaredTokenIssuance::Absent,
        )
        .expect("the container renders")
        .bindings;

        assert_eq!(
            ConsolePlan::build(&index, &bindings)
                .err()
                .expect("the command is rejected")
                .to_string(),
            "console command 'crate::Sign' depends on the url of a route, which is composed only while serving; a console command cannot use it"
        );
    }

    #[test]
    fn propagates_malformed_command_arguments() {
        let message = error_for("#[console_command(= 5)]\nstruct Bad;\n");

        assert!(message.contains("failed to index"));
    }

    #[test]
    fn rejects_a_non_string_command_name() {
        let error = rejection_for("#[console_command(name = 5)]\nstruct Bad;\n");

        assert!(matches!(
            error,
            ConsoleCodegenError::AttributeArguments {
                source: AttributeArgumentsError::UnexpectedArgument {
                    ref key,
                    ref expected,
                    ..
                }
            } if key == "name" && expected == "string literal"
        ));
    }

    #[test]
    fn rejects_a_non_string_command_description() {
        let error =
            rejection_for("#[console_command(name = \"x\", description = 5)]\nstruct Bad;\n");

        assert!(matches!(
            error,
            ConsoleCodegenError::AttributeArguments {
                source: AttributeArgumentsError::UnexpectedArgument {
                    ref key,
                    ref expected,
                    ..
                }
            } if key == "description" && expected == "string literal"
        ));
    }

    #[test]
    fn dispatches_a_fieldless_command_without_a_constructor() {
        let source = source_for(
            "#[singleton]\n#[console_command(name = \"bare\")]\nstruct Bare;\n\nimpl Bare {\n    #[process]\n    fn run(&self) -> anyhow::Result<CommandOutcome> {}\n}\n",
            false,
        );

        assert!(source.contains(r#"Some(("bare",_matches))=>{"#));
        assert!(source.contains("super::container::build::construct_bare()"));
        assert!(!source.contains("super::container::build::construct_bare().await"));
        assert!(source.contains("report_failure::report_failure(error"));
        assert!(source.contains(".run()"));
    }

    #[test]
    fn reports_a_missing_command_runner() {
        let message = error_for("#[console_command(name = \"bad\")]\nstruct Bad;\n");

        assert!(message.contains("no #[process] method"));
    }

    #[test]
    fn rejects_a_non_token_runner_parameter() {
        let message = error_for(
            "#[console_command(name = \"bad\")]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn run(&self, value: String) -> anyhow::Result<CommandOutcome> {}\n}\n",
        );

        assert!(message.contains("may only take &self and an optional CancellationToken"));
    }

    #[test]
    fn rejects_a_request_binding_marker_on_a_runner_parameter() {
        let message = error_for(
            "use margaret::framework::http_validation::request_input::RequestInput;\n\nuse tokio_util::sync::CancellationToken;\n\n#[console_command(name = \"bad\")]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn run(&self, #[form_request(from = RequestInput::Query)] token: CancellationToken) -> anyhow::Result<CommandOutcome> {}\n}\n",
        );

        assert!(message.contains("carries #[form_request]"));
        assert!(message.contains("only available in an HTTP responder"));
    }

    #[test]
    fn binds_a_destructured_positional_console_argument() {
        let source = source_for(
            "struct Point {\n    x: i32,\n    y: i32,\n}\n\n#[singleton]\n#[console_command(name = \"plot\")]\nstruct Plot {\n    point: Point,\n}\n\nimpl Plot {\n    #[constructor]\n    fn create(#[console_argument(positional)] Point { x, y }: Point) -> anyhow::Result<Self> {}\n\n    #[process]\n    fn run(&self) -> anyhow::Result<CommandOutcome> {}\n}\n",
            false,
        );

        assert!(source.contains(r#"clap::Arg::new("argument_0").required(true)"#));
        assert!(source.contains(r#"matches.get_one::<crate::Point>("argument_0")"#));
    }
}
