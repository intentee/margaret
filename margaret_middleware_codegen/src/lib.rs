pub mod fold_layers;
pub mod layer_application;
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
    use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
    use std::fs;

    use quote::format_ident;
    use quote::quote;
    use tempfile::TempDir;
    use tempfile::tempdir;

    use margaret_attributes::attribute_index::AttributeIndex;
    use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_attributes::crate_root::CrateRoot;
    use margaret_attributes::framework_attribute::FrameworkAttribute;
    use margaret_container::container_bindings::ContainerBindings;
    use margaret_container::render_container::render_container;
    use margaret_request_binding_codegen::binding_registries::BindingRegistries;
    use margaret_request_binding_codegen::views_availability::ViewsAvailability;
    use margaret_serve_input_codegen::scan::scan;

    use crate::fold_layers::fold_layers;
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

    fn bindings_for(index: &AttributeIndex) -> ContainerBindings {
        let registry = scan(index).expect("the console arguments are scanned");

        render_container(index, &registry, &[])
            .expect("the container renders")
            .bindings
    }

    fn empty_bindings() -> ContainerBindings {
        bindings_for(&index_for("#[singleton]\nstruct Config;\n"))
    }

    fn registries_for(index: &AttributeIndex, bindings: &ContainerBindings) -> BindingRegistries {
        BindingRegistries::collect(index, bindings, ViewsAvailability::Available)
            .expect("the binding registries are collected")
    }

    fn plans_for(
        index: &AttributeIndex,
    ) -> Result<Vec<crate::middleware_plan::MiddlewarePlan>, MiddlewareCodegenError> {
        let bindings = bindings_for(index);

        middleware_plans(index, &bindings, &registries_for(index, &bindings))
    }

    fn wrappers_for(lib_source: &str) -> String {
        let index = index_for(lib_source);
        let plans = plans_for(&index).expect("the middleware plans are collected");

        render_middleware_wrappers(&plans)
            .into_iter()
            .map(|module| module.to_source())
            .collect::<String>()
            .split_whitespace()
            .collect()
    }

    fn plans_rejection_for(lib_source: &str) -> MiddlewareCodegenError {
        let index = index_for(lib_source);

        plans_for(&index)
            .err()
            .expect("the middleware plans fail to collect")
    }

    fn plans_error_for(lib_source: &str) -> String {
        plans_rejection_for(lib_source).to_string()
    }

    fn layers_for(lib_source: &str) -> Result<Vec<LayerApplication>, MiddlewareCodegenError> {
        let index = index_for(lib_source);
        let plans = plans_for(&index)?;
        let selected = index.select_framework_attribute(FrameworkAttribute::Middleware);
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

    const GUARD: &str = r"
use margaret::framework::http::next::Next;
use margaret::framework::http::request::Request;

#[handles_middleware_attribute(attribute = guard)]
struct Guard;

impl Guard {
    #[process]
    fn process(&self, request: &Request, next: Next) -> anyhow::Result<ResponseContinuation> {}
}
";

    #[test]
    fn generates_a_wrapper_that_forwards_to_the_process_method() {
        let source = wrappers_for(GUARD);

        assert!(source.contains("pubstructGuard{pubinner:std::sync::Arc<crate::Guard>,}"));
        assert!(
            source
                .contains("implmargaret::framework::http::http_middleware::HttpMiddlewareforGuard")
        );
        assert!(source.contains("self.inner.process(request,next)"));
    }

    #[test]
    fn awaits_a_middleware_that_declares_an_asynchronous_process_method() {
        let source = wrappers_for(
            r"
use margaret::framework::http::next::Next;
use margaret::framework::http::request::Request;

#[handles_middleware_attribute(attribute = guard)]
struct Guard;

impl Guard {
    #[process]
    async fn process(&self, request: &Request, next: Next) -> anyhow::Result<ResponseContinuation> {}
}
",
        );

        assert!(source.contains("self.inner.process(request,next).await"));
    }

    #[test]
    fn injects_the_routes_reference_into_the_wrapper() {
        let source = wrappers_for(
            "use crate::margaret::routes::Routes;\nuse margaret::framework::http::next::Next;\nuse margaret::framework::http::request::Request;\n\n#[handles_middleware_attribute(attribute = traced)]\nstruct Tracer;\nimpl Tracer {\n    #[process]\n    fn process(&self, request: &Request, next: Next, routes: &Routes) -> anyhow::Result<ResponseContinuation> {}\n}\n",
        );

        assert!(source.contains(
            "pubstructTracer{pubinner:std::sync::Arc<crate::Tracer>,pubroutes:std::sync::Arc<super::super::routes::Routes>,}"
        ));
        assert!(source.contains("self.inner.process(request,next,&self.routes)"));
    }

    #[test]
    fn omits_the_request_from_a_wrapper_that_only_delegates() {
        let source = wrappers_for(
            "use margaret::framework::http::next::Next;\n\n#[handles_middleware_attribute(attribute = traced)]\nstruct Tracer;\nimpl Tracer {\n    #[process]\n    fn process(&self, next: Next) -> anyhow::Result<ResponseContinuation> {}\n}\n",
        );

        assert!(source.contains(
            "asyncfnprocess(&self,_request:&margaret::framework::http::request::Request,next:margaret::framework::http::next::Next,)"
        ));
        assert!(source.contains("self.inner.process(next)"));
    }

    #[test]
    fn omits_the_next_handler_from_a_short_circuiting_wrapper() {
        let source = wrappers_for(
            "use margaret::framework::http::request::Request;\n\n#[handles_middleware_attribute(attribute = guard)]\nstruct Guard;\nimpl Guard {\n    #[process]\n    fn process(&self, request: &Request) -> anyhow::Result<ResponseContinuation> {}\n}\n",
        );

        assert!(source.contains(
            "asyncfnprocess(&self,request:&margaret::framework::http::request::Request,_next:margaret::framework::http::next::Next,)"
        ));
        assert!(source.contains("self.inner.process(request)"));
    }

    #[test]
    fn disambiguates_wrappers_that_derive_the_same_name() {
        let source = wrappers_for(
            "use margaret::framework::http::next::Next;\n\n#[handles_middleware_attribute(attribute = one)]\nstruct V2;\nimpl V2 {\n    #[process]\n    fn process(&self, next: Next) -> anyhow::Result<ResponseContinuation> {}\n}\n\n#[handles_middleware_attribute(attribute = two)]\nstruct V_2;\nimpl V_2 {\n    #[process]\n    fn process(&self, next: Next) -> anyhow::Result<ResponseContinuation> {}\n}\n",
        );

        assert!(source.contains("pubstructV2{pubinner:std::sync::Arc<crate::V2>,}"));
        assert!(source.contains("pubstructV22{pubinner:std::sync::Arc<crate::V_2>,}"));
    }

    #[test]
    fn rejects_a_handler_absent_from_the_container_plan() {
        let index = index_for(GUARD);
        let bindings = empty_bindings();
        let error = middleware_plans(&index, &bindings, &registries_for(&index, &bindings))
            .err()
            .expect("the middleware plans must come from the same container plan");

        assert!(error.to_string().contains("crate::Guard"));
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
        let error = plans_rejection_for(
            "#[handles_middleware_attribute(attribute = \"x\")]\nstruct Bad;\n",
        );

        assert!(matches!(
            error,
            MiddlewareCodegenError::AttributeArguments {
                source: AttributeArgumentsError::UnexpectedArgument {
                    ref key,
                    ref expected,
                    ..
                }
            } if key == "attribute" && expected == "path"
        ));
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
                "#[handles_middleware_attribute(attribute = guard)]\nstruct Bad;\nimpl Bad {\n    #[process]\n    fn process(&self, flag: bool) -> anyhow::Result<ResponseContinuation> {}\n}\n"
            )
            .contains("must be the current request, the next handler")
        );
    }

    #[test]
    fn resolves_the_layers_in_declaration_order() {
        let layers = layers_for(
            "use margaret::framework::http::next::Next;\n\n#[middleware(first)]\n#[middleware(second)]\nstruct Site;\n\n#[handles_middleware_attribute(attribute = first)]\nstruct First;\nimpl First {\n    #[process]\n    fn process(&self, next: Next) -> anyhow::Result<ResponseContinuation> {}\n}\n\n#[handles_middleware_attribute(attribute = second)]\nstruct Second;\nimpl Second {\n    #[process]\n    fn process(&self, next: Next) -> anyhow::Result<ResponseContinuation> {}\n}\n",
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
                .contains("must reference exactly one tag")
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
    fn rejects_a_handler_with_a_multi_segment_tag() {
        assert!(
            plans_error_for(
                "#[handles_middleware_attribute(attribute = tags::guard)]\nstruct Guard;\n"
            )
            .contains("not a single plain name")
        );
    }

    #[test]
    fn propagates_malformed_middleware_attribute_arguments() {
        assert!(layers_error_for("#[middleware(= 5)]\nstruct Site;\n").contains("failed to index"));
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
            concrete: CanonicalPath::new(vec!["crate".to_string(), wrapper.to_string()]),
            field: format_ident!("{field}"),
            injects_peer_spiffe_id: false,
            injects_routes: false,
            injects_views: false,
            wrapper: format_ident!("{wrapper}"),
        }
    }

    #[test]
    fn folds_the_first_declared_layer_outermost() {
        let folded = fold_layers(
            &[
                plain_layer("tracer", "Tracer"),
                plain_layer("guard", "Guard"),
            ],
            quote! { BASE },
            &quote! { super::super::middleware },
            &empty_bindings(),
        )
        .to_string()
        .split_whitespace()
        .collect::<String>();

        assert_eq!(
            folded,
            "margaret::framework::http::layer::layer(std::sync::Arc::new(super::super::middleware::Tracer{inner:container.tracer()}),margaret::framework::http::layer::layer(std::sync::Arc::new(super::super::middleware::Guard{inner:container.guard()}),BASE))"
        );
    }

    #[test]
    fn folds_no_layers_into_the_base_unchanged() {
        let folded = fold_layers(
            &[],
            quote! { BASE },
            &quote! { super::super::middleware },
            &empty_bindings(),
        )
        .to_string()
        .split_whitespace()
        .collect::<String>();

        assert_eq!(folded, "BASE");
    }

    #[test]
    fn builds_a_declaration_ordered_vector_of_boxed_middleware() {
        let routed = LayerApplication {
            concrete: CanonicalPath::new(vec!["crate".to_string(), "Tracer".to_string()]),
            field: format_ident!("tracer"),
            injects_peer_spiffe_id: false,
            injects_routes: true,
            injects_views: false,
            wrapper: format_ident!("Tracer"),
        };
        let vector = middleware_vec_tokens(
            &[routed, plain_layer("guard", "Guard")],
            &quote! { super::super::middleware },
            &empty_bindings(),
        )
        .to_string()
        .split_whitespace()
        .collect::<String>();

        assert!(vector.contains(
            "letmutmiddleware:::std::vec::Vec<::std::sync::Arc<dynmargaret::framework::http::http_middleware::HttpMiddleware>,>=::std::vec::Vec::new();"
        ));
        assert!(vector.contains(
            "middleware.push(std::sync::Arc::new(super::super::middleware::Tracer{inner:container.tracer(),routes:routes.clone()}));"
        ));
        assert!(vector.contains(
            "middleware.push(std::sync::Arc::new(super::super::middleware::Guard{inner:container.guard()}));"
        ));

        let tracer = vector
            .find("Tracer")
            .expect("the routed middleware is present");
        let guard = vector
            .find("Guard")
            .expect("the plain middleware is present");

        assert!(tracer < guard);
    }

    const CONSOLE_ARGUMENT_MIDDLEWARE: &str = r#"
use margaret::framework::http::next::Next;

#[singleton]
#[handles_middleware_attribute(attribute = guard)]
struct Guard;

impl Guard {
    #[constructor]
    fn create(#[console_argument(from = "token")] token: String) -> anyhow::Result<Self> {}

    #[process]
    fn process(&self, next: Next) -> anyhow::Result<ResponseContinuation> {}
}

#[middleware(guard)]
struct Site;
"#;

    #[test]
    fn weaves_a_serve_input_into_the_middleware_instance() {
        let layers = layers_for(CONSOLE_ARGUMENT_MIDDLEWARE).expect("the layers resolve");
        let bindings = bindings_for(&index_for(CONSOLE_ARGUMENT_MIDDLEWARE));
        let folded = fold_layers(
            &layers,
            quote! { BASE },
            &quote! { super::super::middleware },
            &bindings,
        )
        .to_string()
        .split_whitespace()
        .collect::<String>();

        assert!(folded.contains("container.guard()"));
    }

    #[test]
    fn extracts_a_validation_result_form_request_in_a_middleware() {
        let source = wrappers_for(
            r"
use margaret::framework::http::next::Next;
use margaret::framework::validation::validation_result::ValidationResult;

#[handles_middleware_attribute(attribute = guard)]
struct Guard;

impl Guard {
    #[process]
    fn process(&self, #[form_request(from = Json)] data: ValidationResult<Data>, next: Next) -> anyhow::Result<ResponseContinuation> {}
}
",
        );

        assert!(source.contains(
            "margaret::framework::http_validation::validate_input::validate_input(request,"
        ));
        assert!(
            source.contains(
                "margaret::framework::http_validation::request_input::RequestInput::Json"
            )
        );
        assert!(source.contains(
            "asyncfnprocess(&self,request:&margaret::framework::http::request::Request,next:margaret::framework::http::next::Next,)"
        ));
        assert!(source.contains("self.inner.process(data,next)"));
    }

    #[test]
    fn extracts_a_bare_model_form_request_in_a_middleware() {
        let source = wrappers_for(
            r"
#[handles_middleware_attribute(attribute = guard)]
struct Guard;

impl Guard {
    #[process]
    fn process(&self, #[form_request(from = Form)] data: Data) -> anyhow::Result<ResponseContinuation> {}
}
",
        );

        assert!(source.contains(
            "margaret::framework::http_validation::require_input::require_input(request,"
        ));
        assert!(
            source.contains(
                "margaret::framework::http_validation::request_input::RequestInput::Form"
            )
        );
        assert!(source.contains("Ok(model)=>model"));
        assert!(source.contains("::std::result::Result::Ok(response.into())"));
        assert!(source.contains(
            "asyncfnprocess(&self,request:&margaret::framework::http::request::Request,_next:margaret::framework::http::next::Next,)"
        ));
        assert!(source.contains("self.inner.process(data)"));
    }

    #[test]
    fn extracts_a_form_request_alongside_the_request_and_next_in_a_middleware() {
        let source = wrappers_for(
            r"
use margaret::framework::http::next::Next;
use margaret::framework::http::request::Request;

#[handles_middleware_attribute(attribute = guard)]
struct Guard;

impl Guard {
    #[process]
    fn process(&self, request: &Request, #[form_request(from = Query)] filters: Filters, next: Next) -> anyhow::Result<ResponseContinuation> {}
}
",
        );

        assert!(
            source.contains(
                "margaret::framework::http_validation::request_input::RequestInput::Query"
            )
        );
        assert!(source.contains(
            "asyncfnprocess(&self,request:&margaret::framework::http::request::Request,next:margaret::framework::http::next::Next,)"
        ));
        assert!(source.contains("self.inner.process(request,filters,next)"));
    }

    #[test]
    fn injects_the_views_reference_into_the_wrapper() {
        let source = wrappers_for(
            r"
use margaret::framework::http::next::Next;

#[handles_middleware_attribute(attribute = traced)]
struct Tracer;

impl Tracer {
    #[process]
    fn process(&self, next: Next, views: &crate::margaret::views::Views) -> anyhow::Result<ResponseContinuation> {}
}
",
        );

        assert!(source.contains(
            "pubstructTracer{pubinner:std::sync::Arc<crate::Tracer>,pubviews:std::sync::Arc<super::super::views::Views>,}"
        ));
        assert!(source.contains("self.inner.process(next,&self.views)"));
    }

    #[test]
    fn disambiguates_wrapper_parameters_named_request_and_next() {
        let source = wrappers_for(
            r"
use margaret::framework::http::next::Next;
use margaret::framework::http::request::Request;

#[handles_middleware_attribute(attribute = guard)]
struct Guard;

impl Guard {
    #[process]
    fn process(&self, next: &Request, #[form_request(from = Form)] request: Data, following: Next) -> anyhow::Result<ResponseContinuation> {}
}
",
        );

        assert!(source.contains(
            "asyncfnprocess(&self,request_2:&margaret::framework::http::request::Request,next_2:margaret::framework::http::next::Next,)"
        ));
        assert!(source.contains("letnext=request_2;"));
        assert!(source.contains(
            "margaret::framework::http_validation::require_input::require_input(request_2,"
        ));
        assert!(source.contains("self.inner.process(next,request,next_2)"));
    }

    #[test]
    fn rejects_a_middleware_with_multiple_next_handlers() {
        assert!(
            plans_error_for(
                "use margaret::framework::http::next::Next;\n\n#[handles_middleware_attribute(attribute = guard)]\nstruct Guard;\nimpl Guard {\n    #[process]\n    fn process(&self, first: Next, second: Next) -> anyhow::Result<ResponseContinuation> {}\n}\n"
            )
            .contains("declares more than one next handler")
        );
    }

    #[test]
    fn rejects_a_route_parameter_in_a_middleware() {
        assert!(
            plans_error_for(
                "#[handles_middleware_attribute(attribute = guard)]\nstruct Guard;\nimpl Guard {\n    #[process]\n    fn process(&self, #[route_parameter(from = \"id\")] id: String) -> anyhow::Result<ResponseContinuation> {}\n}\n"
            )
            .contains("has no route path to bind from")
        );
    }
}
