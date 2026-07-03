pub mod has_services;
pub mod render_services;
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
            .split_whitespace()
            .collect()
    }

    fn error_for(lib_source: &str) -> String {
        render_services(&index_for(lib_source), &[])
            .expect_err("the services source fails to generate")
            .to_string()
    }

    fn public() -> Vec<HttpServer> {
        vec![HttpServer::new("public".to_string())]
    }

    const SERVICE: &str = "#[service]\nstruct Pump;\n\nimpl Pump {\n    #[runner]\n    fn run(&self, token: CancellationToken) -> Result<(), Infallible> {}\n}\n";
    const TICKER: &str = "#[scheduled_with_tick_timer(interval = crate::schedule::PERIOD, behavior = tokio::time::MissedTickBehavior::Delay)]\nstruct Flusher;\n\nimpl Flusher {\n    #[runner]\n    fn run(&self) -> Result<(), Infallible> {}\n}\n";

    #[test]
    fn reports_units_present() {
        assert!(has_services(&index_for(SERVICE)));
        assert!(has_services(&index_for(TICKER)));
        assert!(!has_services(&index_for("#[singleton]\nstruct S;\n")));
    }

    #[test]
    fn renders_a_service_adapter_that_passes_the_token() {
        let source = rendered(SERVICE, &[]);

        assert!(source.contains("structPumpService"));
        assert!(source.contains("self.inner.run(cancellation_token).await?;Ok(())"));
        assert!(
            source.contains("manager.register_service(PumpService{inner:container.pump().await")
        );
    }

    #[test]
    fn renders_a_ticker_adapter_with_interval_and_behavior() {
        let source = rendered(TICKER, &[]);

        assert!(source.contains("structFlusherTicker"));
        assert!(source.contains("letmutinterval=tokio::time::interval(crate::schedule::PERIOD);"));
        assert!(source.contains(
            "interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);"
        ));
        assert!(source.contains("_=interval.tick()=>{self.inner.run().await?;}"));
        assert!(source.contains("_=cancellation_token.cancelled()=>returnOk(())"));
    }

    #[test]
    fn renders_a_ticker_that_passes_the_token() {
        let source = rendered(
            "#[scheduled_with_tick_timer(interval = crate::P)]\nstruct Beat;\n\nimpl Beat {\n    #[runner]\n    fn run(&self, token: CancellationToken) -> Result<(), Infallible> {}\n}\n",
            &[],
        );

        assert!(source.contains("self.inner.run(cancellation_token.clone()).await?"));
    }

    #[test]
    fn renders_a_ticker_without_a_behavior() {
        let source = rendered(
            "#[scheduled_with_tick_timer(interval = crate::PERIOD)]\nstruct T;\n\nimpl T {\n    #[runner]\n    fn run(&self) -> Result<(), Infallible> {}\n}\n",
            &[],
        );

        assert!(source.contains("tokio::time::interval(crate::PERIOD)"));
        assert!(!source.contains("set_missed_tick_behavior"));
    }

    #[test]
    fn renders_a_service_without_a_token() {
        let source = rendered(
            "#[service]\nstruct Idle;\n\nimpl Idle {\n    #[runner]\n    fn run(&self) -> Result<(), Infallible> {}\n}\n",
            &[],
        );

        assert!(source.contains("_cancellation_token:tokio_util::sync::CancellationToken"));
        assert!(source.contains("self.inner.run().await?;Ok(())"));
    }

    #[test]
    fn registers_the_http_server_when_responders_exist() {
        let source = rendered(SERVICE, &public());

        assert!(source.contains(r#"matches.get_one::<String>("public-addr")"#));
        assert!(source.contains(r#"matches.get_flag("public-uploads")"#));
        assert!(source.contains(r#"matches.get_one::<String>("public-upload-dir")"#));
        assert!(source.contains("unwrap_or_else(std::env::temp_dir)"));
        assert!(source.contains("margaret_http::upload_config::UploadConfig::Disabled"));
        assert!(source.contains(
            "margaret_service::server_service::ServerService::new(super::http::server_public(container).await,address_public,upload_config_public,)"
        ));
    }

    #[test]
    fn registers_one_server_service_per_active_server() {
        let source = rendered(
            SERVICE,
            &[
                HttpServer::new("public".to_string()),
                HttpServer::new("internal".to_string()),
            ],
        );

        assert!(source.contains(r#"matches.get_one::<String>("public-addr")"#));
        assert!(source.contains(r#"matches.get_flag("public-uploads")"#));
        assert!(source.contains(r#"matches.get_one::<String>("public-upload-dir")"#));
        assert!(source.contains(
            "margaret_service::server_service::ServerService::new(super::http::server_public(container).await,address_public,upload_config_public,)"
        ));
        assert!(source.contains(r#"matches.get_one::<String>("internal-addr")"#));
        assert!(source.contains(r#"matches.get_flag("internal-uploads")"#));
        assert!(source.contains(r#"matches.get_one::<String>("internal-upload-dir")"#));
        assert!(source.contains(
            "margaret_service::server_service::ServerService::new(super::http::server_internal(container).await,address_internal,upload_config_internal,)"
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
            "#[scheduled_with_tick_timer(interval = 5)]\nstruct Bad;\n\nimpl Bad {\n    #[runner]\n    fn run(&self) -> Result<(), Infallible> {}\n}\n",
        );

        assert!(message.contains("failed to index"));
    }

    #[test]
    fn rejects_a_non_path_behavior() {
        let message = error_for(
            "#[scheduled_with_tick_timer(interval = crate::P, behavior = 5)]\nstruct Bad;\n\nimpl Bad {\n    #[runner]\n    fn run(&self) -> Result<(), Infallible> {}\n}\n",
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
            "#[service]\n#[scheduled_with_tick_timer(interval = crate::P)]\nstruct Bad;\n\nimpl Bad {\n    #[runner]\n    fn run(&self) -> Result<(), Infallible> {}\n}\n",
        );

        assert!(message.contains("mutually exclusive"));
    }

    #[test]
    fn rejects_a_unit_without_a_runner() {
        assert!(error_for("#[service]\nstruct Bad;\n").contains("no #[runner] method"));
    }

    #[test]
    fn rejects_an_ambiguous_runner() {
        let message = error_for(
            "#[service]\nstruct Bad;\n\nimpl Bad {\n    #[runner]\n    fn a(&self) -> Result<(), Infallible> {}\n    #[runner]\n    fn b(&self) -> Result<(), Infallible> {}\n}\n",
        );

        assert!(message.contains("more than one #[runner]"));
    }

    #[test]
    fn rejects_a_ticker_without_an_interval() {
        let message = error_for(
            "#[scheduled_with_tick_timer]\nstruct Bad;\n\nimpl Bad {\n    #[runner]\n    fn run(&self) -> Result<(), Infallible> {}\n}\n",
        );

        assert!(message.contains("missing the 'interval'"));
    }

    #[test]
    fn rejects_an_unexpected_runner_parameter() {
        let message = error_for(
            "#[service]\nstruct Bad;\n\nimpl Bad {\n    #[runner]\n    fn run(&self, value: String) -> Result<(), Infallible> {}\n}\n",
        );

        assert!(message.contains("unsupported"));
    }

    #[test]
    fn propagates_malformed_ticker_arguments() {
        let message = error_for(
            "#[scheduled_with_tick_timer(= 5)]\nstruct Bad;\n\nimpl Bad {\n    #[runner]\n    fn run(&self) -> Result<(), Infallible> {}\n}\n",
        );

        assert!(message.contains("failed to index"));
    }
}
