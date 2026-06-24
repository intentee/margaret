mod field_name;
pub mod http_codegen_error;
mod http_route;
mod middleware_binding;
mod render;

use std::fs;
use std::path::Path;

use margaret_attributes::attribute_index::AttributeIndex;

use crate::http_codegen_error::HttpCodegenError;
use crate::http_route::http_routes;
use crate::middleware_binding::middleware_bindings;
use crate::render::render;

pub fn generate_http_source(
    crate_name: &str,
    source_directory: &Path,
) -> Result<String, HttpCodegenError> {
    let index = AttributeIndex::from_crate_root(crate_name, source_directory)?;
    let bindings = middleware_bindings(&index)?;
    let routes = http_routes(&index, &bindings)?;

    Ok(render(&routes))
}

pub fn build(manifest_directory: impl AsRef<Path>) -> Result<(), HttpCodegenError> {
    let manifest_directory = manifest_directory.as_ref();
    let source_directory = manifest_directory.join("src");
    let http_path = source_directory.join("http.rs");

    if !http_path.exists() {
        fs::write(&http_path, "").expect("the http stub is written");
    }

    margaret_container::build(manifest_directory)?;

    let source = generate_http_source("crate", &source_directory)?;

    if !file_matches(&http_path, &source) {
        fs::write(&http_path, &source).expect("the generated http source is written");
    }

    println!("cargo:rerun-if-changed={}", source_directory.display());

    Ok(())
}

