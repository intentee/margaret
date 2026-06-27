use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::crate_root::CrateRoot;

use crate::codegen_error::CodegenError;
use crate::generated_code::GeneratedCode;
use crate::generated_module::GeneratedModule;

pub fn build(crates: &[CrateRoot]) -> Result<GeneratedCode, CodegenError> {
    let mut builder = AttributeIndexBuilder::new();

    for crate_root in crates {
        builder = builder.index_crate(crate_root)?;
    }

    let index = builder.build();
    let container_is_async = margaret_container::container_is_async::container_is_async(&index)?;

    let has_http = margaret_http_codegen::has_responders::has_responders(&index);
    let has_services = margaret_service_codegen::has_services::has_services(&index);
    let serves = has_http || has_services;
    let has_console = margaret_console_codegen::has_commands::has_commands(&index) || serves;

    let mut modules = vec![GeneratedModule::new(
        "container",
        margaret_container::render_container::render_container(&index)?.source(),
    )];

    if has_http {
        modules.push(GeneratedModule::new(
            "http",
            margaret_http_codegen::render_http::render_http(&index, container_is_async)?,
        ));
    }

    if serves {
        modules.push(GeneratedModule::new(
            "services",
            margaret_service_codegen::render_services::render_services(
                &index,
                has_http,
                container_is_async,
            )?,
        ));
    }

    if has_console {
        modules.push(GeneratedModule::new(
            "console",
            margaret_console_codegen::render_console::render_console(
                &index,
                serves,
                has_http,
                container_is_async,
            )?,
        ));
    }

    modules.push(GeneratedModule::new(
        "mod",
        render_umbrella(has_http, has_console, serves),
    ));

    Ok(GeneratedCode::new(modules))
}

