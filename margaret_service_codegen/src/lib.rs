pub mod framework_service;
pub mod framework_service_kind;
pub mod render_services;
pub mod service_codegen_error;
pub mod service_plan;

mod serve_inputs;
mod service_kind;
mod service_unit;
mod service_unit_origin;
mod service_units;
mod spiffe_activation;
mod tick_timer_arguments;

#[cfg(test)]
mod tests {
    use margaret_server_codegen::assemble_servers::assemble_servers;
    use margaret_server_codegen::server_contribution::ServerContribution;
    use margaret_server_codegen::server_name::ServerName;
    use margaret_server_codegen::server_route_source::ServerRouteSource;
    use margaret_server_codegen::server_transport_requirement::ServerTransportRequirement;
    use crate::service_codegen_error::ServiceCodegenError;
    use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
    use std::fs;

    fn negotiable(name: &str) -> ServerContribution {
        ServerContribution {
            routes: ServerRouteSource::Http,
            server: ServerName::parse(name.to_string()).expect("the fixture name is snake_case"),
            transport_requirement: ServerTransportRequirement::Negotiable,
        }
    }

    fn pinned(name: &str) -> ServerContribution {
        ServerContribution {
            routes: ServerRouteSource::Http,
            server: ServerName::parse(name.to_string()).expect("the fixture name is snake_case"),
            transport_requirement: ServerTransportRequirement::VerifiedPeerIdentity,
        }
    }

    use tempfile::TempDir;
    use tempfile::tempdir;

    use margaret_attributes::attribute_index::AttributeIndex;
    use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_attributes::crate_root::CrateRoot;
    use margaret_attributes::framework_attribute::FrameworkAttribute;
    use margaret_console_argument_codegen::console_argument::ConsoleArgument;
    use margaret_container::container_bindings::ContainerBindings;
    use margaret_container::render_container::render_container;
    use margaret_server_codegen::http_server::HttpServer;
    use margaret_serve_input_codegen::scan::scan;
    use margaret_serve_input_codegen::serve_input::ServeInput;

