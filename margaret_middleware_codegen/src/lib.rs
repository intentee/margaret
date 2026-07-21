pub mod fold_layers;
pub mod has_middleware;
pub mod layer_application;
mod middleware_argument;
mod middleware_attribute_arguments;
pub mod middleware_codegen_error;
mod middleware_instance_tokens;
pub mod middleware_plan;
pub mod middleware_plans;
pub mod middleware_vec_tokens;
pub mod render_middleware_wrappers;
pub mod resolve_layers;

#[cfg(test)]
mod tests {
    use std::fs;

    use quote::format_ident;
    use quote::quote;
    use tempfile::TempDir;
    use tempfile::tempdir;

    use margaret_attributes::attribute_index::AttributeIndex;
    use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
    use margaret_attributes::attribute_selector::AttributeSelector;
    use margaret_attributes::crate_root::CrateRoot;

    use crate::fold_layers::fold_layers;
    use crate::has_middleware::has_middleware;
    use crate::layer_application::LayerApplication;
    use crate::middleware_codegen_error::MiddlewareCodegenError;
    use crate::middleware_plans::middleware_plans;
    use crate::middleware_vec_tokens::middleware_vec_tokens;
    use crate::render_middleware_wrappers::render_middleware_wrappers;
    use crate::resolve_layers::resolve_layers;

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

    fn wrappers_for(lib_source: &str) -> String {
        let index = index_for(lib_source);
        let plans = middleware_plans(&index).expect("the middleware plans are collected");

        render_middleware_wrappers(&plans)
            .to_string()
            .split_whitespace()
            .collect()
    }

    fn plans_error_for(lib_source: &str) -> String {
        let index = index_for(lib_source);
        let failure = middleware_plans(&index).err();

        failure
            .expect("the middleware plans fail to collect")
            .to_string()
    }

    fn layers_for(lib_source: &str) -> Result<Vec<LayerApplication>, MiddlewareCodegenError> {
        let index = index_for(lib_source);
        let plans = middleware_plans(&index)?;
        let selected = index.select(&AttributeSelector::from_marker("middleware"));
        let site = selected
            .into_iter()
            .next()
            .expect("a site carrying a #[middleware] attribute");

        resolve_layers(site.item(), &plans, "site 'Site'")
    }

    fn layers_error_for(lib_source: &str) -> String {
        let failure = layers_for(lib_source).err();

        failure.expect("the layers fail to resolve").to_string()
    }

    const GUARD: &str = r#"
use margaret_http::next::Next;
use margaret_http::request::Request;

#[handles_middleware_attribute(attribute = guard)]
struct Guard;

impl Guard {
    #[process]
    fn process(&self, request: &Request, next: Next) -> ResponseContinuation {}
}
"#;

    #[test]
    fn generates_a_wrapper_that_forwards_to_the_process_method() {
        let source = wrappers_for(GUARD);

        assert!(source.contains("pubstructGuard{pubinner:std::sync::Arc<crate::Guard>,}"));
        assert!(source.contains("implmargaret_http::http_middleware::HttpMiddlewareforGuard"));
        assert!(source.contains("self.inner.process(request,next).await"));
    }

    #[test]
    fn injects_the_routes_reference_into_the_wrapper() {
        let source = wrappers_for(
            "use crate::margaret::routes::Routes;\nuse margaret_http::next::Next;\nuse margaret_http::request::Request;\n\n#[handles_middleware_attribute(attribute = traced)]\nstruct Tracer;\nimpl Tracer {\n    #[process]\n    fn process(&self, request: &Request, next: Next, routes: &Routes) -> ResponseContinuation {}\n}\n",
        );

        assert!(source.contains(
            "pubstructTracer{pubinner:std::sync::Arc<crate::Tracer>,pubroutes:std::sync::Arc<super::routes::Routes>,}"
        ));
        assert!(source.contains("self.inner.process(request,next,&self.routes).await"));
    }

    #[test]
    fn omits_the_request_from_a_wrapper_that_only_delegates() {
        let source = wrappers_for(
            "use margaret_http::next::Next;\n\n#[handles_middleware_attribute(attribute = traced)]\nstruct Tracer;\nimpl Tracer {\n    #[process]\n    fn process(&self, next: Next) -> ResponseContinuation {}\n}\n",
        );

        assert!(source.contains(
            "asyncfnprocess(&self,_request:&margaret_http::request::Request,next:margaret_http::next::Next,)"
        ));
        assert!(source.contains("self.inner.process(next).await"));
    }