fn render_umbrella(has_http: bool, has_console: bool, serves: bool) -> String {
    let mut umbrella = String::from("#[rustfmt::skip]\npub mod container;\n");

    if has_http {
        umbrella.push_str("#[rustfmt::skip]\npub mod http;\n");
    }

    if serves {
        umbrella.push_str("#[rustfmt::skip]\npub mod services;\n");
    }

    if has_console {
        umbrella.push_str("#[rustfmt::skip]\npub mod console;\n");
    }

    umbrella
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;
    use tempfile::tempdir;

    use margaret_attributes::crate_root::CrateRoot;

    use super::build;
    use crate::codegen_error::CodegenError;
    use crate::generated_code::GeneratedCode;

    const WEB_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

#[singleton]
#[responds_to_http(method = Get, path = \"/x\")]
struct Page;

impl Page {
    #[responder]
    fn respond(&self) -> Response {}
}
";

    const PLAIN_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

#[singleton]
struct Config;

impl Config {
    #[constructor]
    fn create() -> Self {}
}
";

    const COMMAND_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

#[singleton]
#[console_command(name = \"greet\")]
struct Greet;

impl Greet {
    #[runner]
    fn run(&self) -> CommandOutcome {}
}
";

    fn write_lib(directory: &TempDir, lib_source: &str) {
        let source_directory = directory.path().join("src");

        fs::create_dir_all(&source_directory).expect("the src directory exists");
        fs::write(source_directory.join("lib.rs"), lib_source).expect("lib.rs is written");
    }

    fn crate_with(lib_source: &str) -> TempDir {
        let directory = tempdir().expect("a temporary crate directory is created");

        write_lib(&directory, lib_source);

        directory
    }

    fn bootstrap(directory: &TempDir) {
        let generated_directory = directory.path().join("src/margaret");

        fs::create_dir_all(&generated_directory).expect("the generated directory exists");
        fs::write(generated_directory.join("mod.rs"), "").expect("the umbrella stub is written");
    }

    fn generate(lib_source: &str) -> Result<GeneratedCode, CodegenError> {
        let directory = crate_with(lib_source);

        bootstrap(&directory);

        build(&[CrateRoot::new("crate", directory.path().join("src"))])
    }

    fn module<'code>(code: &'code GeneratedCode, name: &str) -> &'code str {
        code.modules()
            .iter()
            .find(|module| module.name() == name)
            .expect("the requested module is generated")
            .source()
    }

    fn has_module(code: &GeneratedCode, name: &str) -> bool {
        code.modules().iter().any(|module| module.name() == name)
    }

    #[test]
    fn generates_container_and_server_when_responders_exist() {
        let code = generate(WEB_CRATE).expect("the build succeeds");

        assert!(module(&code, "mod").contains("pub mod container;"));
        assert!(module(&code, "mod").contains("pub mod http;"));
        assert!(module(&code, "mod").contains("pub mod console;"));
        assert!(module(&code, "http").contains("use super::container::Container"));
        assert!(module(&code, "http").contains("fn server"));
        assert!(module(&code, "container").contains("struct Container"));
        assert!(module(&code, "console").contains("\"serve\""));
    }

    #[test]
    fn generates_a_console_for_commands_without_http() {
        let code = generate(COMMAND_CRATE).expect("the build succeeds");

        assert!(module(&code, "mod").contains("pub mod console;"));
        assert!(!module(&code, "mod").contains("pub mod http;"));
        assert!(!has_module(&code, "http"));
        assert!(module(&code, "console").contains("\"greet\""));
    }

    #[test]
    fn generates_only_container_without_responders() {
        let code = generate(PLAIN_CRATE).expect("the build succeeds");

        assert!(module(&code, "mod").contains("pub mod container;"));
        assert!(!module(&code, "mod").contains("pub mod http;"));
        assert!(!module(&code, "mod").contains("pub mod console;"));
        assert!(!has_module(&code, "http"));
        assert!(!has_module(&code, "console"));
        assert!(module(&code, "container").contains("struct Container"));
    }

    #[test]
    fn removes_the_server_when_responders_are_removed() {
        let directory = crate_with(WEB_CRATE);
        bootstrap(&directory);
        let source = directory.path().join("src");
        let generated = source.join("margaret");

        build(&[CrateRoot::new("crate", &source)])
            .expect("the first build succeeds")
            .write_to(&generated);

        assert!(generated.join("http.rs").exists());

        write_lib(&directory, PLAIN_CRATE);

        build(&[CrateRoot::new("crate", &source)])
            .expect("the second build succeeds")
            .write_to(&generated);

        let umbrella = fs::read_to_string(generated.join("mod.rs")).expect("the umbrella exists");

        assert!(!generated.join("http.rs").exists());
        assert!(!generated.join("console.rs").exists());
        assert!(!umbrella.contains("pub mod http;"));
        assert!(!umbrella.contains("pub mod console;"));
    }

    #[test]
    fn is_idempotent() {
        let directory = crate_with(WEB_CRATE);
        bootstrap(&directory);
        let source = directory.path().join("src");
        let generated = source.join("margaret");

        build(&[CrateRoot::new("crate", &source)])
            .expect("the first build succeeds")
            .write_to(&generated);

        let umbrella = fs::read_to_string(generated.join("mod.rs")).expect("the umbrella exists");
        let http = fs::read_to_string(generated.join("http.rs")).expect("the http source exists");
        let container =
            fs::read_to_string(generated.join("container.rs")).expect("the container exists");

        build(&[CrateRoot::new("crate", &source)])
            .expect("the second build succeeds")
            .write_to(&generated);

        assert_eq!(
            fs::read_to_string(generated.join("mod.rs")).expect("the umbrella exists"),
            umbrella
        );
        assert_eq!(
            fs::read_to_string(generated.join("http.rs")).expect("the http source exists"),
            http
        );
        assert_eq!(
            fs::read_to_string(generated.join("container.rs")).expect("the container exists"),
            container
        );
    }

    #[test]
    fn propagates_an_index_failure() {
        let message = generate("use other::*;\n")
            .expect_err("the build fails")
            .to_string();

        assert!(message.contains("failed to index"));
    }

    #[test]
    fn propagates_a_container_failure() {
        let message =
            generate("#[rustfmt::skip]\npub mod margaret;\n\n#[singleton]\nenum Bad {}\n")
                .expect_err("the build fails")
                .to_string();

        assert!(message.contains("failed to generate the dependency container"));
    }

    #[test]
    fn propagates_an_http_failure() {
        let message = generate(
            "#[rustfmt::skip]\npub mod margaret;\n\n#[singleton]\n#[responds_to_http(path = \"/x\")]\nstruct Bad;\n\nimpl Bad {\n    #[constructor]\n    fn create() -> Self {}\n}\n",
        )
        .expect_err("the build fails")
        .to_string();

        assert!(message.contains("missing the 'method'"));
    }

    #[test]
    fn propagates_a_console_failure() {
        let message = generate(
            "#[rustfmt::skip]\npub mod margaret;\n\n#[singleton]\nstruct Config;\n\nimpl Config {\n    #[constructor]\n    fn create() -> Self {}\n}\n\n#[console_command(name = \"bad\")]\nenum Bad {}\n",
        )
        .expect_err("the build fails")
        .to_string();

        assert!(message.contains("failed to generate the console"));
    }

    #[test]
    fn propagates_a_services_failure() {
        let message = generate("#[rustfmt::skip]\npub mod margaret;\n\n#[service]\nstruct Bad;\n")
            .expect_err("the build fails")
            .to_string();

        assert!(message.contains("failed to generate the services"));
    }

    #[test]
    fn propagates_a_dependency_cycle_failure() {
        let message = generate(
            "#[rustfmt::skip]\npub mod margaret;\n\n#[singleton]\nstruct A;\n\nimpl A {\n    #[constructor]\n    fn create(b: Arc<B>) -> Self {}\n}\n\n#[singleton]\nstruct B;\n\nimpl B {\n    #[constructor]\n    fn create(a: Arc<A>) -> Self {}\n}\n",
        )
        .expect_err("the build fails")
        .to_string();

        assert!(message.contains("dependency cycle"));
    }

    #[test]
    fn wires_singletons_from_an_explicitly_scanned_crate() {
        let host = crate_with(
            "#[rustfmt::skip]\npub mod margaret;\n\n#[singleton]\nstruct App;\n\nimpl App {\n    #[constructor]\n    fn create(metrics: Arc<margaret_plugin::Metrics>) -> Self {}\n}\n",
        );
        bootstrap(&host);
        let plugin = tempdir().expect("a plugin crate directory is created");
        let plugin_source = plugin.path().join("src");

        fs::create_dir_all(&plugin_source).expect("the plugin src directory exists");
        fs::write(
            plugin_source.join("lib.rs"),
            "#[singleton]\nstruct Metrics;\n\nimpl Metrics {\n    #[constructor]\n    fn create() -> Self {}\n}\n",
        )
        .expect("the plugin lib is written");

        let code = build(&[
            CrateRoot::new("crate", host.path().join("src")),
            CrateRoot::new("margaret_plugin", plugin_source),
        ])
        .expect("the build succeeds across crates");

        assert!(module(&code, "container").contains("margaret_plugin::Metrics"));
        assert!(module(&code, "container").contains("crate::App"));
    }

    #[test]
    fn generates_an_async_application_when_a_constructor_is_async() {
        let code = generate(
            "#[rustfmt::skip]\npub mod margaret;\n\n#[singleton]\nstruct Pool;\n\nimpl Pool {\n    #[constructor]\n    async fn create() -> Self {}\n}\n\n#[singleton]\n#[responds_to_http(method = Get, path = \"/x\")]\nstruct Page;\n\nimpl Page {\n    #[responder]\n    fn respond(&self) -> Response {}\n}\n",
        )
        .expect("the async build succeeds");

        assert!(module(&code, "http").contains("async fn server"));
        assert!(module(&code, "services").contains("super::http::server(container).await"));
        assert!(
            module(&code, "console")
                .contains("super::services::serve(container, matches, cancellation_token)")
        );
    }
}