    use crate::framework_service::FrameworkService;
    use crate::framework_service_kind::FrameworkServiceKind;
    use crate::render_services::render_services;
    use crate::service_plan::ServicePlan;

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
            .expect("the container is rendered")
            .bindings
    }

    fn serve_roots(index: &AttributeIndex) -> Vec<CanonicalPath> {
        let mut roots = Vec::new();

        for framework_attribute in [
            FrameworkAttribute::Service,
            FrameworkAttribute::ScheduledWithTickTimer,
            FrameworkAttribute::RespondsToHttp,
            FrameworkAttribute::RendersView,
        ] {
            for matched in index.select_framework_attribute(framework_attribute) {
                roots.push(matched.item().canonical_path().clone());
            }
        }

        roots
    }

    fn render_source(lib_source: &str, servers: &[HttpServer], has_views: bool) -> String {
        let index = index_for(lib_source);
        let bindings = bindings(&index);
        let serve_inputs = bindings
            .serve_inputs(&serve_roots(&index), &[])
            .expect("the rendered roots have planned console arguments");

        let plan = ServicePlan::build(&index, &[], &bindings, &serve_inputs)
            .expect("the service construction is planned");

        render_services(&plan, servers, has_views, &bindings)
            .format()
            .expect("the module formats")
            .source()
            .split_whitespace()
            .collect()
    }

    fn rendered(lib_source: &str, servers: &[HttpServer]) -> String {
        render_source(lib_source, servers, false)
    }

    fn rendered_with_views(lib_source: &str, servers: &[HttpServer]) -> String {
        render_source(lib_source, servers, true)
    }

    fn rejection_for(lib_source: &str) -> ServiceCodegenError {
        let placeholder = bindings(&index_for("#[singleton]\nstruct Placeholder;\n"));

        let index = index_for(lib_source);
        ServicePlan::build(&index, &[], &placeholder, &[])
            .err()
            .expect("the services source fails to generate")
    }

    fn error_for(lib_source: &str) -> String {
        rejection_for(lib_source).to_string()
    }

    fn public() -> Vec<HttpServer> {
        assemble_servers(&[negotiable("public")])
    }

    const SERVICE: &str = "use tokio_util::sync::CancellationToken;\n\n#[service]\nstruct Pump;\n\nimpl Pump {\n    #[process]\n    fn run(&self, token: CancellationToken) -> anyhow::Result<()> {}\n}\n";
    const STATELESS_RESPONDER: &str = "#[singleton]\n#[responds_to_http(method = \"get\", path = \"/health\", server = \"public\")]\nstruct Health;\n\nimpl Health {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n";
    const TICKER: &str = "#[scheduled_with_tick_timer(interval = crate::schedule::PERIOD, behavior = tokio::time::MissedTickBehavior::Delay)]\nstruct Flusher;\n\nimpl Flusher {\n    #[process]\n    fn run(&self) -> anyhow::Result<()> {}\n}\n";
    const SPIFFE_CLIENT: &str = "use tokio_util::sync::CancellationToken;\n\n#[singleton]\nstruct OutboundCaller {\n    client: reqwest::Client,\n}\n\nimpl OutboundCaller {\n    #[constructor]\n    fn create(#[spiffe_http_client] client: reqwest::Client) -> anyhow::Result<Self> {}\n}\n\n#[service]\nstruct Worker {\n    caller: std::sync::Arc<OutboundCaller>,\n}\n\nimpl Worker {\n    #[constructor]\n    fn create(caller: std::sync::Arc<OutboundCaller>) -> anyhow::Result<Self> {}\n\n    #[process]\n    fn run(&self, token: CancellationToken) -> anyhow::Result<()> {}\n}\n";

    #[test]
    fn invokes_server_and_views_helpers_directly_when_the_container_has_no_accessors() {
        let source = rendered_with_views(STATELESS_RESPONDER, &public());

        assert!(
            source.contains(
                "routes:super::http::server_public::server_public(container,&routes,&views"
            )
        );
        assert!(
            source
                .contains("letviews=::std::sync::Arc::new(super::views::build::build(container));")
        );
        assert!(source.contains("super::container::build::serve("));
    }

    #[test]
    fn renders_a_service_adapter_that_passes_the_token() {
        let source = rendered(SERVICE, &[]);

        assert!(source.contains("structPump{"));
        assert!(source.contains(
            "letoutcome:margaret::framework::anyhow::Result<()>=self.inner.run(cancellation_token);outcome"
        ));
        assert!(source.contains("manager.register_service(Pump{inner:container.pump()})"));
    }

    #[test]
    fn renders_a_ticker_adapter_with_interval_and_behavior() {
        let source = rendered(TICKER, &[]);

        assert!(source.contains("structFlusher{"));
        assert!(source.contains("impltrzcina::TickerforFlusher"));
        assert!(
            source.contains("fntick_interval(&self)->std::time::Duration{crate::schedule::PERIOD}")
        );
        assert!(source.contains(
            "fnmissed_tick_behavior(&self)->tokio::time::MissedTickBehavior{tokio::time::MissedTickBehavior::Delay}"
        ));
        assert!(source.contains("_tick_context:trzcina::TickContext"));
        assert!(source.contains(
            "letoutcome:margaret::framework::anyhow::Result<()>=self.inner.run();outcome"
        ));
    }

    #[test]
    fn canonicalizes_imported_ticker_paths_before_rendering() {
        let source = rendered(
            r"
mod schedule {}

use crate::schedule as cadence;
use tokio::time::MissedTickBehavior as Behavior;

#[scheduled_with_tick_timer(
    interval = cadence::PERIOD,
    behavior = Behavior::Delay
)]
struct Flusher;