    #[test]
    fn omits_the_next_handler_from_a_short_circuiting_wrapper() {
        let source = wrappers_for(
            "use margaret_http::request::Request;\n\n#[handles_middleware_attribute(attribute = guard)]\nstruct Guard;\nimpl Guard {\n    #[process]\n    fn process(&self, request: &Request) -> ResponseContinuation {}\n}\n",
        );

        assert!(source.contains(
            "asyncfnprocess(&self,request:&margaret_http::request::Request,_next:margaret_http::next::Next,)"
        ));
        assert!(source.contains("self.inner.process(request).await"));
    }

    #[test]
    fn disambiguates_wrappers_that_derive_the_same_name() {
        let source = wrappers_for(
            "use margaret_http::next::Next;\n\n#[handles_middleware_attribute(attribute = one)]\nstruct V2;\nimpl V2 {\n    #[process]\n    fn process(&self, next: Next) -> ResponseContinuation {}\n}\n\n#[handles_middleware_attribute(attribute = two)]\nstruct V_2;\nimpl V_2 {\n    #[process]\n    fn process(&self, next: Next) -> ResponseContinuation {}\n}\n",
        );

        assert!(source.contains("pubstructV2{pubinner:std::sync::Arc<crate::V2>,}"));
        assert!(source.contains("pubstructV22{pubinner:std::sync::Arc<crate::V_2>,}"));
    }

    #[test]
    fn rejects_a_handler_on_a_non_struct() {
        assert!(
            plans_error_for("#[handles_middleware_attribute(attribute = x)]\nenum Bad {}\n")
                .contains("#[handles_middleware_attribute]")
        );
    }

    #[test]
    fn rejects_a_handler_without_the_attribute_argument() {
        assert!(
            plans_error_for("#[handles_middleware_attribute]\nstruct Bad;\n")
                .contains("missing the 'attribute'")
        );
    }

    #[test]
    fn rejects_a_handler_with_a_non_path_attribute_argument() {
        assert!(
            plans_error_for("#[handles_middleware_attribute(attribute = \"x\")]\nstruct Bad;\n")
                .contains("failed to index")
        );
    }

    #[test]
    fn propagates_malformed_handler_arguments() {
        assert!(
            plans_error_for("#[handles_middleware_attribute(= 5)]\nstruct Bad;\n")
                .contains("failed to index")
        );
    }

    #[test]
    fn rejects_a_handler_without_a_process_method() {
        assert!(
            plans_error_for("#[handles_middleware_attribute(attribute = guard)]\nstruct Bad;\n")
                .contains("no #[process] method")
        );
    }

    #[test]
    fn rejects_an_unclassifiable_handler_parameter() {
        assert!(
            plans_error_for(
                "#[handles_middleware_attribute(attribute = guard)]\nstruct Bad;\nimpl Bad {\n    #[process]\n    fn process(&self, flag: bool) -> ResponseContinuation {}\n}\n"
            )
            .contains("must be the current request, the next handler, or the routes")
        );
    }

    #[test]
    fn resolves_the_layers_in_declaration_order() {
        let layers = layers_for(
            "use margaret_http::next::Next;\n\n#[middleware(first)]\n#[middleware(second)]\nstruct Site;\n\n#[handles_middleware_attribute(attribute = first)]\nstruct First;\nimpl First {\n    #[process]\n    fn process(&self, next: Next) -> ResponseContinuation {}\n}\n\n#[handles_middleware_attribute(attribute = second)]\nstruct Second;\nimpl Second {\n    #[process]\n    fn process(&self, next: Next) -> ResponseContinuation {}\n}\n",
        )
        .expect("the layers resolve");

        assert_eq!(layers.len(), 2);
        assert_eq!(layers[0].wrapper.to_string(), "First");
        assert_eq!(layers[1].wrapper.to_string(), "Second");
    }

    #[test]
    fn rejects_a_middleware_attribute_without_a_tag() {
        assert!(
            layers_error_for("#[middleware]\nstruct Site;\n")
                .contains("must name exactly one middleware tag")
        );
    }

