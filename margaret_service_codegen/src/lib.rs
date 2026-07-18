pub mod has_services;
pub mod render_services;
pub mod rendered_services;
pub mod service_codegen_error;

mod service_kind;
mod service_unit;
mod service_units;
mod tick_timer_arguments;

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;
    use tempfile::tempdir;

    use margaret_attributes::attribute_index::AttributeIndex;
    use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
    use margaret_attributes::crate_root::CrateRoot;
    use margaret_http_codegen::http_server::HttpServer;
    use margaret_http_codegen::server_transport_policy::ServerTransportPolicy;

    use crate::has_services::has_services;
    use crate::render_services::render_services;

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

    fn rendered(lib_source: &str, servers: &[HttpServer]) -> String {
        render_services(&index_for(lib_source), servers)
            .expect("the services source is generated")
            .module
            .format()
            .source()
            .split_whitespace()
            .collect()
    }

    fn error_for(lib_source: &str) -> String {
        render_services(&index_for(lib_source), &[])
            .err()
            .expect("the services source fails to generate")
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
            r#"margaret_service::server_assembly::ServerAssembly{address_argument:"public-addr",name:"public",routes:super::http::server_public::server_public(container,"#
        ));
        assert!(source.contains(
            r#"transport:margaret_http::transport_config::TransportConfig::Plain,upload_dir_argument:"public-upload-dir",uploads_argument:"public-uploads","#
        ));
        assert!(source.contains(
            "margaret_service::serve_application::serve_application(matches,servers,margaret_service::resolved_services::ResolvedServices{services:bundle_services,},)"
        ));
        assert!(!source.contains("margaret_service::bundle_services::bundle_services"));
        assert!(!source.contains("letmutbundle_services"));
        assert!(source.contains("letbundle_services:"));
        assert!(source.contains("letmutmanager"));
    }

    #[test]
    fn declares_the_bundle_services_immutable_and_the_manager_immutable_without_units() {
        let source = rendered("#[singleton]\nstruct S;\n", &public());

        assert!(!source.contains("letmutbundle_services"));
        assert!(!source.contains("letmutmanager"));
        assert!(source.contains("letmanager=match"));
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
            "margaret_spiffe_svid::install_default_crypto_provider::install_default_crypto_provider();"
        ));
        assert!(source.contains(
            "margaret_spiffe_svid_server::SvidServerBundle::new(margaret_spiffe_svid::SvidServiceBundleParams{"
        ));
        assert!(source.contains(r#"matches.get_one::<String>("spiffe-trust-domain")"#));
        assert!(source.contains(r#"matches.get_one::<String>("spire-agent-addr")"#));
        assert!(source.contains(
            "letspiffe_server_config=::std::sync::Arc::new(spiffe_bundle.server_config());"
        ));
        assert!(source.contains(
            "transport:margaret_http::transport_config::TransportConfig::MutualTls{server_config:spiffe_server_config.clone(),}"
        ));
        assert!(source.contains(
            r#"transport:matchmatches.get_one::<String>("public-transport").map(String::as_str)"#
        ));
        assert!(source.contains(
            r#"Some("spiffe_mtls")=>{margaret_http::transport_config::TransportConfig::MutualTls{server_config:spiffe_server_config.clone(),}}"#
        ));
        assert!(source.contains("_=>margaret_http::transport_config::TransportConfig::Plain,"));
        assert!(source.contains(
            "matchmargaret_service::bundle_services::bundle_services(spiffe_bundle).await{Ok(services)=>bundle_services.extend(services),Err(outcome)=>returnoutcome,}"
        ));
        assert!(source.contains(
            "margaret_service::serve_application::serve_application(matches,servers,margaret_service::resolved_services::ResolvedServices{services:bundle_services,},)"
        ));
        assert!(source.contains("letmutbundle_services"));
    }

    #[test]
    fn builds_the_routes_once_and_threads_them_into_the_server_builders() {
        let source = rendered(
            SERVICE,
            &[
                HttpServer::new("internal".to_string(), ServerTransportPolicy::Negotiable),
                HttpServer::new("public".to_string(), ServerTransportPolicy::Negotiable),
            ],
        );

        assert!(source.contains(
            r#"letorigin_public:::std::sync::Arc<str>=matchmatches.get_one::<String>("public-url"){Some(value)=>value.clone().into(),None=>returnmargaret_console::command_outcome::CommandOutcome::Failed,};"#
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
            r#"margaret_service::server_assembly::ServerAssembly{address_argument:"public-addr",name:"public",routes:super::http::server_public::server_public(container,"#
        ));
        assert!(source.contains(
            r#"upload_dir_argument:"public-upload-dir",uploads_argument:"public-uploads","#
        ));
        assert!(source.contains(
            r#"margaret_service::server_assembly::ServerAssembly{address_argument:"internal-addr",name:"internal",routes:super::http::server_internal::server_internal(container,"#
        ));
        assert!(source.contains(
            r#"upload_dir_argument:"internal-upload-dir",uploads_argument:"internal-uploads","#
        ));
        assert!(
            source.contains(
                "margaret_service::serve_application::serve_application(matches,servers,"
            )
        );
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

        assert!(message.contains("must be a console argument"));
    }

    #[test]
    fn rejects_a_cancellation_token_passed_by_reference() {
        let message = error_for(
            "use tokio_util::sync::CancellationToken;\n\n#[service]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn run(&self, token: &CancellationToken) -> Result<(), Infallible> {}\n}\n",
        );

        assert!(message.contains("must be a console argument"));
    }

    #[test]
    fn threads_a_console_argument_into_a_ticker() {
        let source = rendered(
            "use std::path::PathBuf;\n\n#[scheduled_with_tick_timer(interval = crate::P)]\nstruct Roller;\n\nimpl Roller {\n    #[process]\n    fn run(&self, #[console_argument(from = \"secret-path\")] secret_path: PathBuf) -> Result<(), Infallible> {}\n}\n",
            &[],
        );

        assert!(
            source
                .contains("structRoller{inner:std::sync::Arc<crate::Roller>,argument_0:PathBuf,}")
        );
        assert!(source.contains("impltrzcina::TickerforRoller"));
        assert!(source.contains(
            "self.inner.run(self.argument_0.clone()).await.map_err(anyhow::Error::from)"
        ));
        assert!(!source.contains("impltrzcina::Servicefor"));
        assert!(source.contains(r#"argument_0:matchmatches.get_one::<PathBuf>("secret-path")"#));
    }

    #[test]
    fn threads_a_console_argument_and_the_token_into_a_ticker() {
        let source = rendered(
            "use std::path::PathBuf;\nuse tokio_util::sync::CancellationToken;\n\n#[scheduled_with_tick_timer(interval = crate::P)]\nstruct Roller;\n\nimpl Roller {\n    #[process]\n    fn run(&self, #[console_argument(from = \"secret-path\")] secret_path: PathBuf, token: CancellationToken) -> Result<(), Infallible> {}\n}\n",
            &[],
        );

        assert!(source.contains(
            "self.inner.run(self.argument_0.clone(),cancellation_token).await.map_err(anyhow::Error::from)"
        ));
        assert!(!source.contains("_cancellation_token"));
    }

    #[test]
    fn threads_a_console_argument_into_a_service_with_a_token() {
        let source = rendered(
            "use tokio_util::sync::CancellationToken;\n\n#[service]\nstruct Worker;\n\nimpl Worker {\n    #[process]\n    fn run(&self, #[console_argument(from = \"label\")] label: String, token: CancellationToken) -> Result<(), Infallible> {}\n}\n",
            &[],
        );

        assert!(source.contains("letWorker{inner,argument_0}=*self;"));
        assert!(source.contains("inner.run(argument_0,cancellation_token).await?;Ok(())"));
    }

    #[test]
    fn threads_a_console_argument_into_a_service() {
        let source = rendered(
            "#[service]\nstruct Worker;\n\nimpl Worker {\n    #[process]\n    fn run(&self, #[console_argument(from = \"label\")] label: String) -> Result<(), Infallible> {}\n}\n",
            &[],
        );

        assert!(
            source.contains("structWorker{inner:std::sync::Arc<crate::Worker>,argument_0:String,}")
        );
        assert!(
            source
                .contains("letWorker{inner,argument_0}=*self;inner.run(argument_0).await?;Ok(())")
        );
    }

    #[test]
    fn rejects_two_components_declaring_the_same_console_argument() {
        let message = error_for(
            "#[service]\nstruct First;\n\nimpl First {\n    #[process]\n    fn run(&self, #[console_argument(from = \"shared\")] a: String) -> Result<(), Infallible> {}\n}\n\n#[service]\nstruct Second;\n\nimpl Second {\n    #[process]\n    fn run(&self, #[console_argument(from = \"shared\")] b: String) -> Result<(), Infallible> {}\n}\n",
        );

        assert!(message.contains("already declared"));
    }

    #[test]
    fn propagates_malformed_ticker_arguments() {
        let message = error_for(
            "#[scheduled_with_tick_timer(= 5)]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn run(&self) -> Result<(), Infallible> {}\n}\n",
        );

        assert!(message.contains("failed to index"));
    }
}