impl Flusher {
    #[process]
    fn run(&self) -> anyhow::Result<()> {}
}
",
            &[],
        );

        assert!(
            source.contains("fntick_interval(&self)->std::time::Duration{crate::schedule::PERIOD}")
        );
        assert!(source.contains(
            "fnmissed_tick_behavior(&self)->tokio::time::MissedTickBehavior{tokio::time::MissedTickBehavior::Delay}"
        ));
    }

    #[test]
    fn rejects_an_unresolvable_ticker_interval_path() {
        let message = error_for(
            "#[scheduled_with_tick_timer(interval = PERIOD)]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn run(&self) -> anyhow::Result<()> {}\n}\n",
        );

        assert!(message.contains("'interval' path 'PERIOD'"));
        assert!(message.contains("cannot be resolved"));
    }

    #[test]
    fn rejects_an_unresolvable_ticker_behavior_path() {
        let message = error_for(
            "#[scheduled_with_tick_timer(interval = crate::PERIOD, behavior = Delay)]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn run(&self) -> anyhow::Result<()> {}\n}\n",
        );

        assert!(message.contains("'behavior' path 'Delay'"));
        assert!(message.contains("cannot be resolved"));
    }

    fn canonical(segments: &[&str]) -> CanonicalPath {
        CanonicalPath::new(
            segments
                .iter()
                .map(|segment| (*segment).to_string())
                .collect(),
        )
    }

    #[test]
    fn renders_framework_ticker_and_service_adapters_beside_user_units() {
        let index = index_for("#[singleton]\nstruct Placeholder;\n");
        let framework_services = vec![
            FrameworkService {
                concrete_path: canonical(&[
                    "margaret",
                    "framework",
                    "jwks_roller_server",
                    "jwks_roller",
                    "JwksRoller",
                ]),
                field_name: "framework_jwks_roller_server_jwks_roller_jwks_roller".to_string(),
                is_async: false,
                kind: FrameworkServiceKind::Ticker {
                    interval: canonical(&[
                        "margaret",
                        "framework",
                        "jwks_roller_server",
                        "jwks_roll_interval",
                        "JWKS_ROLL_INTERVAL",
                    ]),
                },
                runner: "run".to_string(),
                takes_token: false,
                type_name: "JwksRoller".to_string(),
            },
            FrameworkService {
                concrete_path: canonical(&[
                    "margaret",
                    "framework",
                    "jwks_client",
                    "jwks_client",
                    "JwksClient",
                ]),
                field_name: "framework_jwks_client_jwks_client_jwks_client".to_string(),
                is_async: true,
                kind: FrameworkServiceKind::Service,
                runner: "run".to_string(),
                takes_token: true,
                type_name: "JwksClient".to_string(),
            },
        ];
        let container_bindings = bindings(&index);
        let plan = ServicePlan::build(&index, &framework_services, &container_bindings, &[])
            .expect("the framework services are planned");
        let source: String = render_services(&plan, &public(), false, &container_bindings)
            .format()
            .expect("the module formats")
            .source()
            .split_whitespace()
            .collect();

        assert!(source.contains("impltrzcina::TickerforJwksRoller"));
        assert!(source.contains(
            "fntick_interval(&self)->std::time::Duration{margaret::framework::jwks_roller_server::jwks_roll_interval::JWKS_ROLL_INTERVAL}"
        ));
        assert!(source.contains(
            "manager.register_service(JwksRoller{inner:container.framework_jwks_roller_server_jwks_roller_jwks_roller(),});"
        ));
        assert!(source.contains("impltrzcina::ServiceforJwksClient"));
        assert!(
            source.contains(
                "self.inner.run(cancellation_token).await?;::std::result::Result::Ok(())"
            )
        );
        assert!(source.contains(
            "manager.register_service(JwksClient{inner:container.framework_jwks_client_jwks_client_jwks_client(),});"
        ));
    }

    #[test]
    fn renders_a_ticker_that_passes_the_token() {
        let source = rendered(
            "use tokio_util::sync::CancellationToken;\n\n#[scheduled_with_tick_timer(interval = crate::P)]\nstruct Beat;\n\nimpl Beat {\n    #[process]\n    fn run(&self, token: CancellationToken) -> anyhow::Result<()> {}\n}\n",
            &[],
        );

        assert!(source.contains(
            "letoutcome:margaret::framework::anyhow::Result<()>=self.inner.run(cancellation_token);outcome"
        ));
        assert!(!source.contains("_cancellation_token"));
    }

    #[test]
    fn renders_a_ticker_without_a_behavior() {
        let source = rendered(
            "#[scheduled_with_tick_timer(interval = crate::PERIOD)]\nstruct T;\n\nimpl T {\n    #[process]\n    fn run(&self) -> anyhow::Result<()> {}\n}\n",
            &[],
        );

        assert!(source.contains("fntick_interval(&self)->std::time::Duration{crate::PERIOD}"));
        assert!(!source.contains("missed_tick_behavior"));
        assert!(!source.contains("impltrzcina::Servicefor"));
    }

    #[test]
    fn renders_a_service_without_a_token() {
        let source = rendered(
            "#[service]\nstruct Idle;\n\nimpl Idle {\n    #[process]\n    fn run(&self) -> anyhow::Result<()> {}\n}\n",
            &[],
        );

        assert!(source.contains("_cancellation_token:tokio_util::sync::CancellationToken"));
        assert!(source.contains(
            "letoutcome:margaret::framework::anyhow::Result<()>=self.inner.run();outcome"
        ));
    }

    #[test]
    fn awaits_a_service_that_declares_an_asynchronous_runner() {
        let source = rendered(
            "#[service]\nstruct Idle;\n\nimpl Idle {\n    #[process]\n    async fn run(&self) -> anyhow::Result<()> {}\n}\n",
            &[],
        );

        assert!(source.contains(
            "letoutcome:margaret::framework::anyhow::Result<()>=self.inner.run().await;outcome"
        ));
    }

    #[test]
    fn awaits_a_ticker_that_declares_an_asynchronous_runner() {
        let source = rendered(
            "use tokio_util::sync::CancellationToken;\n\n#[scheduled_with_tick_timer(interval = crate::P)]\nstruct Beat;\n\nimpl Beat {\n    #[process]\n    async fn run(&self, token: CancellationToken) -> anyhow::Result<()> {}\n}\n",
            &[],
        );

        assert!(source.contains(
            "letoutcome:margaret::framework::anyhow::Result<()>=self.inner.run(cancellation_token).await;outcome"
        ));
    }

    #[test]
    fn registers_the_http_server_when_responders_exist() {
        let source = rendered(SERVICE, &public());

        assert!(source.contains(r#"matches.get_one::<String>("public-url")"#));
        assert!(source.contains(
            r#"margaret::framework::service::server_assembly::ServerAssembly{address_argument:"public-addr",name:"public",routes:super::http::server_public::server_public(container,"#
        ));
        assert!(source.contains(
            r#"transport:margaret::framework::http::transport_config::TransportConfig::Plain,upload_dir_argument:"public-upload-dir",uploads_argument:"public-uploads","#
        ));
        assert!(source.contains(
            "letserver_services=margaret::framework::service::serve_application::serve_application(matches,servers,)?;"
        ));
        assert!(source.contains(
            "forserver_serviceinserver_services{manager.register_service(server_service);}"
        ));
        assert!(source.contains("manager.register_service(Pump{"));
        assert!(source.contains("inner:container.pump()"));
        assert!(!source.contains("bundle_services"));
        assert!(!source.contains("resolved_services"));
        assert!(source.contains("letmutmanager=trzcina::ServiceManager::default();"));
    }

    #[test]
    fn builds_one_mutable_manager_for_a_units_only_application() {
        let source = rendered(SERVICE, &[]);

        assert_eq!(
            source
                .matches("letmutmanager=trzcina::ServiceManager::default();")
                .count(),
            1
        );
        assert!(source.contains("manager.register_service(Pump{"));
        assert!(source.contains("inner:container.pump()"));
        assert!(!source.contains("serve_application"));
    }

    #[test]
    fn rejects_a_serve_input_absent_from_the_container_plan() {
        let index = index_for("");
        let bindings = bindings(&index);
        let serve_inputs = [ServeInput::ConsoleArgument(ConsoleArgument::Flag {
            name: "missing".to_string(),
        })];
        let error = ServicePlan::build(&index, &[], &bindings, &serve_inputs)
            .err()
            .expect("every serve input must belong to the container plan");

        assert!(error.to_string().contains("missing"));
    }

    #[test]
    fn wires_the_spiffe_bundle_and_pins_transports_when_a_server_is_pinned() {
        let source = rendered(
            SERVICE,
            &assemble_servers(&[pinned("internal"), negotiable("public")]),
        );

        assert!(source.contains(
            "margaret::framework::spiffe_svid::install_default_crypto_provider::install_default_crypto_provider();"
        ));
        assert!(source.contains(
            "margaret::framework::spiffe_svid_server::svid_server_bundle::SvidServerBundle::new(margaret::framework::spiffe_svid::svid_service_bundle_params::SvidServiceBundleParams{"
        ));
        assert!(source.contains(r#"matches.get_one::<String>("spiffe-trust-domain")"#));
        assert!(source.contains(r#"matches.get_one::<String>("spire-agent-addr")"#));
        assert!(source.contains(
            "letspiffe_server_config=::std::sync::Arc::new(spiffe_bundle.server_config());"
        ));
        assert!(source.contains(
            "transport:margaret::framework::http::transport_config::TransportConfig::MutualTls{server_config:::std::sync::Arc::clone(spiffe_server_config),}"
        ));
        assert!(source.contains(
            r#"transport:matchmatches.get_one::<String>("public-transport").map(String::as_str)"#
        ));
        assert!(source.contains(
            r#"Some("spiffe_mtls")=>{margaret::framework::http::transport_config::TransportConfig::MutualTls{server_config:::std::sync::Arc::clone(spiffe_server_config),}}"#
        ));
        assert!(source.contains(
            r#"Some("plain")=>{margaret::framework::http::transport_config::TransportConfig::Plain}"#
        ));
        assert!(source.contains(
            "Some(_)|None=>{return::std::result::Result::Err(margaret::framework::console::command_outcome::CommandOutcome::Failed,);}"
        ));
        assert!(source.contains(
            "ifletErr(error)=manager.register_bundle(spiffe_bundle).await{returnmargaret::framework::console::report_failure::report_failure(error);}"
        ));
        assert!(source.contains(
            "letserver_services=margaret::framework::service::serve_application::serve_application(matches,servers,)?;"
        ));
        assert!(source.contains(
            "forserver_serviceinserver_services{manager.register_service(server_service);}"
        ));
        assert!(!source.contains("bundle_services"));
    }

    #[test]
    fn wires_the_client_bundle_when_a_spiffe_http_client_is_injected() {
        let source = rendered(SPIFFE_CLIENT, &[]);

        assert!(source.contains(
            "letspiffe_bundle=margaret::framework::spiffe_svid_client::svid_client_bundle::SvidClientBundle::new(margaret::framework::spiffe_svid::svid_service_bundle_params::SvidServiceBundleParams{"
        ));
        assert!(source.contains("letspiffe_client_readiness=spiffe_bundle.client_readiness();"));
        assert!(source.contains(
            "letspiffe_http_client=matchspiffe_bundle.reqwest_client(){Ok(client)=>client,Err(error)=>{returnmargaret::framework::console::report_failure::report_failure(error);}};"
        ));
        assert!(source.contains(
            "ifletErr(error)=manager.register_bundle(spiffe_bundle).await{returnmargaret::framework::console::report_failure::report_failure(error);}"
        ));
        assert!(source.contains(
            "manager.register_service(margaret::framework::spiffe_svid_client::readiness_gated_service::ReadinessGatedService::new(spiffe_client_readiness.clone(),Worker{inner:container.worker(),},),);"
        ));
        assert!(source.contains(
            "margaret::framework::service::run::run(manager,cancellation_token,trzcina::ServiceShutdownOptions::default(),).await"
        ));
        assert!(!source.contains("run_all"));
        assert!(!source.contains("spiffe_identity_manager"));
        assert!(!source.contains("wait_until_ready"));
        assert!(!source.contains("serve_application"));
        assert!(!source.contains("spiffe_server_config"));
    }

    #[test]
    fn shares_the_svid_bundle_when_a_pinned_server_and_a_client_are_active() {
        let source = rendered(
            SPIFFE_CLIENT,
            &assemble_servers(&[pinned("internal")]),
        );

        assert!(source.contains(
            "letspiffe_bundle=margaret::framework::spiffe_svid_bundle::svid_bundle::SvidBundle::new(margaret::framework::spiffe_svid::svid_service_bundle_params::SvidServiceBundleParams{"
        ));
        assert!(source.contains(
            "letspiffe_server_config=::std::sync::Arc::new(spiffe_bundle.server_config());"
        ));
        assert!(source.contains(
            "letspiffe_http_client=matchspiffe_bundle.reqwest_client(){Ok(client)=>client,Err(error)=>{returnmargaret::framework::console::report_failure::report_failure(error);}};"
        ));
        assert!(source.contains(
            "ifletErr(error)=manager.register_bundle(spiffe_bundle).await{returnmargaret::framework::console::report_failure::report_failure(error);}"
        ));
        assert!(source.contains(
            "forserver_serviceinserver_services{manager.register_service(margaret::framework::spiffe_svid_client::readiness_gated_service::ReadinessGatedService::new(spiffe_client_readiness.clone(),server_service,),);}"
        ));
        assert!(source.contains(
            "margaret::framework::service::run::run(manager,cancellation_token,trzcina::ServiceShutdownOptions::default(),).await"
        ));
        assert!(!source.contains("run_all"));
    }

    #[test]
    fn builds_the_routes_once_and_weaves_them_into_the_server_builders() {
        let source = rendered(
            SERVICE,
            &assemble_servers(&[negotiable("internal"), negotiable("public")]),
        );

        assert!(source.contains(
            r#"letorigin_public:::std::sync::Arc<str>=matchmatches.get_one::<String>("public-url"){Some(value)=>value.clone().into(),None=>{return::std::result::Result::Err(margaret::framework::console::command_outcome::CommandOutcome::Failed,);}};"#
        ));
        assert!(source.contains(
            "letroutes=::std::sync::Arc::new(super::routes::Routes::from_origins(origin_internal.clone(),origin_public.clone(),),);"
        ));
        assert!(source.contains("super::http::server_internal::server_internal(container,"));
        assert!(source.contains("super::http::server_public::server_public(container,"));
    }

    #[test]
    fn registers_one_server_service_per_active_server() {
        let source = rendered(
            SERVICE,
            &assemble_servers(&[negotiable("public"), negotiable("internal")]),
        );

        assert!(source.contains(
            r#"margaret::framework::service::server_assembly::ServerAssembly{address_argument:"public-addr",name:"public",routes:super::http::server_public::server_public(container,"#
        ));
        assert!(source.contains(
            r#"upload_dir_argument:"public-upload-dir",uploads_argument:"public-uploads","#
        ));
        assert!(source.contains(
            r#"margaret::framework::service::server_assembly::ServerAssembly{address_argument:"internal-addr",name:"internal",routes:super::http::server_internal::server_internal(container,"#
        ));
        assert!(source.contains(
            r#"upload_dir_argument:"internal-upload-dir",uploads_argument:"internal-uploads","#
        ));
        assert!(source.contains(
            "margaret::framework::service::serve_application::serve_application(matches,servers,"
        ));
    }

    #[test]
    fn rejects_a_ticker_conflicting_with_a_command() {
        let message = error_for(
            "#[scheduled_with_tick_timer(interval = crate::P)]\n#[console_command(name = \"x\")]\nstruct Bad;\n",
        );

        assert!(message.contains("mutually exclusive"));
    }

    #[test]
    fn rejects_a_non_path_interval() {
        let error = rejection_for(
            "#[scheduled_with_tick_timer(interval = 5)]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn run(&self) -> anyhow::Result<()> {}\n}\n",
        );

        assert!(matches!(
            error,
            ServiceCodegenError::AttributeArguments {
                source: AttributeArgumentsError::UnexpectedArgument {
                    ref key,
                    ref expected,
                    ..
                }
            } if key == "interval" && expected == "path"
        ));
    }

    #[test]
    fn rejects_a_non_path_behavior() {
        let error = rejection_for(
            "#[scheduled_with_tick_timer(interval = crate::P, behavior = 5)]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn run(&self) -> anyhow::Result<()> {}\n}\n",
        );

        assert!(matches!(
            error,
            ServiceCodegenError::AttributeArguments {
                source: AttributeArgumentsError::UnexpectedArgument {
                    ref key,
                    ref expected,
                    ..
                }
            } if key == "behavior" && expected == "path"
        ));
    }

    #[test]
    fn rejects_a_service_on_a_non_struct() {
        assert!(error_for("#[service]\nenum Bad {}\n").contains("#[service]"));
    }

    #[test]
    fn rejects_a_ticker_on_a_non_struct() {
        assert!(
            error_for("#[scheduled_with_tick_timer(interval = crate::P)]\nenum Bad {}\n")
                .contains("#[scheduled_with_tick_timer]")
        );
    }

    #[test]
    fn rejects_conflicting_roles() {
        let message = error_for(
            "#[service]\n#[scheduled_with_tick_timer(interval = crate::P)]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn run(&self) -> anyhow::Result<()> {}\n}\n",
        );

        assert!(message.contains("mutually exclusive"));
    }

    #[test]
    fn rejects_a_unit_without_a_runner() {
        assert!(error_for("#[service]\nstruct Bad;\n").contains("no #[process] method"));
    }

    #[test]
    fn rejects_an_ambiguous_runner() {
        let message = error_for(
            "#[service]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn a(&self) -> anyhow::Result<()> {}\n    #[process]\n    fn b(&self) -> anyhow::Result<()> {}\n}\n",
        );

        assert!(message.contains("more than one #[process]"));
    }

    #[test]
    fn rejects_a_ticker_without_an_interval() {
        let message = error_for(
            "#[scheduled_with_tick_timer]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn run(&self) -> anyhow::Result<()> {}\n}\n",
        );

        assert!(message.contains("missing the 'interval'"));
    }

    #[test]
    fn rejects_an_unmarked_runner_parameter() {
        let message = error_for(
            "#[service]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn run(&self, value: String) -> anyhow::Result<()> {}\n}\n",
        );

        assert!(message.contains("takes parameter 'value'"));
        assert!(message.contains("a runner may only take &self and an optional CancellationToken"));
    }

    #[test]
    fn rejects_a_request_binding_marker_on_a_runner_parameter() {
        let message = error_for(
            "use tokio_util::sync::CancellationToken;\n\n#[service]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn run(&self, #[authenticated_user] token: CancellationToken) -> anyhow::Result<()> {}\n}\n",
        );

        assert!(message.contains("carries #[authenticated_user]"));
        assert!(message.contains("only available in an HTTP responder"));
    }

    #[test]
    fn rejects_a_cancellation_token_passed_by_reference() {
        let message = error_for(
            "use tokio_util::sync::CancellationToken;\n\n#[service]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn run(&self, token: &CancellationToken) -> anyhow::Result<()> {}\n}\n",
        );

        assert!(message.contains("takes parameter 'token'"));
        assert!(message.contains("a runner may only take &self and an optional CancellationToken"));
    }

    #[test]
    fn weaves_a_serve_input_into_a_ticker() {
        let source = rendered(
            r#"use std::path::PathBuf;

#[scheduled_with_tick_timer(interval = crate::P)]
struct Roller {
    secret_path: PathBuf,
}

impl Roller {
    #[constructor]
    fn create(#[console_argument(from = "secret-path")] secret_path: PathBuf) -> anyhow::Result<Self> {}

    #[process]
    fn run(&self) -> anyhow::Result<()> {}
}
"#,
            &[],
        );

        assert!(source.contains("structRoller{inner:std::sync::Arc<crate::Roller>,}"));
        assert!(source.contains("impltrzcina::TickerforRoller"));
        assert!(source.contains(
            "letoutcome:margaret::framework::anyhow::Result<()>=self.inner.run();outcome"
        ));
        assert!(!source.contains("impltrzcina::Servicefor"));
        assert!(source.contains("manager.register_service(Roller{inner:container.roller(),});"));
        assert!(source.contains(
            r#"letserve_input_0=matchmatches.get_one::<std::path::PathBuf>("secret-path")"#
        ));
    }

    #[test]
    fn weaves_a_serve_input_and_the_token_into_a_ticker() {
        let source = rendered(
            r#"use std::path::PathBuf;
use tokio_util::sync::CancellationToken;

#[scheduled_with_tick_timer(interval = crate::P)]
struct Roller {
    secret_path: PathBuf,
}

impl Roller {
    #[constructor]
    fn create(#[console_argument(from = "secret-path")] secret_path: PathBuf) -> anyhow::Result<Self> {}

    #[process]
    fn run(&self, token: CancellationToken) -> anyhow::Result<()> {}
}
"#,
            &[],
        );

        assert!(source.contains(
            "letoutcome:margaret::framework::anyhow::Result<()>=self.inner.run(cancellation_token);outcome"
        ));
        assert!(!source.contains("_cancellation_token"));
        assert!(source.contains("manager.register_service(Roller{inner:container.roller(),});"));
    }

    #[test]
    fn weaves_a_serve_input_into_a_service_with_a_token() {
        let source = rendered(
            r#"use tokio_util::sync::CancellationToken;

#[service]
struct Worker {
    label: String,
}

impl Worker {
    #[constructor]
    fn create(#[console_argument(from = "label")] label: String) -> anyhow::Result<Self> {}

    #[process]
    fn run(&self, token: CancellationToken) -> anyhow::Result<()> {}
}
"#,
            &[],
        );

        assert!(source.contains(
            "letoutcome:margaret::framework::anyhow::Result<()>=self.inner.run(cancellation_token);outcome"
        ));
        assert!(source.contains("manager.register_service(Worker{inner:container.worker(),});"));
    }

    #[test]
    fn weaves_a_serve_input_into_a_service() {
        let source = rendered(
            r#"#[service]
struct Worker {
    label: String,
}

impl Worker {
    #[constructor]
    fn create(#[console_argument(from = "label")] label: String) -> anyhow::Result<Self> {}

    #[process]
    fn run(&self) -> anyhow::Result<()> {}
}
"#,
            &[],
        );

        assert!(source.contains("structWorker{inner:std::sync::Arc<crate::Worker>,}"));
        assert!(source.contains(
            "letoutcome:margaret::framework::anyhow::Result<()>=self.inner.run();outcome"
        ));
        assert!(source.contains("manager.register_service(Worker{inner:container.worker(),});"));
        assert!(
            source.contains(
                r#"letserve_input_0=matchmatches.get_one::<std::string::String>("label")"#
            )
        );
    }

    #[test]
    fn weaves_a_copy_serve_input_into_a_service_by_value() {
        let source = rendered(
            r#"#[service]
struct Watcher {
    verbose: bool,
}

impl Watcher {
    #[constructor]
    fn create(#[console_argument(from = "verbose")] verbose: bool) -> anyhow::Result<Self> {}

    #[process]
    fn run(&self) -> anyhow::Result<()> {}
}
"#,
            &[],
        );

        assert!(source.contains("manager.register_service(Watcher{inner:container.watcher(),});"));
        assert!(source.contains(r#"letserve_input_0=matches.get_flag("verbose")"#));
    }

    #[test]
    fn weaves_a_shared_serve_input_into_both_services_once() {
        let source = rendered(
            r#"#[service]
struct First {
    shared: String,
}

impl First {
    #[constructor]
    fn create(#[console_argument(from = "shared")] a: String) -> anyhow::Result<Self> {}

    #[process]
    fn run(&self) -> anyhow::Result<()> {}
}

#[service]
struct Second {
    shared: String,
}

impl Second {
    #[constructor]
    fn create(#[console_argument(from = "shared")] b: String) -> anyhow::Result<Self> {}

    #[process]
    fn run(&self) -> anyhow::Result<()> {}
}
"#,
            &[],
        );

        assert!(source.contains("structFirst"));
        assert!(source.contains("structSecond"));
        assert_eq!(source.matches("manager.register_service(").count(), 2);
        assert_eq!(source.matches("letserve_input_0=").count(), 1);
    }

    #[test]
    fn propagates_malformed_ticker_arguments() {
        let message = error_for(
            "#[scheduled_with_tick_timer(= 5)]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn run(&self) -> anyhow::Result<()> {}\n}\n",
        );

        assert!(message.contains("failed to index"));
    }
}