    #[test]
    fn rejects_an_unknown_middleware_tag() {
        assert!(
            layers_error_for("#[middleware(missing)]\nstruct Site;\n")
                .contains("no #[handles_middleware_attribute] handles it")
        );
    }

    #[test]
    fn rejects_an_ambiguous_middleware_tag() {
        assert!(
            layers_error_for(
                "use margaret_http::next::Next;\n\n#[middleware(shared)]\nstruct Site;\n\n#[handles_middleware_attribute(attribute = shared)]\nstruct First;\nimpl First {\n    #[process]\n    fn process(&self, next: Next) -> ResponseContinuation {}\n}\n\n#[handles_middleware_attribute(attribute = shared)]\nstruct Second;\nimpl Second {\n    #[process]\n    fn process(&self, next: Next) -> ResponseContinuation {}\n}\n"
            )
            .contains("handled by more than one #[handles_middleware_attribute]")
        );
    }

    #[test]
    fn propagates_malformed_middleware_attribute_arguments() {
        assert!(
            layers_error_for("#[middleware(= 5)]\nstruct Site;\n").contains("failed to index")
        );
    }

    #[test]
    fn propagates_a_plan_failure_while_resolving_layers() {
        assert!(
            layers_error_for(
                "#[middleware(logged)]\nstruct Site;\n\n#[handles_middleware_attribute]\nstruct Bad;\n"
            )
            .contains("missing the 'attribute'")
        );
    }

    fn plain_layer(field: &str, wrapper: &str) -> LayerApplication {
        LayerApplication {
            field: format_ident!("{field}"),
            injects_routes: false,
            wrapper: format_ident!("{wrapper}"),
        }
    }

    #[test]
    fn folds_the_first_declared_layer_outermost() {
        let folded = fold_layers(
            &[plain_layer("tracer", "Tracer"), plain_layer("guard", "Guard")],
            quote! { BASE },
            &quote! { super::super::middleware },
        )
        .to_string()
        .split_whitespace()
        .collect::<String>();

        assert_eq!(
            folded,
            "margaret_http::layer::layer(std::sync::Arc::new(super::super::middleware::Tracer{inner:container.tracer().await}),margaret_http::layer::layer(std::sync::Arc::new(super::super::middleware::Guard{inner:container.guard().await}),BASE))"
        );
    }

    #[test]
    fn folds_no_layers_into_the_base_unchanged() {
        let folded = fold_layers(&[], quote! { BASE }, &quote! { super::super::middleware })
            .to_string()
            .split_whitespace()
            .collect::<String>();

        assert_eq!(folded, "BASE");
    }

    #[test]
    fn builds_a_declaration_ordered_vector_of_boxed_middleware() {
        let routed = LayerApplication {
            field: format_ident!("tracer"),
            injects_routes: true,
            wrapper: format_ident!("Tracer"),
        };
        let vector = middleware_vec_tokens(
            &[routed, plain_layer("guard", "Guard")],
            &quote! { super::super::middleware },
        )
        .to_string()
        .split_whitespace()
        .collect::<String>();

        assert!(vector.contains(
            "letmutmiddleware:::std::vec::Vec<::std::sync::Arc<dynmargaret_http::http_middleware::HttpMiddleware>,>=::std::vec::Vec::new();"
        ));
        assert!(vector.contains(
            "middleware.push(std::sync::Arc::new(super::super::middleware::Tracer{inner:container.tracer().await,routes:routes.clone()}));"
        ));
        assert!(vector.contains(
            "middleware.push(std::sync::Arc::new(super::super::middleware::Guard{inner:container.guard().await}));"
        ));

        let tracer = vector.find("Tracer").expect("the routed middleware is present");
        let guard = vector.find("Guard").expect("the plain middleware is present");

        assert!(tracer < guard);
    }

    #[test]
    fn reports_middleware_present() {
        let index = index_for("#[handles_middleware_attribute(attribute = guard)]\nstruct Guard;\n");

        assert!(has_middleware(&index));
    }

    #[test]
    fn reports_no_middleware() {
        let index = index_for("#[singleton]\nstruct Config;\n");

        assert!(!has_middleware(&index));
    }
}
