use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::crate_root::CrateRoot;
use margaret_container::provided_singleton::ProvidedSingleton;

use crate::build_context::BuildContext;
use crate::codegen_error::CodegenError;
use crate::console_pass::console_pass;
use crate::container_pass::container_pass;
use crate::generated_code::GeneratedCode;
use crate::http_pass::http_pass;
use crate::model_pass::model_pass;
use crate::services_pass::services_pass;
use crate::views_pass::views_pass;
use crate::websocket_pass::websocket_pass;

pub fn build(
    crate_root: &CrateRoot,
    provided_singletons: Vec<ProvidedSingleton>,
) -> Result<GeneratedCode, CodegenError> {
    let index = AttributeIndexBuilder::new()
        .index_crate(crate_root)?
        .build();
    let mut context = BuildContext::new(&index, provided_singletons);

    let bindings = container_pass(&mut context)?;
    websocket_pass(&mut context, &bindings)?;
    views_pass(&mut context)?;
    http_pass(&mut context)?;
    services_pass(&mut context)?;
    model_pass(&mut context)?;
    console_pass(&mut context)?;

    context.into_generated_code()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use tempfile::TempDir;
    use tempfile::tempdir;

    use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
    use margaret_attributes::crate_root::CrateRoot;
    use margaret_generated_module::generated_module::GeneratedModule;

    use super::build;
    use crate::build_context::BuildContext;
    use crate::codegen_error::CodegenError;
    use crate::console_pass::console_pass;
    use crate::container_pass::container_pass;
    use crate::generated_code::GeneratedCode;
    use crate::http_pass::http_pass;
    use crate::model_pass::model_pass;
    use crate::services_pass::services_pass;
    use crate::umbrella::umbrella;
    use crate::views_pass::views_pass;

    const WEB_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

#[singleton]
#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]
struct Page;

impl Page {
    #[process]
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
    #[process]
    fn run(&self) -> CommandOutcome {}
}
";

    const MODELS_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

#[model(table = \"widgets\")]
struct Widget {
    #[column(primary_key, name = \"id\")]
    id: uuid::Uuid,
}
";

    const WEBSOCKET_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

use margaret_websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;

trait Clock {}

#[singleton(provides = Clock)]
struct SystemClock;

impl SystemClock {
    #[constructor]
    fn create() -> Self {}
}

impl Clock for SystemClock {}

#[websocket_session(path = \"/room/{name}\", server = \"public\")]
struct Room;

impl Room {
    #[build_for_session]
    fn build(clock: std::sync::Arc<dyn Clock>, #[route_parameter(from = \"name\")] name: String) -> Self {}
}

#[websocket_message(request, method = \"chat\", response = single)]
struct Chat;

#[singleton]
struct Chatter;

impl RespondsToWebSocketMessage for Chatter {
    type Session = Room;
    type Message = Chat;
}
";

    const INVALID_WEBSOCKET_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

#[websocket_session(path = \"/x\", server = \"public\")]
struct Room;
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

        build(&CrateRoot::new("crate", directory.path().join("src")), Vec::new())
    }

    fn generate_unformatted(source: &Path) -> GeneratedCode {
        let index = AttributeIndexBuilder::new()
            .index_crate(&CrateRoot::new("crate", source))
            .expect("the crate is indexed")
            .build();
        let mut context = BuildContext::new(&index, Vec::new());

        container_pass(&mut context).expect("the container pass succeeds");
        views_pass(&mut context).expect("the views pass succeeds");
        http_pass(&mut context).expect("the http pass succeeds");
        services_pass(&mut context).expect("the services pass succeeds");
        model_pass(&mut context).expect("the model pass succeeds");
        console_pass(&mut context).expect("the console pass succeeds");

        let mut modules: Vec<GeneratedModule> = context
            .module_tokens()
            .iter()
            .map(|module| GeneratedModule::new(module.name(), module.to_source()))
            .collect();

        modules.push(umbrella(context.capabilities()));

        GeneratedCode::new(modules)
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

    fn concatenated(code: &GeneratedCode) -> String {
        code.modules()
            .iter()
            .map(|module| module.source())
            .collect::<Vec<&str>>()
            .join("\n")
    }

    #[test]
    fn generates_container_and_server_when_responders_exist() {
        let code = generate(WEB_CRATE).expect("the build succeeds");

        assert!(module(&code, "mod").contains("pub mod container;"));
        assert!(module(&code, "mod").contains("pub mod http;"));
        assert!(module(&code, "mod").contains("pub mod routes;"));
        assert!(module(&code, "mod").contains("pub mod run;"));
        assert!(concatenated(&code).contains("container: &super::super::container::Container"));
        assert!(concatenated(&code).contains("async fn server"));
        assert!(module(&code, "routes").contains("pub struct Routes"));
        assert!(module(&code, "container").contains("struct Container"));
        assert!(module(&code, "run").contains("\"serve\""));
    }

    #[test]
    fn generates_a_console_for_commands_without_http() {
        let code = generate(COMMAND_CRATE).expect("the build succeeds");

        assert!(module(&code, "mod").contains("pub mod run;"));
        assert!(!module(&code, "mod").contains("pub mod http;"));
        assert!(!has_module(&code, "http"));
        assert!(module(&code, "run").contains("\"greet\""));
    }

    #[test]
    fn generates_only_container_without_responders() {
        let code = generate(PLAIN_CRATE).expect("the build succeeds");

        assert!(module(&code, "mod").contains("pub mod container;"));
        assert!(!module(&code, "mod").contains("pub mod http;"));
        assert!(!module(&code, "mod").contains("pub mod routes;"));
        assert!(!module(&code, "mod").contains("pub mod run;"));
        assert!(!has_module(&code, "http"));
        assert!(!has_module(&code, "routes"));
        assert!(!has_module(&code, "run"));
        assert!(module(&code, "container").contains("struct Container"));
    }

    #[test]
    fn generates_the_websocket_module_for_websocket_sessions() {
        let code = generate(WEBSOCKET_CRATE).expect("the build succeeds");

        assert!(module(&code, "mod").contains("pub mod websocket;"));
        assert!(module(&code, "mod").contains("pub mod http;"));
        assert!(has_module(&code, "websocket"));
        assert!(concatenated(&code).contains("WebSocketRequestMessage"));
        assert!(concatenated(&code).contains("public_routes"));
    }

    #[test]
    fn propagates_a_websocket_codegen_error() {
        let error = generate(INVALID_WEBSOCKET_CRATE).expect_err("the invalid session is rejected");

        assert!(error.to_string().contains("failed to generate the websockets"));
    }

    #[test]
    fn removes_the_server_when_responders_are_removed() {
        let directory = crate_with(WEB_CRATE);
        bootstrap(&directory);
        let source = directory.path().join("src");
        let generated = source.join("margaret");

        generate_unformatted(&source)
            .write_to(&generated)
            .expect("the first sources are written");

        assert!(generated.join("http.rs").exists());

        write_lib(&directory, PLAIN_CRATE);

        generate_unformatted(&source)
            .write_to(&generated)
            .expect("the second sources are written");

        let umbrella_source =
            fs::read_to_string(generated.join("mod.rs")).expect("the umbrella exists");

        assert!(!generated.join("http.rs").exists());
        assert!(!generated.join("routes.rs").exists());
        assert!(!generated.join("run.rs").exists());
        assert!(!umbrella_source.contains("pub mod http;"));
        assert!(!umbrella_source.contains("pub mod routes;"));
        assert!(!umbrella_source.contains("pub mod run;"));
    }

    #[test]
    fn is_idempotent() {
        let directory = crate_with(WEB_CRATE);
        bootstrap(&directory);
        let source = directory.path().join("src");

        let first = generate_unformatted(&source);
        let second = generate_unformatted(&source);

        assert_eq!(module(&first, "mod"), module(&second, "mod"));
        assert_eq!(module(&first, "http"), module(&second, "http"));
        assert_eq!(module(&first, "routes"), module(&second, "routes"));
        assert_eq!(module(&first, "container"), module(&second, "container"));
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

    const VIEWS_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

#[singleton]
#[renders_view(name = \"card\")]
struct Card;

#[singleton]
#[responds_to_http(method = \"get\", path = \"/card\", server = \"public\")]
struct GetCard;

impl GetCard {
    #[process]
    fn respond(&self, views: &crate::margaret::views::Views) -> Response {}
}
";

    #[test]
    fn generates_the_views_module_when_views_and_http_exist() {
        let code = generate(VIEWS_CRATE).expect("the build succeeds");

        assert!(module(&code, "mod").contains("pub mod views;"));
        assert!(module(&code, "views").contains("pub struct Views"));
        assert!(module(&code, "views").contains("card"));
        assert!(module(&code, "views/build").contains("pub async fn build"));
        assert!(concatenated(&code).contains("super::views::build::build(container)"));
    }

    #[test]
    fn propagates_a_views_failure() {
        let message = generate(
            "#[rustfmt::skip]\npub mod margaret;\n\n#[singleton]\n#[renders_view(name = \"CardLayout\")]\nstruct Card;\n\n#[singleton]\n#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\nstruct Page;\n\nimpl Page {\n    #[process]\n    fn respond(&self) -> Response {}\n}\n",
        )
        .expect_err("the build fails")
        .to_string();

        assert!(message.contains("failed to generate the views"));
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
    fn awaits_an_async_constructor_in_the_container() {
        let code = generate(
            "#[rustfmt::skip]\npub mod margaret;\n\n#[singleton]\nstruct Pool;\n\nimpl Pool {\n    #[constructor]\n    async fn create() -> Self {}\n}\n\n#[singleton]\n#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\nstruct Page;\n\nimpl Page {\n    #[process]\n    fn respond(&self) -> Response {}\n}\n",
        )
        .expect("the async build succeeds");

        let serve: String = module(&code, "serve").split_whitespace().collect();

        assert!(module(&code, "container").contains("Pool::create().await"));
        assert!(concatenated(&code).contains("async fn server_public"));
        assert!(serve.contains("super::http::server_public::server_public(container,"));
    }

    const MULTI_SERVER_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

#[singleton]
#[responds_to_http(method = \"get\", path = \"/\", server = \"public\")]
struct Index;

impl Index {
    #[process]
    fn respond(&self) -> Response {}
}

#[singleton]
#[responds_to_http(method = \"get\", path = \"/metrics\", server = \"internal\")]
struct Metrics;

impl Metrics {
    #[process]
    fn respond(&self) -> Response {}
}
";

    #[test]
    fn generates_a_server_function_and_address_argument_per_active_server() {
        let code = generate(MULTI_SERVER_CRATE).expect("the build succeeds");

        let http = concatenated(&code);
        assert!(http.contains("async fn server_public"));
        assert!(http.contains("async fn server_internal"));

        let serve: String = module(&code, "serve").split_whitespace().collect();
        assert!(serve.contains("super::http::server_public::server_public(container,"));
        assert!(serve.contains("super::http::server_internal::server_internal(container,"));
        assert!(serve.contains(r#"address_argument:"public-addr""#));
        assert!(serve.contains(r#"address_argument:"internal-addr""#));

        let run = module(&code, "run");
        assert!(run.contains(r#"clap::Arg::new("public-addr")"#));
        assert!(run.contains(r#"clap::Arg::new("internal-addr")"#));
    }

    #[test]
    fn rejects_a_non_string_server() {
        let message = generate(
            "#[rustfmt::skip]\npub mod margaret;\n\n#[singleton]\n#[responds_to_http(method = \"get\", path = \"/\", server = crate::Ghost)]\nstruct Page;\n\nimpl Page {\n    #[process]\n    fn respond(&self) -> Response {}\n}\n",
        )
        .expect_err("the build fails")
        .to_string();

        assert!(message.contains("failed to index"));
    }

    #[test]
    fn supports_same_struct_name_route_handlers_in_different_modules() {
        let code = generate(
            "#[rustfmt::skip]\npub mod margaret;\n\nmod routes {\n#[singleton]\n#[responds_to_http(method = \"get\", path = \"/a\", server = \"public\")]\nstruct Page;\n\nimpl Page {\n    #[process]\n    fn respond(&self) -> Response {}\n}\n}\n\nmod endpoints {\n#[singleton]\n#[responds_to_http(method = \"get\", path = \"/b\", server = \"internal\")]\nstruct Page;\n\nimpl Page {\n    #[process]\n    fn respond(&self) -> Response {}\n}\n}\n",
        )
        .expect("two `Page` handlers in different modules coexist");

        let http = concatenated(&code);

        assert!(!http.contains("enum RouteName"));
        assert!(http.contains("\"/a\""));
        assert!(http.contains("\"/b\""));
        assert!(http.contains("crate::routes::Page"));
        assert!(http.contains("crate::endpoints::Page"));
    }

    const FRAMEWORK_NAMED_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

#[singleton]
struct Build;

#[singleton]
struct Container;

#[singleton]
struct Routes;

#[singleton]
#[responds_to_http(method = \"get\", name = \"origin\", path = \"/o\", server = \"routes\")]
struct Origin;

impl Origin {
    #[process]
    fn respond(&self) -> Response {}
}

#[singleton]
#[responds_to_http(method = \"get\", name = \"new\", path = \"/n/{id}\", server = \"routes\")]
struct New;

impl New {
    #[process]
    fn respond(&self, #[route_parameter(from = \"id\")] id: String) -> Response {}
}
";

    #[test]
    fn generates_without_reserving_names_for_components_named_after_framework_identifiers() {
        let code = generate(FRAMEWORK_NAMED_CRATE).expect("the build succeeds");

        let container: String = module(&code, "container").split_whitespace().collect();
        let build: String = module(&code, "container/build")
            .split_whitespace()
            .collect();
        let routes: String = module(&code, "routes").split_whitespace().collect();
        let http: String = concatenated(&code).split_whitespace().collect();

        assert!(build.contains("pubfnbuild()->super::Container"));
        assert!(container.contains("pubasyncfnbuild(&self)"));
        assert!(container.contains("pubasyncfncontainer(&self)"));
        assert!(container.contains("pubasyncfnroutes(&self)"));
        assert!(!container.contains("build_2"));
        assert!(!container.contains("container_2"));
        assert!(!container.contains("routes_2"));

        assert!(routes.contains("pubstructRoutes{pubroutes:servers::routes::Routes,}"));
        assert!(module(&code, "routes/servers/routes").contains("pub struct Routes"));

        assert!(http.contains("container:&super::super::container::Container"));
        assert!(!http.contains("usesuper::container::Container"));
    }

    #[test]
    fn generates_the_schema_module_and_command_when_models_exist() {
        let code = generate(MODELS_CRATE).expect("the build succeeds");

        assert!(module(&code, "mod").contains("pub mod schema;"));
        assert!(module(&code, "mod").contains("pub mod run;"));
        assert!(!has_module(&code, "http"));
        assert!(module(&code, "schema").contains("pub fn schema"));
        assert!(module(&code, "schema").contains("\"widgets\""));
        assert!(module(&code, "schema").contains("ColumnType::Uuid"));

        let run: String = module(&code, "run").split_whitespace().collect();

        assert!(run.contains("\"schema\""));
        assert!(run.contains("super::schema::schema()"));
    }

    #[test]
    fn propagates_a_models_failure() {
        let message = generate(
            "#[rustfmt::skip]\npub mod margaret;\n\n#[model(table = \"widgets\")]\nstruct Widget {\n    #[column]\n    id: u64,\n}\n",
        )
        .expect_err("the build fails")
        .to_string();

        assert!(message.contains("failed to generate the models"));
    }
}
