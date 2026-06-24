pub mod http_codegen_error;
mod http_route;
mod middleware_binding;
mod render;

use std::path::Path;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;

use crate::http_codegen_error::HttpCodegenError;
use crate::http_route::http_routes;
use crate::middleware_binding::middleware_bindings;
use crate::render::render;

pub fn render_http(index: &AttributeIndex) -> Result<String, HttpCodegenError> {
    let bindings = middleware_bindings(index)?;
    let routes = http_routes(index, &bindings)?;

    Ok(render(&routes))
}

pub fn has_responders(index: &AttributeIndex) -> bool {
    let selector = AttributeSelector::parse("responds_to_http").expect("a valid selector");

    !index.select(&selector).is_empty()
}

pub fn generate_http_source(
    crate_name: &str,
    source_directory: &Path,
) -> Result<String, HttpCodegenError> {
    let index = AttributeIndex::from_crate_root(crate_name, source_directory)?;

    render_http(&index)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use margaret_attributes::attribute_index::AttributeIndex;
    use tempfile::TempDir;
    use tempfile::tempdir;

    use crate::generate_http_source;
    use crate::has_responders;

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

    fn crate_with(lib_source: &str) -> TempDir {
        let directory = tempdir().expect("a temporary crate directory is created");
        let source_directory = directory.path().join("src");

        fs::create_dir(&source_directory).expect("the src directory is created");
        fs::write(source_directory.join("lib.rs"), lib_source).expect("lib.rs is written");

        directory
    }

    fn index_for(lib_source: &str) -> AttributeIndex {
        let directory = crate_with(lib_source);

        AttributeIndex::from_crate_root("crate", &directory.path().join("src"))
            .expect("the crate is indexed")
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
        assert!(source.contains("usesuper::container::Container"));
        assert!(source.contains("margaret_http::method::Method::Get"));
        assert!(source.contains(
            "\"/open\",margaret_http::responder_handler::responder_handler(container.open())"
        ));
        assert!(source.contains("container.guard(),crate::action::Action::Read"));
        assert!(source.contains("container.tracer(),()"));
        assert!(
            source.contains(
                "margaret_http::responder_handler::responder_handler(container.resource()"
            )
        );

        let guard = source.find("container.guard").expect("the guard is wired");
        let tracer = source
            .find("container.tracer")
            .expect("the tracer is wired");

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
        let message =
            error_for("#[http_middleware(handles = x, priority = \"high\")]\nstruct Bad;\n");

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
    fn reports_responders_present() {
        let index = index_for("#[responds_to_http(method = Get, path = \"/x\")]\nstruct R;\n");

        assert!(has_responders(&index));
    }

    #[test]
    fn reports_no_responders() {
        let index = index_for("#[singleton]\nstruct S;\n");

        assert!(!has_responders(&index));
    }
}