fn file_matches(path: &Path, source: &str) -> bool {
    matches!(fs::read_to_string(path), Ok(existing) if existing == source)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;
    use tempfile::tempdir;

    use crate::build;
    use crate::generate_http_source;

    const RESPONDERS_AND_MIDDLEWARE: &str = r#"
#[responds_to_http(method = Get, path = "/resource")]
#[traced]
#[guard(crate::action::Action::Read)]
struct Resource;

#[responds_to_http(method = Get, path = "/open")]
struct Open;

#[http_middleware(handles = guard, priority = 100)]
struct Guard;

#[http_middleware(handles = traced, priority = 10)]
struct Tracer;
"#;

    const BUILDABLE: &str = r#"
pub mod container;
pub mod http;

#[singleton]
#[responds_to_http(method = Get, path = "/items")]
#[traced]
struct ShowItems;

impl ShowItems {
    #[constructor]
    fn create() -> Self {}
}

#[singleton]
#[http_middleware(handles = traced, priority = 100)]
struct RequestLogger;

impl RequestLogger {
    #[constructor]
    fn create() -> Self {}
}
"#;

    fn crate_with(lib_source: &str) -> TempDir {
        let directory = tempdir().expect("a temporary crate directory is created");
        let source_directory = directory.path().join("src");

        fs::create_dir(&source_directory).expect("the src directory is created");
        fs::write(source_directory.join("lib.rs"), lib_source).expect("lib.rs is written");

        directory
    }

    fn source_for(lib_source: &str) -> String {
        let directory = crate_with(lib_source);

        generate_http_source("crate", &directory.path().join("src"))
            .expect("the http source is generated")
            .split_whitespace()
            .collect()
    }

    fn error_for(lib_source: &str) -> String {
        let directory = crate_with(lib_source);

        generate_http_source("crate", &directory.path().join("src"))
            .expect_err("the http source fails to generate")
            .to_string()
    }

    #[test]
    fn generates_per_route_typed_marker_middleware() {
        let source = source_for(RESPONDERS_AND_MIDDLEWARE);

        assert!(source.contains("pubfnserver"));
        assert!(source.contains("margaret_http::method::Method::Get"));
        assert!(source.contains(
            "\"/open\",margaret_http::responder_handler::responder_handler(container.open.clone())"
        ));
        assert!(source.contains("container.guard.clone(),crate::action::Action::Read"));
        assert!(source.contains("container.tracer.clone(),()"));
        assert!(source.contains(
            "margaret_http::responder_handler::responder_handler(container.resource.clone()"
        ));

        let guard = source.find("container.guard").expect("the guard is wired");
        let tracer = source.find("container.tracer").expect("the tracer is wired");

        assert!(guard < tracer);
    }

    #[test]
    fn propagates_an_index_failure() {
        assert!(error_for("use other::*;\n").contains("failed to index"));
    }

    #[test]
    fn rejects_responds_to_http_on_a_non_struct() {
        let message = error_for("#[responds_to_http(method = Get, path = \"/x\")]\nenum Bad {}\n");

        assert!(message.contains("#[responds_to_http]"));
    }

    #[test]
    fn propagates_malformed_responder_arguments() {
        assert!(error_for("#[responds_to_http(= 5)]\nstruct Bad;\n").contains("failed to index"));
    }

    #[test]
    fn propagates_a_non_path_method_argument() {
        let message =
            error_for("#[responds_to_http(method = \"GET\", path = \"/x\")]\nstruct Bad;\n");

        assert!(message.contains("failed to index"));
    }

    #[test]
    fn rejects_a_responder_without_a_method() {
        assert!(
            error_for("#[responds_to_http(path = \"/x\")]\nstruct Bad;\n")
                .contains("missing the 'method'")
        );
    }

    #[test]
    fn propagates_a_non_string_path_argument() {
        let message = error_for("#[responds_to_http(method = Get, path = 5)]\nstruct Bad;\n");

        assert!(message.contains("failed to index"));
    }

    #[test]
    fn rejects_a_responder_without_a_path() {
        assert!(
            error_for("#[responds_to_http(method = Get)]\nstruct Bad;\n")
                .contains("missing the 'path'")
        );
    }

    #[test]
    fn rejects_http_middleware_on_a_non_struct() {
        let message = error_for("#[http_middleware(handles = x, priority = 1)]\nenum Bad {}\n");

        assert!(message.contains("#[http_middleware]"));
    }

    #[test]
    fn propagates_malformed_middleware_arguments() {
        assert!(error_for("#[http_middleware(= 5)]\nstruct Bad;\n").contains("failed to index"));
    }

    #[test]
    fn rejects_middleware_without_handles() {
        assert!(
            error_for("#[http_middleware(priority = 1)]\nstruct Bad;\n")
                .contains("missing the 'handles'")
        );
    }

    #[test]
    fn propagates_a_non_path_handles_argument() {
        let message = error_for("#[http_middleware(handles = \"x\", priority = 1)]\nstruct Bad;\n");

        assert!(message.contains("failed to index"));
    }

    #[test]
    fn rejects_middleware_without_a_priority() {
        assert!(
            error_for("#[http_middleware(handles = x)]\nstruct Bad;\n")
                .contains("missing the 'priority'")
        );
    }

    #[test]
    fn rejects_middleware_with_a_non_integer_priority() {
        let message = error_for("#[http_middleware(handles = x, priority = \"high\")]\nstruct Bad;\n");

        assert!(message.contains("not an integer"));
    }

    #[test]
    fn rejects_a_malformed_marker() {
        let message = error_for(
            "#[responds_to_http(method = Get, path = \"/x\")]\n#[guard(crate::A, crate::B)]\nstruct Bad;\n\n#[http_middleware(handles = guard, priority = 1)]\nstruct Guard;\n",
        );

        assert!(message.contains("must carry zero or one positional argument"));
    }

    #[test]
    fn propagates_malformed_marker_arguments() {
        let message = error_for(
            "#[responds_to_http(method = Get, path = \"/x\")]\n#[guard(= 5)]\nstruct Bad;\n\n#[http_middleware(handles = guard, priority = 1)]\nstruct Guard;\n",
        );

        assert!(message.contains("failed to index"));
    }

    #[test]
    fn build_generates_the_server_and_is_idempotent() {
        let directory = crate_with(BUILDABLE);

        build(directory.path()).expect("the first build succeeds");
        build(directory.path()).expect("the second build is idempotent");

        let http =
            fs::read_to_string(directory.path().join("src/http.rs")).expect("http.rs exists");
        let container = fs::read_to_string(directory.path().join("src/container.rs"))
            .expect("container.rs exists");

        assert!(http.contains("fn server"));
        assert!(container.contains("struct Container"));
    }

    #[test]
    fn build_propagates_a_container_failure() {
        let directory =
            crate_with("pub mod container;\npub mod http;\n\n#[singleton]\nenum Bad {}\n");

        let message = build(directory.path())
            .expect_err("the build fails")
            .to_string();

        assert!(message.contains("failed to generate the dependency container"));
    }

    #[test]
    fn build_propagates_an_http_failure() {
        let directory = crate_with(
            "pub mod container;\npub mod http;\n\n#[singleton]\n#[responds_to_http(path = \"/x\")]\nstruct Bad;\n\nimpl Bad {\n    #[constructor]\n    fn create() -> Self {}\n}\n",
        );

        let message = build(directory.path())
            .expect_err("the build fails")
            .to_string();

        assert!(message.contains("missing the 'method'"));
    }
}
