pub mod framework_service;
pub mod framework_service_kind;
pub mod has_services;
pub mod render_services;
pub mod serve_console_arguments;
pub mod service_codegen_error;
pub mod startup_singleton;

mod service_kind;
mod service_unit;
mod service_units;
mod spiffe_activation;
mod tick_timer_arguments;

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::fs;

    use tempfile::TempDir;
    use tempfile::tempdir;

    use margaret_attributes::attribute_index::AttributeIndex;
    use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
    use margaret_attributes::attribute_selector::AttributeSelector;
    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_attributes::crate_root::CrateRoot;
    use margaret_console_argument_codegen::scan::scan;
    use margaret_container::container_bindings::ContainerBindings;
    use margaret_container::render_container::render_container;
    use margaret_http_codegen::http_server::HttpServer;
    use margaret_http_codegen::server_transport_policy::ServerTransportPolicy;

    use crate::framework_service::FrameworkService;
    use crate::framework_service_kind::FrameworkServiceKind;
    use crate::has_services::has_services;
    use crate::render_services::render_services;
    use crate::serve_console_arguments::ServeConsoleArguments;

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

        for marker in [
            "service",
            "scheduled_with_tick_timer",
            "responds_to_http",
            "renders_view",
        ] {
            for matched in index.select(&AttributeSelector::from_marker(marker)) {
                roots.push(matched.item().canonical_path().clone());
            }
        }

        roots
    }

    fn render_source(lib_source: &str, servers: &[HttpServer], has_views: bool) -> String {
        let index = index_for(lib_source);
        let bindings = bindings(&index);
        let serve_arguments = bindings
            .serve_arguments(&serve_roots(&index), &[])
            .expect("the serve arguments unify");

        render_services(
            &index,
            servers,
            has_views,
            &bindings,
            ServeConsoleArguments {
                serve_arguments: &serve_arguments,
                server_console_arguments: &BTreeMap::new(),
                views_console_arguments: &[],
            },
            &[],
            &[],
        )
        .expect("the services source is generated")
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

    fn error_for(lib_source: &str) -> String {
        let placeholder = bindings(&index_for("#[singleton]\nstruct Placeholder;\n"));

        render_services(
            &index_for(lib_source),
            &[],
            false,
            &placeholder,
            ServeConsoleArguments {
                serve_arguments: &[],
                server_console_arguments: &BTreeMap::new(),
                views_console_arguments: &[],
            },
            &[],
            &[],
        )
        .expect_err("the services source fails to generate")
        .to_string()
    }

    fn public() -> Vec<HttpServer> {
        vec![HttpServer::new(
            "public".to_string(),
            ServerTransportPolicy::Negotiable,
        )]
    }

    const SERVICE: &str = "use tokio_util::sync::CancellationToken;\n\n#[service]\nstruct Pump;\n\nimpl Pump {\n    #[process]\n    fn run(&self, token: CancellationToken) -> Result<(), Infallible> {}\n}\n";
    const TICKER: &str = "#[scheduled_with_tick_timer(interval = crate::schedule::PERIOD, behavior = tokio::time::MissedTickBehavior::Delay)]\nstruct Flusher;\n\nimpl Flusher {\n    #[process]\n    fn run(&self) -> Result<(), Infallible> {}\n}\n";
    const SPIFFE_CLIENT: &str = "use tokio_util::sync::CancellationToken;\n\n#[singleton]\nstruct OutboundCaller {\n    client: reqwest::Client,\n}\n\nimpl OutboundCaller {\n    #[constructor]\n    fn create(#[spiffe_http_client] client: reqwest::Client) -> Self {}\n}\n\n#[service]\nstruct Worker {\n    caller: std::sync::Arc<OutboundCaller>,\n}\n\nimpl Worker {\n    #[constructor]\n    fn create(caller: std::sync::Arc<OutboundCaller>) -> Self {}\n\n    #[process]\n    fn run(&self, token: CancellationToken) -> Result<(), Infallible> {}\n}\n";

    #[test]
    fn reports_units_present() {
        assert!(has_services(&index_for(SERVICE)));
        assert!(has_services(&index_for(TICKER)));
        assert!(!has_services(&index_for("#[singleton]\nstruct S;\n")));
    }

    #[test]
    fn renders_a_service_adapter_that_passes_the_token() {
        let source = rendered(SERVICE, &[]);

        assert!(source.contains("structPump{"));
        assert!(source.contains("self.inner.run(cancellation_token).await?;Ok(())"));
        assert!(source.contains("manager.register_service(Pump{inner:container.pump().await"));
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
        assert!(source.contains("self.inner.run().await.map_err(anyhow::Error::from)"));
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
                kind: FrameworkServiceKind::Service,
                runner: "run".to_string(),
                takes_token: true,
                type_name: "JwksClient".to_string(),
            },
        ];
        let source: String = render_services(
            &index,
            &public(),
            false,
            &bindings(&index),
            ServeConsoleArguments {
                serve_arguments: &[],
                server_console_arguments: &BTreeMap::new(),
                views_console_arguments: &[],
            },
            &framework_services,
            &[],
        )
        .expect("the services source is generated")
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
            "manager.register_service(JwksRoller{inner:container.framework_jwks_roller_server_jwks_roller_jwks_roller().await"
        ));
        assert!(source.contains("impltrzcina::ServiceforJwksClient"));
        assert!(source.contains("self.inner.run(cancellation_token).await?;Ok(())"));
        assert!(source.contains(
            "manager.register_service(JwksClient{inner:container.framework_jwks_client_jwks_client_jwks_client().await"
        ));
    }

    #[test]
    fn renders_a_ticker_that_passes_the_token() {
        let source = rendered(
            "use tokio_util::sync::CancellationToken;\n\n#[scheduled_with_tick_timer(interval = crate::P)]\nstruct Beat;\n\nimpl Beat {\n    #[process]\n    fn run(&self, token: CancellationToken) -> Result<(), Infallible> {}\n}\n",
            &[],
        );

        assert!(
            source
                .contains("self.inner.run(cancellation_token).await.map_err(anyhow::Error::from)")
        );
        assert!(!source.contains("_cancellation_token"));
    }

    #[test]
    fn renders_a_ticker_without_a_behavior() {
        let source = rendered(
            "#[scheduled_with_tick_timer(interval = crate::PERIOD)]\nstruct T;\n\nimpl T {\n    #[process]\n    fn run(&self) -> Result<(), Infallible> {}\n}\n",
            &[],
        );

        assert!(source.contains("fntick_interval(&self)->std::time::Duration{crate::PERIOD}"));
        assert!(!source.contains("missed_tick_behavior"));
        assert!(!source.contains("impltrzcina::Servicefor"));
    }

    #[test]
    fn renders_a_service_without_a_token() {
        let source = rendered(
            "#[service]\nstruct Idle;\n\nimpl Idle {\n    #[process]\n    fn run(&self) -> Result<(), Infallible> {}\n}\n",
            &[],
        );

        assert!(source.contains("_cancellation_token:tokio_util::sync::CancellationToken"));
        assert!(source.contains("self.inner.run().await?;Ok(())"));
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
            "margaret::framework::service::serve_application::serve_application(matches,servers,margaret::framework::service::resolved_services::ResolvedServices{services:bundle_services,},)"
        ));
        assert!(!source.contains("margaret::framework::service::bundle_services::bundle_services"));
        assert!(!source.contains("letmutbundle_services"));
        assert!(source.contains("letbundle_services:"));
        assert!(source.contains("letmutmanager"));
    }

    #[test]
    fn binds_the_manager_immutably_when_no_services_are_registered() {
        let source = rendered("#[singleton]\nstruct Store;\n", &public());

        assert!(source.contains(
            "letmanager=matchmargaret::framework::service::serve_application::serve_application("
        ));
        assert!(!source.contains("letmutmanager"));
    }

    #[test]
    fn builds_views_once_and_weaves_them_into_each_server() {
        let source = rendered_with_views("#[singleton]\nstruct Store;\n", &public());

        assert!(source.contains(
            "letviews=::std::sync::Arc::new(super::views::build::build(container).await);"
        ));
        assert!(
            source.contains(
                "super::http::server_public::server_public(container,&routes,&views).await"
            )
        );
    }

    #[test]
    fn wires_the_spiffe_bundle_and_pins_transports_when_a_server_is_pinned() {
        let source = rendered(
            SERVICE,
            &[
                HttpServer::new(
                    "internal".to_string(),
                    ServerTransportPolicy::PinnedSpiffeMtls,
                ),
                HttpServer::new("public".to_string(), ServerTransportPolicy::Negotiable),
            ],
        );

        assert!(source.contains(
            "margaret::framework::spiffe_svid::install_default_crypto_provider::install_default_crypto_provider();"
        ));
        assert!(source.contains(
            "margaret::framework::spiffe_svid_server::SvidServerBundle::new(margaret::framework::spiffe_svid::SvidServiceBundleParams{"
        ));
        assert!(source.contains(r#"matches.get_one::<String>("spiffe-trust-domain")"#));
        assert!(source.contains(r#"matches.get_one::<String>("spire-agent-addr")"#));
        assert!(source.contains(
            "letspiffe_server_config=::std::sync::Arc::new(spiffe_bundle.server_config());"
        ));
        assert!(source.contains(
            "transport:margaret::framework::http::transport_config::TransportConfig::MutualTls{server_config:spiffe_server_config.clone(),}"
        ));
        assert!(source.contains(
            r#"transport:matchmatches.get_one::<String>("public-transport").map(String::as_str)"#
        ));
        assert!(source.contains(
            r#"Some("spiffe_mtls")=>{margaret::framework::http::transport_config::TransportConfig::MutualTls{server_config:spiffe_server_config.clone(),}}"#
        ));
        assert!(
            source.contains(
                "_=>margaret::framework::http::transport_config::TransportConfig::Plain,"
            )
        );
        assert!(source.contains(
            "matchmargaret::framework::service::bundle_services::bundle_services(spiffe_bundle).await{Ok(services)=>bundle_services.extend(services),Err(outcome)=>returnoutcome,}"
        ));
        assert!(source.contains(
            "margaret::framework::service::serve_application::serve_application(matches,servers,margaret::framework::service::resolved_services::ResolvedServices{services:bundle_services,},)"
        ));
        assert!(source.contains("letmutbundle_services"));
    }

    #[test]
    fn wires_the_client_bundle_when_a_spiffe_http_client_is_injected() {
        let source = rendered(SPIFFE_CLIENT, &[]);

        assert!(source.contains(
            "letspiffe_bundle=margaret::framework::spiffe_svid_client::SvidClientBundle::new(margaret::framework::spiffe_svid::SvidServiceBundleParams{"
        ));
        assert!(source.contains("letmutspiffe_client_readiness=spiffe_bundle.client_readiness();"));
        assert!(source.contains(
            "letspiffe_http_client=matchspiffe_bundle.reqwest_client(){Ok(client)=>client,Err(error)=>{returnmargaret::framework::console::report_failure::report_failure(error);}};"
        ));
        assert!(source.contains(
            "ifletErr(error)=spiffe_identity_manager.register_bundle(spiffe_bundle).await{returnmargaret::framework::console::report_failure::report_failure(error);}"
        ));
        assert!(source.contains(
            "margaret::framework::service::run_all::run_all(::std::vec![spiffe_identity_running,spiffe_application_running],trzcina::ServiceShutdownOptions::default(),).await"
        ));
        assert!(!source.contains("serve_application"));
        assert!(!source.contains("spiffe_server_config"));
    }

    #[test]
    fn shares_the_svid_bundle_when_a_pinned_server_and_a_client_are_active() {
        let source = rendered(
            SPIFFE_CLIENT,
            &[HttpServer::new(
                "internal".to_string(),
                ServerTransportPolicy::PinnedSpiffeMtls,
            )],
        );

        assert!(source.contains(
            "letspiffe_bundle=margaret::framework::spiffe_svid_bundle::SvidBundle::new(margaret::framework::spiffe_svid::SvidServiceBundleParams{"
        ));
        assert!(source.contains(
            "letspiffe_server_config=::std::sync::Arc::new(spiffe_bundle.server_config());"
        ));
        assert!(source.contains(
            "letspiffe_http_client=matchspiffe_bundle.reqwest_client(){Ok(client)=>client,Err(error)=>{returnmargaret::framework::console::report_failure::report_failure(error);}};"
        ));
    }

    #[test]
    fn builds_the_routes_once_and_weaves_them_into_the_server_builders() {
        let source = rendered(
            SERVICE,
            &[
                HttpServer::new("internal".to_string(), ServerTransportPolicy::Negotiable),
                HttpServer::new("public".to_string(), ServerTransportPolicy::Negotiable),
            ],
        );

        assert!(source.contains(
            r#"letorigin_public:::std::sync::Arc<str>=matchmatches.get_one::<String>("public-url"){Some(value)=>value.clone().into(),None=>{returnmargaret::framework::console::command_outcome::CommandOutcome::Failed;}};"#
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
            &[
                HttpServer::new("public".to_string(), ServerTransportPolicy::Negotiable),
                HttpServer::new("internal".to_string(), ServerTransportPolicy::Negotiable),
            ],
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
        let message = error_for(
            "#[scheduled_with_tick_timer(interval = 5)]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn run(&self) -> Result<(), Infallible> {}\n}\n",
        );

        assert!(message.contains("failed to index"));
    }

    #[test]
    fn rejects_a_non_path_behavior() {
        let message = error_for(
            "#[scheduled_with_tick_timer(interval = crate::P, behavior = 5)]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn run(&self) -> Result<(), Infallible> {}\n}\n",
        );

        assert!(message.contains("failed to index"));
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
            "#[service]\n#[scheduled_with_tick_timer(interval = crate::P)]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn run(&self) -> Result<(), Infallible> {}\n}\n",
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
            "#[service]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn a(&self) -> Result<(), Infallible> {}\n    #[process]\n    fn b(&self) -> Result<(), Infallible> {}\n}\n",
        );

        assert!(message.contains("more than one #[process]"));
    }

    #[test]
    fn rejects_a_ticker_without_an_interval() {
        let message = error_for(
            "#[scheduled_with_tick_timer]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn run(&self) -> Result<(), Infallible> {}\n}\n",
        );

        assert!(message.contains("missing the 'interval'"));
    }

    #[test]
    fn rejects_an_unmarked_runner_parameter() {
        let message = error_for(
            "#[service]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn run(&self, value: String) -> Result<(), Infallible> {}\n}\n",
        );

        assert!(message.contains("takes parameter 'value'"));
        assert!(message.contains("a runner may only take &self and an optional CancellationToken"));
    }

    #[test]
    fn rejects_a_request_binding_marker_on_a_runner_parameter() {
        let message = error_for(
            "use tokio_util::sync::CancellationToken;\n\n#[service]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn run(&self, #[authenticated_user] token: CancellationToken) -> Result<(), Infallible> {}\n}\n",
        );

        assert!(message.contains("carries #[authenticated_user]"));
        assert!(message.contains("only available in an HTTP responder"));
    }

    #[test]
    fn rejects_a_cancellation_token_passed_by_reference() {
        let message = error_for(
            "use tokio_util::sync::CancellationToken;\n\n#[service]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn run(&self, token: &CancellationToken) -> Result<(), Infallible> {}\n}\n",
        );

        assert!(message.contains("takes parameter 'token'"));
        assert!(message.contains("a runner may only take &self and an optional CancellationToken"));
    }

    #[test]
    fn weaves_a_console_argument_into_a_ticker() {
        let source = rendered(
            r#"use std::path::PathBuf;

#[scheduled_with_tick_timer(interval = crate::P)]
struct Roller {
    secret_path: PathBuf,
}

impl Roller {
    #[constructor]
    fn create(#[console_argument(from = "secret-path")] secret_path: PathBuf) -> Self {}

    #[process]
    fn run(&self) -> Result<(), Infallible> {}
}
"#,
            &[],
        );

        assert!(source.contains("structRoller{inner:std::sync::Arc<crate::Roller>,}"));
        assert!(source.contains("impltrzcina::TickerforRoller"));
        assert!(source.contains("self.inner.run().await.map_err(anyhow::Error::from)"));
        assert!(!source.contains("impltrzcina::Servicefor"));
        assert!(source.contains(
            "manager.register_service(Roller{inner:container.roller(console_argument_0.to_owned()).await,});"
        ));
        assert!(source.contains(
            r#"letconsole_argument_0=matchmatches.get_one::<std::path::PathBuf>("secret-path")"#
        ));
    }

    #[test]
    fn weaves_a_console_argument_and_the_token_into_a_ticker() {
        let source = rendered(
            r#"use std::path::PathBuf;
use tokio_util::sync::CancellationToken;

#[scheduled_with_tick_timer(interval = crate::P)]
struct Roller {
    secret_path: PathBuf,
}

impl Roller {
    #[constructor]
    fn create(#[console_argument(from = "secret-path")] secret_path: PathBuf) -> Self {}

    #[process]
    fn run(&self, token: CancellationToken) -> Result<(), Infallible> {}
}
"#,
            &[],
        );

        assert!(
            source
                .contains("self.inner.run(cancellation_token).await.map_err(anyhow::Error::from)")
        );
        assert!(!source.contains("_cancellation_token"));
        assert!(source.contains(
            "manager.register_service(Roller{inner:container.roller(console_argument_0.to_owned()).await,});"
        ));
    }

    #[test]
    fn weaves_a_console_argument_into_a_service_with_a_token() {
        let source = rendered(
            r#"use tokio_util::sync::CancellationToken;

#[service]
struct Worker {
    label: String,
}

impl Worker {
    #[constructor]
    fn create(#[console_argument(from = "label")] label: String) -> Self {}

    #[process]
    fn run(&self, token: CancellationToken) -> Result<(), Infallible> {}
}
"#,
            &[],
        );

        assert!(source.contains("self.inner.run(cancellation_token).await?;Ok(())"));
        assert!(source.contains(
            "manager.register_service(Worker{inner:container.worker(console_argument_0.to_owned()).await,});"
        ));
    }

    #[test]
    fn weaves_a_console_argument_into_a_service() {
        let source = rendered(
            r#"#[service]
struct Worker {
    label: String,
}

impl Worker {
    #[constructor]
    fn create(#[console_argument(from = "label")] label: String) -> Self {}

    #[process]
    fn run(&self) -> Result<(), Infallible> {}
}
"#,
            &[],
        );

        assert!(source.contains("structWorker{inner:std::sync::Arc<crate::Worker>,}"));
        assert!(source.contains("self.inner.run().await?;Ok(())"));
        assert!(source.contains(
            "manager.register_service(Worker{inner:container.worker(console_argument_0.to_owned()).await,});"
        ));
        assert!(source.contains(
            r#"letconsole_argument_0=matchmatches.get_one::<std::string::String>("label")"#
        ));
    }

    #[test]
    fn weaves_a_copy_console_argument_into_a_service_by_value() {
        let source = rendered(
            r#"#[service]
struct Watcher {
    verbose: bool,
}

impl Watcher {
    #[constructor]
    fn create(#[console_argument(from = "verbose")] verbose: bool) -> Self {}

    #[process]
    fn run(&self) -> Result<(), Infallible> {}
}
"#,
            &[],
        );

        assert!(source.contains(
            "manager.register_service(Watcher{inner:container.watcher(console_argument_0).await,});"
        ));
        assert!(source.contains(r#"letconsole_argument_0=matches.get_flag("verbose")"#));
    }

    #[test]
    fn weaves_a_shared_console_argument_into_both_services_once() {
        let source = rendered(
            r#"#[service]
struct First {
    shared: String,
}

impl First {
    #[constructor]
    fn create(#[console_argument(from = "shared")] a: String) -> Self {}

    #[process]
    fn run(&self) -> Result<(), Infallible> {}
}

#[service]
struct Second {
    shared: String,
}

impl Second {
    #[constructor]
    fn create(#[console_argument(from = "shared")] b: String) -> Self {}

    #[process]
    fn run(&self) -> Result<(), Infallible> {}
}
"#,
            &[],
        );

        assert!(source.contains(
            "manager.register_service(First{inner:container.first(console_argument_0.to_owned()).await,});"
        ));
        assert!(source.contains(
            "manager.register_service(Second{inner:container.second(console_argument_0.to_owned()).await,});"
        ));
        assert_eq!(source.matches("letconsole_argument_0=").count(), 1);
    }

    #[test]
    fn propagates_malformed_ticker_arguments() {
        let message = error_for(
            "#[scheduled_with_tick_timer(= 5)]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn run(&self) -> Result<(), Infallible> {}\n}\n",
        );

        assert!(message.contains("failed to index"));
    }
}
