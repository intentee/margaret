mod active_servers;
pub mod http_artifacts;
pub mod http_codegen_error;
pub mod http_plan;
mod http_responder_arguments;
mod http_route;
mod http_route_table;
mod http_routes;
pub mod http_server;
mod named_route;
mod render;
mod render_forwarders;
pub mod render_http;
mod render_routes;
mod route_body_intake;
mod route_group;
mod route_request_body_intake;
mod server_route_group;
mod server_serve_inputs;
pub mod server_transport_policy;
pub mod serves_spiffe;

#[cfg(test)]
mod tests {
    use std::mem::discriminant;

    use margaret_injection_codegen::injection_error::InjectionError;
    use margaret_tag_codegen::tag_error::TagError;

    use margaret_container::container_error::ContainerError;

    use margaret_attributes::attribute_error::AttributeError;

    use http::method::InvalidMethod;

    use http::Method;

    use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
    use margaret_middleware_codegen::middleware_codegen_error::MiddlewareCodegenError;
    use margaret_request_binding_codegen::request_binding_error::RequestBindingError;
    use std::collections::BTreeMap;
    use std::fs;
    use std::path::Path;

    use tempfile::TempDir;
    use tempfile::tempdir;

    use margaret_attributes::attribute_index::AttributeIndex;
    use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
    use margaret_attributes::crate_root::CrateRoot;
    use margaret_console_argument_codegen::console_argument::ConsoleArgument;
    use margaret_container::container_bindings::ContainerBindings;
    use margaret_container::render_container::render_container;
    use margaret_middleware_codegen::middleware_plans::middleware_plans;
    use margaret_request_binding_codegen::binding_registries::BindingRegistries;
    use margaret_request_binding_codegen::views_availability::ViewsAvailability;
    use margaret_serve_input_codegen::scan::scan;
    use margaret_serve_input_codegen::serve_input::ServeInput;

    use crate::http_codegen_error::HttpCodegenError;
    use crate::http_plan::HttpPlan;
    use crate::serves_spiffe::serves_spiffe;

    fn render_http(
        index: &AttributeIndex,
        has_views: bool,
        websocket_servers: &[String],
        middleware_plans: &[margaret_middleware_codegen::middleware_plan::MiddlewarePlan],
        bindings: &ContainerBindings,
        websocket_server_serve_inputs: &BTreeMap<String, Vec<ServeInput>>,
        registries: &BindingRegistries,
    ) -> Result<crate::http_artifacts::HttpArtifacts, HttpCodegenError> {
        HttpPlan::build(
            index,
            has_views,
            websocket_servers,
            middleware_plans,
            bindings,
            websocket_server_serve_inputs,
            registries,
        )
        .map(|plan| crate::render_http::render_http(plan, bindings))
    }

    fn bindings_for(index: &AttributeIndex) -> ContainerBindings {
        let registry = scan(index).expect("the console arguments are scanned");

        render_container(index, &registry, &[])
            .expect("the container renders")
            .bindings
    }

    fn no_websocket_arguments() -> BTreeMap<String, Vec<ServeInput>> {
        BTreeMap::new()
    }

    fn views_availability(has_views: bool) -> ViewsAvailability {
        if has_views {
            ViewsAvailability::Available
        } else {
            ViewsAvailability::Unavailable
        }
    }

    fn registries_for(index: &AttributeIndex, has_views: bool) -> BindingRegistries {
        BindingRegistries::collect(index, views_availability(has_views))
            .expect("the binding registries are collected")
    }

    fn rejection_with_container_source(source: &str, container_source: &str) -> HttpCodegenError {
        let index = index_for(source);
        let registries = registries_for(&index, false);
        let plans =
            middleware_plans(&index, &registries).expect("the middleware plans are collected");
        let container_index = index_for(container_source);
        let bindings = bindings_for(&container_index);

        render_http(
            &index,
            false,
            &[],
            &plans,
            &bindings,
            &no_websocket_arguments(),
            &registries,
        )
        .map(drop)
        .expect_err("the HTTP plan and container plan must agree")
    }

    fn http_source(
        crate_name: &str,
        source_directory: &Path,
        has_views: bool,
    ) -> Result<String, HttpCodegenError> {
        let index = AttributeIndexBuilder::new()
            .index_crate(&CrateRoot::new(crate_name, source_directory))?
            .build();
        let registries = BindingRegistries::collect(&index, views_availability(has_views))?;
        let plans = middleware_plans(&index, &registries)?;
        let bindings = bindings_for(&index);

        Ok(render_http(
            &index,
            has_views,
            &[],
            &plans,
            &bindings,
            &no_websocket_arguments(),
            &registries,
        )?
        .into_modules()
        .into_iter()
        .filter(|module| module.name() == "http" || module.name().starts_with("http/"))
        .map(|module| {
            module
                .format()
                .expect("the module formats")
                .source()
                .to_string()
        })
        .collect::<Vec<String>>()
        .join("\n"))
    }

    fn generate_http_source(
        crate_name: &str,
        source_directory: &Path,
    ) -> Result<String, HttpCodegenError> {
        http_source(crate_name, source_directory, false)
    }

    const RESPONDERS_AND_MIDDLEWARE: &str = r#"
use margaret::framework::http::next::Next;
use margaret::framework::http::request::Request;

#[singleton]
#[responds_to_http(method = "get", path = "/resource", server = "public")]
#[middleware(traced)]
#[middleware(guard)]
struct Resource;

impl Resource {
    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}

#[singleton]
#[responds_to_http(method = "get", path = "/open", server = "public")]
struct Open;

impl Open {
    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}

#[handles_middleware_attribute(attribute = guard)]
struct Guard;

impl Guard {
    #[process]
    fn process(&self, request: &Request, next: Next) -> anyhow::Result<ResponseContinuation> {}
}

#[handles_middleware_attribute(attribute = traced)]
struct Tracer;

impl Tracer {
    #[process]
    fn process(&self, request: &Request, next: Next) -> anyhow::Result<ResponseContinuation> {}
}
"#;

    const ROUTE_PARAMETER: &str = r#"
#[singleton]
#[responds_to_http(method = "get", path = "/users/{id}", server = "public")]
struct GetUser;

impl GetUser {
    #[process]
    fn respond(&self, #[route_parameter(from = "id")] id: String) -> anyhow::Result<Response> {}
}
"#;

    #[test]
    fn rejects_a_responder_absent_from_the_container_plan() {
        assert_eq!(
            discriminant(&rejection_with_container_source(ROUTE_PARAMETER, "")),
            discriminant(&HttpCodegenError::Container {
                source: ContainerError::MissingPlannedProvider { path: any_text() }
            })
        );
    }

    #[test]
    fn rejects_a_route_parameter_binder_absent_from_the_container_plan() {
        let container_source = r#"
#[singleton]
#[responds_to_http(method = "get", path = "/users/{user}", server = "public")]
struct GetUser;

impl GetUser {
    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}
"#;

        assert_eq!(
            discriminant(&rejection_with_container_source(
                CONSOLE_ARGUMENT_BINDER,
                container_source
            )),
            discriminant(&HttpCodegenError::Container {
                source: ContainerError::MissingPlannedProvider { path: any_text() }
            })
        );
    }

    #[test]
    fn rejects_a_middleware_layer_absent_from_the_container_plan() {
        let container_source = r#"
#[singleton]
#[responds_to_http(method = "get", path = "/resource", server = "public")]
struct Resource;

impl Resource {
    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}
"#;

        assert_eq!(
            discriminant(&rejection_with_container_source(
                CONSOLE_ARGUMENT_MIDDLEWARE,
                container_source
            )),
            discriminant(&HttpCodegenError::Container {
                source: ContainerError::MissingPlannedProvider { path: any_text() }
            })
        );
    }

    #[test]
    fn rejects_websocket_arguments_absent_from_the_container_plan() {
        let index = index_for("");
        let bindings = bindings_for(&index);
        let registries = registries_for(&index, false);
        let websocket_arguments = BTreeMap::from([(
            "public".to_string(),
            vec![ServeInput::ConsoleArgument(ConsoleArgument::Flag {
                name: "missing".to_string(),
            })],
        )]);
        let error = render_http(
            &index,
            false,
            &["public".to_string()],
            &[],
            &bindings,
            &websocket_arguments,
            &registries,
        )
        .map(drop)
        .expect_err("websocket arguments must belong to the same container plan");

        assert!(error.to_string().contains("missing"));
    }

    const AUTHENTICATED_RESPONDER: &str = r#"
use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;

struct User;

#[singleton]
#[infers_authenticated_user(user_model = User)]
struct SessionUserProvider;

impl SessionUserProvider {
    #[constructor]
    fn create(#[console_argument(from = "realm")] realm: String) -> anyhow::Result<Self> {}

    #[infer_from_request]
    fn infer(&self, routes: &crate::margaret::routes::Routes) -> anyhow::Result<AuthenticatedUserOutcome<User>> {}
}

#[singleton]
#[responds_to_http(method = "get", path = "/profile", server = "public")]
struct GetProfile;

impl GetProfile {
    #[process]
    fn present(&self, #[authenticated_user] user: User) -> anyhow::Result<Response> {}
}
"#;

    #[test]
    fn builds_the_authenticated_user_provider_once_per_route_handler() {
        let source: String = source_for(AUTHENTICATED_RESPONDER)
            .split_whitespace()
            .collect();

        assert!(source.contains(
            "letsession_user_provider=std::sync::Arc::new(super::super::authenticated_users::SessionUserProvider{inner:container.session_user_provider(),routes:routes.clone(),});"
        ));
        assert!(source.contains("letsession_user_provider=session_user_provider.clone();"));
        assert!(source.contains(
            "margaret::framework::identity::infers_authenticated_user::InfersAuthenticatedUser::infer(session_user_provider.as_ref(),request,).await"
        ));
        assert!(source.contains(
            "margaret::framework::identity::require_authenticated_user::require_authenticated_user(outcome,)"
        ));
    }

    #[test]
    fn reads_a_preconstructed_authenticated_user_provider() {
        let source: String = source_for(AUTHENTICATED_RESPONDER)
            .split_whitespace()
            .collect();

        assert!(source.contains("container.session_user_provider()"));
        assert!(!source.contains("serve_input_"));
    }

    #[test]
    fn calls_the_process_method_by_the_name_the_responder_declares() {
        let source: String = source_for(AUTHENTICATED_RESPONDER)
            .split_whitespace()
            .collect();

        assert!(source.contains("responder.present(user)"));
    }

    #[test]
    fn hands_the_views_to_an_authenticated_user_provider_that_renders_them() {
        let source: String = source_for_with_views(
            "use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;\n\nstruct User;\n\n#[singleton]\n#[infers_authenticated_user(user_model = User)]\nstruct SessionUserProvider;\n\nimpl SessionUserProvider {\n    #[infer_from_request]\n    fn infer(&self, views: &crate::margaret::views::Views) -> anyhow::Result<AuthenticatedUserOutcome<User>> {}\n}\n\n#[singleton]\n#[responds_to_http(method = \"get\", path = \"/profile\", server = \"public\")]\nstruct GetProfile;\n\nimpl GetProfile {\n    #[process]\n    fn respond(&self, #[authenticated_user] user: User) -> anyhow::Result<Response> {}\n}\n",
        )
        .split_whitespace()
        .collect();

        assert!(source.contains("views:views.clone(),"));
    }

    #[test]
    fn rejects_a_route_parameter_binder_that_is_not_a_singleton() {
        assert_eq!(
            discriminant(binding_rejection(&rejection_for(
                "struct User;\n\n#[provides_route_parameter]\nstruct UserBinder;\nimpl HttpRouteParameterBinder for UserBinder {\n    type Model = User;\n    async fn bind(&self, value: String) -> anyhow::Result<RouteParameterBindingOutcome<User>> {}\n}\n"
            )).expect("the responder is rejected by its request bindings")),
            discriminant(&RequestBindingError::RouteParameterBinderRequiresSingleton { binder: any_text() })
        );
    }

    #[test]
    fn rejects_an_authenticated_user_in_a_middleware() {
        assert_eq!(
            discriminant(middleware_binding_rejection(middleware_rejection(&rejection_for(
                "use margaret::framework::http::next::Next;\nuse margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;\n\nstruct User;\n\n#[singleton]\n#[infers_authenticated_user(user_model = User)]\nstruct SessionUserProvider;\n\nimpl SessionUserProvider {\n    #[infer_from_request]\n    fn infer(&self) -> anyhow::Result<AuthenticatedUserOutcome<User>> {}\n}\n\n#[singleton]\n#[handles_middleware_attribute(attribute = guard)]\nstruct Guard;\n\nimpl Guard {\n    #[process]\n    fn process(&self, #[authenticated_user] user: User, next: Next) -> anyhow::Result<ResponseContinuation> {}\n}\n"
            )).expect("the responder is rejected by its middleware")).expect("the middleware is rejected by its request bindings")),
            discriminant(&RequestBindingError::AuthenticatedUserUnavailable { subject: any_text(), parameter: any_text() })
        );
    }

    const COLLIDING_PROVIDER: &str = r#"
use margaret::framework::http::request::Request;
use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;

struct User;

#[singleton]
#[infers_authenticated_user(user_model = User)]
struct Session;

impl Session {
    #[infer_from_request]
    fn infer(&self) -> anyhow::Result<AuthenticatedUserOutcome<User>> {}
}

#[singleton]
#[responds_to_http(method = "get", path = "/profile", server = "public")]
struct GetProfile;

impl GetProfile {
    #[process]
    fn respond(&self, session: &Request, #[authenticated_user] user: User) -> anyhow::Result<Response> {}
}
"#;

    #[test]
    fn keeps_a_captured_provider_clear_of_a_parameter_that_takes_its_name() {
        let source: String = source_for(COLLIDING_PROVIDER).split_whitespace().collect();

        assert!(source.contains(
            "letsession_2=std::sync::Arc::new(super::super::authenticated_users::Session{inner:container.session(),});"
        ));
        assert!(source.contains("letsession_2=session_2.clone();"));
        assert!(source.contains("letsession=request;"));
        assert!(source.contains(
            "margaret::framework::identity::infers_authenticated_user::InfersAuthenticatedUser::infer(session_2.as_ref(),request,)"
        ));
    }

    const COLLIDING_BINDER: &str = r#"
struct User;

struct Filters;

#[singleton]
#[provides_route_parameter]
struct Store;

impl HttpRouteParameterBinder for Store {
    type Model = User;
    async fn bind(&self, value: String) -> anyhow::Result<RouteParameterBindingOutcome<User>> {}
}

#[singleton]
#[responds_to_http(method = "get", path = "/users/{user}", server = "public")]
struct GetUser;

impl GetUser {
    #[process]
    fn respond(
        &self,
        #[form_request(from = Query)] store: Filters,
        #[route_parameter(from = "user")] user: User,
    ) -> anyhow::Result<Response> {}
}
"#;

    #[test]
    fn keeps_a_captured_binder_clear_of_a_parameter_that_takes_its_name() {
        let source: String = source_for(COLLIDING_BINDER).split_whitespace().collect();

        assert!(source.contains("letstore_2=container.store();"));
        assert!(source.contains("letstore_2=store_2.clone();"));
        assert!(source.contains(
            "margaret::framework::http::require_bound_route_parameter::require_bound_route_parameter(request,\"user\",store_2.as_ref(),)"
        ));
    }

    const ASSET_BAG_NAMED_REQUEST: &str = r#"
#[singleton]
#[responds_to_http(method = "get", path = "/assets/{id}", server = "public")]
struct GetAsset;

impl GetAsset {
    #[process]
    fn respond(
        &self,
        request: margaret::framework::asset_bag::asset_bag::AssetBag,
        #[route_parameter(from = "id")] id: String,
    ) -> anyhow::Result<Response> {}
}
"#;

    #[test]
    fn keeps_the_request_clear_of_an_asset_bag_that_takes_its_name() {
        let source: String = source_for(ASSET_BAG_NAMED_REQUEST)
            .split_whitespace()
            .collect();

        assert!(source.contains("request_2:&margaret::framework::http::request::Request"));
        assert!(
            source.contains(
                "letrequest=::margaret::framework::asset_bag::asset_bag::AssetBag::new();"
            )
        );
        assert!(source.contains(
            "margaret::framework::http::require_route_parameter::require_route_parameter(request_2,\"id\",)"
        ));
    }

    const PROVIDER_THAT_ALSO_BINDS: &str = r#"
use margaret::framework::http::request::Request;
use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;

struct User;

struct Article;

#[singleton]
#[infers_authenticated_user(user_model = User)]
#[provides_route_parameter]
struct Store;

impl Store {
    #[infer_from_request]
    fn infer(&self) -> anyhow::Result<AuthenticatedUserOutcome<User>> {}
}

impl HttpRouteParameterBinder for Store {
    type Model = Article;
    async fn bind(&self, value: String) -> anyhow::Result<RouteParameterBindingOutcome<Article>> {}
}

#[singleton]
#[responds_to_http(method = "get", path = "/articles/{article}", server = "public")]
struct GetArticle;

impl GetArticle {
    #[process]
    fn respond(
        &self,
        #[authenticated_user] user: User,
        #[route_parameter(from = "article")] article: Article,
    ) -> anyhow::Result<Response> {}
}
"#;

    #[test]
    fn captures_a_struct_that_provides_a_user_and_binds_a_route_parameter_separately() {
        let source: String = source_for(PROVIDER_THAT_ALSO_BINDS)
            .split_whitespace()
            .collect();

        assert!(source.contains(
            "letstore=std::sync::Arc::new(super::super::authenticated_users::Store{inner:container.store(),});"
        ));
        assert!(source.contains("letstore_2=container.store();"));
        assert!(source.contains(
            "margaret::framework::identity::infers_authenticated_user::InfersAuthenticatedUser::infer(store.as_ref(),request,)"
        ));
        assert!(source.contains(
            "margaret::framework::http::require_bound_route_parameter::require_bound_route_parameter(request,\"article\",store_2.as_ref(),)"
        ));
    }

    const TWICE_BOUND_MODEL: &str = r#"
struct User;

#[singleton]
#[provides_route_parameter]
struct UserBinder;

impl HttpRouteParameterBinder for UserBinder {
    type Model = User;
    async fn bind(&self, value: String) -> anyhow::Result<RouteParameterBindingOutcome<User>> {}
}

#[singleton]
#[responds_to_http(method = "get", path = "/pairs/{author}/{editor}", server = "public")]
struct GetPair;

impl GetPair {
    #[process]
    fn respond(
        &self,
        #[route_parameter(from = "author")] author: User,
        #[route_parameter(from = "editor")] editor: User,
    ) -> anyhow::Result<Response> {}
}
"#;

    #[test]
    fn captures_a_binder_shared_by_two_route_parameters_once() {
        let source: String = source_for(TWICE_BOUND_MODEL).split_whitespace().collect();

        assert_eq!(
            source
                .matches("letuser_binder=container.user_binder();")
                .count(),
            1
        );
        assert_eq!(
            source
                .matches(
                    "margaret::framework::http::require_bound_route_parameter::require_bound_route_parameter(request,"
                )
                .count(),
            2
        );
        assert!(source.contains(
            "margaret::framework::route_parameter_binding::join_route_parameter_bindings::join_route_parameter_bindings("
        ));
        assert!(source.contains("\"author\",user_binder.as_ref(),"));
        assert!(source.contains("\"editor\",user_binder.as_ref(),"));
    }

    const BOUND_MODEL: &str = r#"
struct User;

#[singleton]
#[provides_route_parameter]
struct UserBinder;

impl HttpRouteParameterBinder for UserBinder {
    type Model = User;
    async fn bind(&self, value: String) -> anyhow::Result<RouteParameterBindingOutcome<User>> {}
}

#[singleton]
#[responds_to_http(method = "get", path = "/users/{user}", server = "public")]
struct GetUser;

impl GetUser {
    #[process]
    fn respond(&self, #[route_parameter(from = "user")] user: User) -> anyhow::Result<Response> {}
}
"#;

    const DESTRUCTURED_ROUTE_PARAMETER: &str = r#"
struct User;

#[singleton]
#[provides_route_parameter]
struct UserBinder;

impl HttpRouteParameterBinder for UserBinder {
    type Model = User;
    async fn bind(&self, value: String) -> anyhow::Result<RouteParameterBindingOutcome<User>> {}
}

#[singleton]
#[responds_to_http(method = "get", path = "/users/{id}", server = "public")]
struct GetUser;

impl GetUser {
    #[process]
    fn respond(&self, #[route_parameter(from = "id")] User { name }: User) -> anyhow::Result<Response> {}
}
"#;

    const CURRENT_REQUEST: &str = r#"
use margaret::framework::http::request::Request;

#[singleton]
#[responds_to_http(method = "get", path = "/echo/{id}", server = "public")]
struct Echo;

impl Echo {
    #[process]
    fn respond(&self, request: &Request, #[route_parameter(from = "id")] id: String) -> anyhow::Result<Response> {}
}
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

        AttributeIndexBuilder::new()
            .index_crate(&CrateRoot::new("crate", directory.path().join("src")))
            .expect("the crate is indexed")
            .build()
    }

    fn source_for(lib_source: &str) -> String {
        let directory = crate_with(lib_source);

        generate_http_source("crate", &directory.path().join("src"))
            .expect("the http source is generated")
            .split_whitespace()
            .collect()
    }

    fn source_for_with_views(lib_source: &str) -> String {
        let directory = crate_with(lib_source);

        http_source("crate", &directory.path().join("src"), true)
            .expect("the http source is generated")
            .split_whitespace()
            .collect()
    }

    fn rejection_for(lib_source: &str) -> HttpCodegenError {
        let directory = crate_with(lib_source);

        generate_http_source("crate", &directory.path().join("src"))
            .expect_err("the http source fails to generate")
    }

    fn any_text() -> String {
        "any".to_string()
    }

    fn invalid_method() -> InvalidMethod {
        Method::from_bytes(b"BAD METHOD").expect_err("the fixture is not a valid HTTP method")
    }

    fn binding_rejection(error: &HttpCodegenError) -> Option<&RequestBindingError> {
        match error {
            HttpCodegenError::Binding { source } => Some(source),
            _ => None,
        }
    }

    fn injection_rejection(error: &HttpCodegenError) -> Option<&InjectionError> {
        match error {
            HttpCodegenError::Injection { source } => Some(source),
            _ => None,
        }
    }

    fn middleware_rejection(error: &HttpCodegenError) -> Option<&MiddlewareCodegenError> {
        match error {
            HttpCodegenError::Middleware { source } => Some(source),
            _ => None,
        }
    }

    fn middleware_binding_rejection(
        error: &MiddlewareCodegenError,
    ) -> Option<&RequestBindingError> {
        match error {
            MiddlewareCodegenError::Binding { source } => Some(source),
            _ => None,
        }
    }

    fn middleware_injection_rejection(error: &MiddlewareCodegenError) -> Option<&InjectionError> {
        match error {
            MiddlewareCodegenError::Injection { source } => Some(source),
            _ => None,
        }
    }

    fn middleware_tag_rejection(error: &MiddlewareCodegenError) -> Option<&TagError> {
        match error {
            MiddlewareCodegenError::Tag { source } => Some(source),
            _ => None,
        }
    }

    #[test]
    fn reads_no_wrapped_rejection_from_an_unrelated_error() {
        let http = HttpCodegenError::MissingHttpServer {
            responder: any_text(),
        };
        let middleware = MiddlewareCodegenError::MissingMiddlewareHandles {
            middleware: any_text(),
        };

        assert!(binding_rejection(&http).is_none());
        assert!(injection_rejection(&http).is_none());
        assert!(middleware_rejection(&http).is_none());
        assert!(middleware_binding_rejection(&middleware).is_none());
        assert!(middleware_injection_rejection(&middleware).is_none());
        assert!(middleware_tag_rejection(&middleware).is_none());
    }

    const VIEWS_INJECTION: &str = r#"
#[singleton]
#[responds_to_http(method = "get", path = "/card", server = "public")]
struct GetCard;

impl GetCard {
    #[process]
    fn respond(&self, views: &crate::margaret::views::Views) -> anyhow::Result<Response> {}
}

#[singleton]
#[responds_to_http(method = "get", path = "/health", server = "internal")]
struct GetHealth;

impl GetHealth {
    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}
"#;

    #[test]
    fn injects_views_into_a_responder() {
        let source = source_for_with_views(VIEWS_INJECTION);

        assert!(source.contains(",views:&::std::sync::Arc<super::super::views::Views>"));
        assert!(source.contains(",_views:&::std::sync::Arc<super::super::views::Views>"));
        assert!(source.contains("views.as_ref()"));
        assert!(source.contains("letviews=views.clone();"));
    }

    #[test]
    fn rejects_views_injection_without_declared_views() {
        let rejection = rejection_for(VIEWS_INJECTION);

        assert_eq!(
            discriminant(
                binding_rejection(&rejection)
                    .expect("the responder is rejected by its request bindings")
            ),
            discriminant(&RequestBindingError::ViewsUnavailable {
                subject: any_text(),
                parameter: any_text()
            })
        );
    }

    const HEALTH_RESPONDER: &str = r#"
#[singleton]
#[responds_to_http(method = "get", path = "/health", server = "public")]
struct Health;

impl Health {
    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}
"#;

    fn websocket_http_source(lib_source: &str, websocket_servers: &[String]) -> String {
        websocket_http_source_with_views(lib_source, websocket_servers, false)
    }

    fn websocket_http_source_with_views(
        lib_source: &str,
        websocket_servers: &[String],
        has_views: bool,
    ) -> String {
        let index = index_for(lib_source);
        let registries = registries_for(&index, false);
        let plans =
            middleware_plans(&index, &registries).expect("the middleware plans are collected");
        let bindings = bindings_for(&index);

        render_http(
            &index,
            has_views,
            websocket_servers,
            &plans,
            &bindings,
            &no_websocket_arguments(),
            &registries,
        )
        .expect("the http source is generated")
        .into_modules()
        .into_iter()
        .filter(|module| module.name().starts_with("http/"))
        .map(|module| {
            module
                .format()
                .expect("the module formats")
                .source()
                .to_string()
        })
        .collect::<String>()
        .split_whitespace()
        .collect()
    }

    #[test]
    fn splices_websocket_routes_into_a_server_with_http_routes() {
        let source = websocket_http_source(HEALTH_RESPONDER, &["public".to_string()]);

        assert!(source.contains("super::super::websocket::public_routes(container,routes)"));
    }

    #[test]
    fn generates_a_server_module_for_a_websocket_only_server() {
        let source = websocket_http_source(HEALTH_RESPONDER, &["realtime".to_string()]);

        assert!(source.contains("pub(crate)fnserver_realtime"));
        assert!(source.contains("super::super::websocket::realtime_routes(container,routes)"));
    }

    #[test]
    fn keeps_the_views_out_of_the_websocket_routes_of_a_server_with_views() {
        let source =
            websocket_http_source_with_views(HEALTH_RESPONDER, &["public".to_string()], true);

        assert!(source.contains("super::super::websocket::public_routes(container,routes)"));
        assert!(source.contains("_views:&::std::sync::Arc<super::super::views::Views>"));
    }

    fn routes_source_for(lib_source: &str) -> String {
        let directory = crate_with(lib_source);
        let index = AttributeIndexBuilder::new()
            .index_crate(&CrateRoot::new("crate", directory.path().join("src")))
            .expect("the crate is indexed")
            .build();
        let registries = registries_for(&index, false);
        let plans =
            middleware_plans(&index, &registries).expect("the middleware plans are collected");
        let bindings = bindings_for(&index);

        render_http(
            &index,
            false,
            &[],
            &plans,
            &bindings,
            &no_websocket_arguments(),
            &registries,
        )
        .expect("the http source is generated")
        .into_modules()
        .into_iter()
        .filter(|module| module.name() == "routes" || module.name().starts_with("routes/"))
        .map(|module| {
            module
                .format()
                .expect("the module formats")
                .source()
                .to_string()
        })
        .collect::<Vec<String>>()
        .join("\n")
        .split_whitespace()
        .collect()
    }

    const ROUTES_FIXTURE: &str = r#"
#[singleton]
#[responds_to_http(method = "get", name = "get_greeting", path = "/greeting", server = "public")]
struct GetGreeting;
impl GetGreeting { #[process] fn respond(&self) -> anyhow::Result<Response> {} }

#[singleton]
#[responds_to_http(method = "get", name = "get_article", path = "/articles/{article}", server = "public")]
struct GetArticle;
impl GetArticle { #[process] fn respond(&self, #[route_parameter(from = "article")] article: String) -> anyhow::Result<Response> {} }

#[singleton]
#[responds_to_http(method = "post", name = "post_ping", path = "/ping", server = "public")]
struct PostPing;
impl PostPing { #[process] fn respond(&self) -> anyhow::Result<Response> {} }

#[singleton]
#[responds_to_http(method = "patch", name = "patch_article", path = "/articles/{article}", server = "public")]
struct PatchArticle;
impl PatchArticle { #[process] fn respond(&self, #[route_parameter(from = "article")] article: String) -> anyhow::Result<Response> {} }

#[singleton]
#[responds_to_http(method = "get", name = "get_file", path = "/files/{*rest}", server = "public")]
struct GetFile;
impl GetFile { #[process] fn respond(&self, #[route_parameter(from = "rest")] rest: String) -> anyhow::Result<Response> {} }

#[singleton]
#[responds_to_http(method = "get", path = "/health", server = "internal")]
struct GetHealth;
impl GetHealth { #[process] fn respond(&self) -> anyhow::Result<Response> {} }
"#;

    #[test]
    fn generates_a_forwardable_route_field_for_a_named_paramless_get() {
        let source = routes_source_for(ROUTES_FIXTURE);

        assert!(source.contains(
            "pubget_greeting:margaret::framework::http::forwardable_route::ForwardableRoute,"
        ));
        assert!(source.contains(
            "get_greeting:margaret::framework::http::forwardable_route::ForwardableRoute::new(origin.clone(),::std::vec::Vec::from([margaret::framework::http::url_segment::UrlSegment::Literal(\"/greeting\",),]),)"
        ));
    }

    #[test]
    fn generates_a_positional_forwardable_method_for_a_parameterized_get() {
        let source = routes_source_for(ROUTES_FIXTURE);

        assert!(source.contains(
            "pubfnget_article(&self,article:String,)->margaret::framework::http::forwardable_route::ForwardableRoute"
        ));
        assert!(source.contains(
            "margaret::framework::http::forwardable_route::ForwardableRoute::new(self.origin.clone(),::std::vec::Vec::from([margaret::framework::http::url_segment::UrlSegment::Literal(\"/articles/\",),margaret::framework::http::url_segment::UrlSegment::Parameter(margaret::framework::http::url_parameter::UrlParameter{name:\"article\",value:article,}),]),)"
        ));
        assert!(!source.contains("Params"));
    }

    #[test]
    fn generates_a_catch_all_segment_for_a_named_wildcard_get() {
        let source = routes_source_for(ROUTES_FIXTURE);

        assert!(source.contains(
            "margaret::framework::http::url_segment::UrlSegment::CatchAllParameter(margaret::framework::http::url_parameter::UrlParameter{name:\"rest\",value:rest,})"
        ));
    }

    #[test]
    fn renders_a_named_non_get_route_as_a_plain_route_reference() {
        let source = routes_source_for(ROUTES_FIXTURE);

        assert!(
            source.contains(
                "pubpost_ping:margaret::framework::http::route_reference::RouteReference,"
            )
        );
        assert!(source.contains(
            "pubfnpatch_article(&self,article:String,)->margaret::framework::http::route_reference::RouteReference"
        ));
        assert!(!source.contains("forward_to"));
    }

    #[test]
    fn omits_route_members_for_a_server_without_named_routes() {
        let source = routes_source_for(ROUTES_FIXTURE);

        assert!(source.contains("pubstructInternal{}"));
        assert!(source.contains(
            "implInternal{pub(crate)fnnew(_origin:::std::sync::Arc<str>)->Self{Self{}}}"
        ));
    }

    #[test]
    fn constructs_the_routes_from_origins_in_alphabetical_server_order() {
        let source = routes_source_for(ROUTES_FIXTURE);

        assert!(source.contains(
            "pubfnfrom_origins(origin_internal:::std::sync::Arc<str>,origin_public:::std::sync::Arc<str>,)->Self"
        ));
        assert!(source.contains(
            "internal:servers::internal::Internal::new(origin_internal),public:servers::public::Public::new(origin_public),"
        ));
    }

    #[test]
    fn injects_the_routes_reference_into_a_responder_by_type() {
        let source = source_for(
            "use crate::margaret::routes::Routes;\n\n#[singleton]
#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\nstruct GetX;\nimpl GetX {\n    #[process]\n    fn respond(&self, routes: &Routes) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(source.contains("&::std::sync::Arc<super::super::routes::Routes>"));
        assert!(source.contains("letroutes=routes.clone();"));
        assert!(source.contains("responder.respond(routes.as_ref())"));
    }

    #[test]
    fn injects_a_fresh_asset_bag_into_a_responder_by_value() {
        let source = source_for(
            "use margaret::framework::asset_bag::asset_bag::AssetBag;\n\n#[singleton]
#[responds_to_http(method = \"get\", path = \"/page\", server = \"public\")]\nstruct GetPage;\nimpl GetPage {\n    #[process]\n    fn respond(&self, asset_bag: AssetBag) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(source.contains(
            "|responder:std::sync::Arc<crate::GetPage>,_request:&margaret::framework::http::request::Request"
        ));
        assert!(source.contains(
            "letasset_bag=::margaret::framework::asset_bag::asset_bag::AssetBag::new();"
        ));
        assert!(source.contains("responder.respond(asset_bag)"));
    }

    #[test]
    fn injects_the_peer_spiffe_id_into_a_responder_by_type() {
        let source = source_for(
            "use spiffe::spiffe_id::SpiffeId;\n\n#[singleton]
#[responds_to_http(method = \"get\", path = \"/x\", server = \"internal\")]\nstruct GetX;\nimpl GetX {\n    #[process]\n    fn respond(&self, peer: &SpiffeId) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(source.contains(
            "margaret::framework::http::require_peer_spiffe_id::require_peer_spiffe_id("
        ));
    }

    #[test]
    fn injects_the_scoped_forwarder_into_a_responder_by_type() {
        let source = source_for(
            "use crate::margaret::forwarders::public::Forwarder;\n\n#[singleton]
#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\nstruct GetX;\nimpl GetX {\n    #[process]\n    fn respond(&self, forward: Forwarder) -> anyhow::Result<Forward> {}\n}\n",
        );

        assert!(source.contains("responder.respond(super::super::forwarders::public::Forwarder)"));
    }

    #[test]
    fn rejects_a_user_type_named_routes_that_shadows_the_injectable() {
        let rejection = rejection_for(
            "mod app {\n    pub struct Routes;\n}\n\nuse crate::app::Routes;\n\n#[singleton]
#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\nstruct GetX;\nimpl GetX {\n    #[process]\n    fn respond(&self, routes: &Routes) -> anyhow::Result<Response> {}\n}\n",
        );

        assert_eq!(
            discriminant(
                binding_rejection(&rejection)
                    .expect("the responder is rejected by its request bindings")
            ),
            discriminant(&RequestBindingError::UnmarkedParameter {
                subject: any_text(),
                parameter: any_text()
            })
        );
    }

    #[test]
    fn rejects_a_forwarder_imported_from_a_foreign_server() {
        let rejection = rejection_for(
            "use crate::margaret::forwarders::public::Forwarder;\n\n#[singleton]
#[responds_to_http(method = \"get\", path = \"/x\", server = \"internal\")]\nstruct GetX;\nimpl GetX {\n    #[process]\n    fn respond(&self, forward: Forwarder) -> anyhow::Result<Response> {}\n}\n",
        );

        assert_eq!(
            discriminant(
                binding_rejection(&rejection)
                    .expect("the responder is rejected by its request bindings")
            ),
            discriminant(&RequestBindingError::UnmarkedParameter {
                subject: any_text(),
                parameter: any_text()
            })
        );
    }

    #[test]
    fn rejects_a_peer_spiffe_id_parameter_that_also_carries_a_marker() {
        let rejection = rejection_for(
            "use spiffe::spiffe_id::SpiffeId;\n\n#[singleton]
#[responds_to_http(method = \"get\", path = \"/x/{id}\", server = \"internal\")]\nstruct GetX;\nimpl GetX {\n    #[process]\n    fn respond(&self, #[route_parameter(from = \"id\")] peer: &SpiffeId) -> anyhow::Result<Response> {}\n}\n",
        );

        assert_eq!(
            discriminant(
                binding_rejection(&rejection)
                    .expect("the responder is rejected by its request bindings")
            ),
            discriminant(&RequestBindingError::MarkedPeerSpiffeIdParameter {
                subject: any_text(),
                parameter: any_text()
            })
        );
    }

    #[test]
    fn rejects_a_responder_with_multiple_peer_spiffe_id_parameters() {
        let rejection = rejection_for(
            "use spiffe::spiffe_id::SpiffeId;\n\n#[singleton]
#[responds_to_http(method = \"get\", path = \"/x\", server = \"internal\")]\nstruct GetX;\nimpl GetX {\n    #[process]\n    fn respond(&self, first: &SpiffeId, second: &SpiffeId) -> anyhow::Result<Response> {}\n}\n",
        );

        assert_eq!(
            discriminant(
                binding_rejection(&rejection)
                    .expect("the responder is rejected by its request bindings")
            ),
            discriminant(&RequestBindingError::MultiplePeerSpiffeIdParameters {
                subject: any_text()
            })
        );
    }

    #[test]
    fn rejects_a_route_name_that_is_not_an_identifier() {
        let rejection = rejection_for(
            "#[singleton]
#[responds_to_http(method = \"get\", name = \"not an identifier\", path = \"/x\", server = \"public\")]\nstruct Bad;\nimpl Bad {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert_eq!(
            discriminant(&rejection),
            discriminant(&HttpCodegenError::InvalidRouteName {
                name: any_text(),
                responder: any_text()
            })
        );
    }

    #[test]
    fn rejects_route_paths_that_conflict_on_the_same_server() {
        let rejection = rejection_for(
            "#[singleton]
#[responds_to_http(method = \"get\", path = \"/articles/{article}\", server = \"public\")]\nstruct First;\nimpl First {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n\n#[singleton]
#[responds_to_http(method = \"get\", path = \"/articles/{id}\", server = \"public\")]\nstruct Second;\nimpl Second {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert_eq!(
            discriminant(&rejection),
            discriminant(&HttpCodegenError::ConflictingRoutePaths {
                conflicting_path: any_text(),
                path: any_text(),
                responder: any_text(),
                server: any_text()
            })
        );
    }

    #[test]
    fn rejects_two_responders_registering_the_same_method_and_path() {
        let rejection = rejection_for(
            "#[singleton]
#[responds_to_http(method = \"get\", path = \"/articles\", server = \"public\")]\nstruct First;\nimpl First {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n\n#[singleton]
#[responds_to_http(method = \"get\", path = \"/articles\", server = \"public\")]\nstruct Second;\nimpl Second {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert_eq!(
            discriminant(&rejection),
            discriminant(&HttpCodegenError::DuplicateRoute {
                existing_responder: any_text(),
                method: any_text(),
                path: any_text(),
                responder: any_text(),
                server: any_text()
            })
        );
    }

    #[test]
    fn accepts_the_same_route_path_with_different_methods() {
        let source = source_for(
            "#[singleton]
#[responds_to_http(method = \"get\", path = \"/articles/{article}\", server = \"public\")]\nstruct Read;\nimpl Read {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n\n#[singleton]
#[responds_to_http(method = \"delete\", path = \"/articles/{article}\", server = \"public\")]\nstruct Remove;\nimpl Remove {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(source.contains("\"GET\""));
        assert!(source.contains("\"DELETE\""));
    }

    #[test]
    fn accepts_the_same_route_path_on_different_servers() {
        let source = source_for(
            "#[singleton]
#[responds_to_http(method = \"get\", path = \"/articles/{article}\", server = \"public\")]\nstruct First;\nimpl First {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n\n#[singleton]
#[responds_to_http(method = \"get\", path = \"/articles/{id}\", server = \"internal\")]\nstruct Second;\nimpl Second {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(source.contains("\"/articles/{article}\""));
        assert!(source.contains("\"/articles/{id}\""));
    }

    #[test]
    fn wraps_a_responder_with_its_middleware_in_attribute_order() {
        let source = source_for(RESPONDERS_AND_MIDDLEWARE);

        assert!(source.contains("pub(crate)fnserver"));
        assert!(source.contains("container:&super::super::container::Container"));
        assert!(source.contains("\"GET\""));
        assert!(source.contains(
            "margaret::framework::http::responder_handler::responder_handler(container.open()"
        ));
        assert!(source.contains(
            "|responder:std::sync::Arc<crate::Open>,_request:&margaret::framework::http::request::Request"
        ));
        assert!(source.contains("responder.respond()"));
        assert!(source.contains(
            "margaret::framework::http::layer::layer(std::sync::Arc::new(super::super::middleware::Guard{inner:container.guard(),}),margaret::framework::http::responder_handler::responder_handler(container.resource()"
        ));
        assert!(source.contains(
            "margaret::framework::http::layer::layer(std::sync::Arc::new(super::super::middleware::Tracer{inner:container.tracer(),}),margaret::framework::http::layer::layer(std::sync::Arc::new(super::super::middleware::Guard{"
        ));

        let guard = source.find("container.guard").expect("the guard is wired");
        let tracer = source
            .find("container.tracer")
            .expect("the tracer is wired");

        assert!(tracer < guard);
    }

    #[test]
    fn weaves_routes_into_a_routes_injecting_middleware_onion() {
        let source = source_for(
            "use crate::margaret::routes::Routes;\nuse margaret::framework::http::next::Next;\nuse margaret::framework::http::request::Request;\n\n#[singleton]
#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\n#[middleware(traced)]\nstruct Page;\nimpl Page {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n\n#[handles_middleware_attribute(attribute = traced)]\nstruct Tracer;\nimpl Tracer {\n    #[process]\n    fn process(&self, request: &Request, next: Next, routes: &Routes) -> anyhow::Result<ResponseContinuation> {}\n}\n",
        );

        assert!(source.contains(
            "std::sync::Arc::new(super::super::middleware::Tracer{inner:container.tracer(),routes:routes.clone()"
        ));
    }

    #[test]
    fn injects_route_parameters_into_the_responder() {
        let source = source_for(ROUTE_PARAMETER);

        assert!(source.contains("\"/users/{id}\""));
        assert!(source.contains("container.get_user()"));
        assert!(source.contains(
            "|responder:std::sync::Arc<crate::GetUser>,request:&margaret::framework::http::request::Request"
        ));
        assert!(source.contains(
            r#"letid=matchmargaret::framework::http::require_route_parameter::require_route_parameter(request,"id""#
        ));
        assert!(source.contains("Ok(value)=>value"));
        assert!(source.contains("::std::result::Result::Ok(response.into())"));
        assert!(source.contains("responder.respond(id)"));
    }

    #[test]
    fn awaits_a_responder_that_declares_an_asynchronous_process_method() {
        let source = source_for(
            "#[singleton]\n#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\nstruct GetX;\nimpl GetX {\n    #[process]\n    async fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(source.contains("responder.respond().await"));
    }

    #[test]
    fn injects_distinct_route_parameter_value_types_into_the_responder() {
        let source = source_for(
            r#"
#[route_parameter_value]
struct ProjectSlug(String);

#[route_parameter_value]
struct ProjectId(String);

#[singleton]
#[responds_to_http(method = "get", path = "/projects/{project_slug}/{project_id}", server = "public")]
struct GetProject;

impl GetProject {
    #[process]
    fn respond(
        &self,
        #[route_parameter(from = "project_slug")] project_slug: ProjectSlug,
        #[route_parameter(from = "project_id")] project_id: ProjectId,
    ) -> anyhow::Result<Response> {}
}
"#,
        );

        assert!(source.contains(
            r#"letproject_slug=matchmargaret::framework::http::require_route_parameter::require_route_parameter(request,"project_slug""#
        ));
        assert!(source.contains(
            r#"letproject_id=matchmargaret::framework::http::require_route_parameter::require_route_parameter(request,"project_id""#
        ));
        assert!(source.contains("responder.respond(project_slug,project_id)"));
        assert!(!source.contains("responder.respond(project_slug,project_id).await"));
    }

    #[test]
    fn rejects_a_route_parameter_taken_by_reference() {
        let rejection = rejection_for(
            "#[route_parameter_value]\nstruct ProjectSlug(String);\n\n#[singleton]\n#[responds_to_http(method = \"get\", path = \"/projects/{slug}\", server = \"public\")]\nstruct GetProject;\nimpl GetProject {\n    #[process]\n    fn respond(&self, #[route_parameter(from = \"slug\")] slug: &ProjectSlug) -> anyhow::Result<Response> {}\n}\n",
        );

        assert_eq!(
            discriminant(
                binding_rejection(&rejection)
                    .expect("the responder is rejected by its request bindings")
            ),
            discriminant(&RequestBindingError::RouteParameterByReference {
                subject: any_text(),
                parameter: any_text()
            })
        );
    }

    #[test]
    fn binds_the_current_request_alongside_a_route_parameter() {
        let source = source_for(CURRENT_REQUEST);

        assert!(source.contains(
            "|responder:std::sync::Arc<crate::Echo>,request:&margaret::framework::http::request::Request"
        ));
        assert!(source.contains(
            r#"letid=matchmargaret::framework::http::require_route_parameter::require_route_parameter(request,"id""#
        ));
        assert!(source.contains("responder.respond(request,id)"));
    }

    #[test]
    fn binds_a_current_request_parameter_under_a_custom_name() {
        let source = source_for(
            "use margaret::framework::http::request::Request;\n\n#[singleton]
#[responds_to_http(method = \"get\", path = \"/echo\", server = \"public\")]\nstruct Echo;\n\nimpl Echo {\n    #[process]\n    fn respond(&self, incoming: &Request) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(source.contains("letincoming=request;"));
        assert!(source.contains("responder.respond(incoming)"));
    }

    #[test]
    fn disambiguates_a_route_parameter_named_request_from_the_request_binding() {
        let source = source_for(
            "#[singleton]
#[responds_to_http(method = \"get\", path = \"/x/{request}\", server = \"public\")]\nstruct GetX;\nimpl GetX {\n    #[process]\n    fn respond(&self, #[route_parameter(from = \"request\")] request: String) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(source.contains("request_2:&margaret::framework::http::request::Request"));
        assert!(source.contains(
            r#"letrequest=matchmargaret::framework::http::require_route_parameter::require_route_parameter(request_2,"request""#
        ));
        assert!(source.contains("responder.respond(request)"));
    }

    #[test]
    fn disambiguates_a_route_parameter_named_responder_from_the_responder_binding() {
        let source = source_for(
            "#[singleton]
#[responds_to_http(method = \"get\", path = \"/x/{responder}\", server = \"public\")]\nstruct GetX;\nimpl GetX {\n    #[process]\n    fn respond(&self, #[route_parameter(from = \"responder\")] responder: String) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(source.contains("|responder_2:std::sync::Arc<crate::GetX>"));
        assert!(source.contains(
            r#"letresponder=matchmargaret::framework::http::require_route_parameter::require_route_parameter(request,"responder""#
        ));
        assert!(source.contains("responder_2.respond(responder)"));
    }

    #[test]
    fn binds_a_destructured_route_parameter_named_by_from() {
        let source = source_for(DESTRUCTURED_ROUTE_PARAMETER);

        assert!(source.contains(
            r#"margaret::framework::http::require_bound_route_parameter::require_bound_route_parameter(request,"id",user_binder.as_ref()"#
        ));
        assert!(source.contains("responder.respond(argument_1)"));
    }

    #[test]
    fn injects_a_bound_model() {
        let source = source_for(BOUND_MODEL);

        assert!(source.contains("container.user_binder()"));
        assert!(source.contains(
            r#"margaret::framework::http::require_bound_route_parameter::require_bound_route_parameter(request,"user",user_binder.as_ref()"#
        ));
        assert!(!source.contains("http_route_parameter_binder::HttpRouteParameterBinder"));
        assert!(source.contains("responder.respond(user)"));
        assert!(!source.contains("forbidden"));
    }

    #[test]
    fn ignores_an_unrelated_trait_impl_when_resolving_the_binder_model() {
        let source = source_for(
            "struct User;\n\ntrait Marker {}\n\n#[singleton]\n#[provides_route_parameter]\nstruct UserBinder;\n\nimpl Marker for UserBinder {}\n\nimpl HttpRouteParameterBinder for UserBinder {\n    type Model = User;\n    async fn bind(&self, value: String) -> anyhow::Result<RouteParameterBindingOutcome<User>> {}\n}\n\n#[singleton]
#[responds_to_http(method = \"get\", path = \"/users/{user}\", server = \"public\")]\nstruct GetUser;\n\nimpl GetUser {\n    #[process]\n    fn respond(&self, #[route_parameter(from = \"user\")] user: User) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(source.contains("container.user_binder()"));
    }

    #[test]
    fn reports_a_binder_without_a_model_associated_type() {
        let rejection = rejection_for("#[singleton]\n#[provides_route_parameter]\nstruct Bare;\n");

        assert_eq!(
            discriminant(
                binding_rejection(&rejection)
                    .expect("the responder is rejected by its request bindings")
            ),
            discriminant(&RequestBindingError::RouteParameterBinderModel { binder: any_text() })
        );
    }

    #[test]
    fn reports_a_binder_with_a_non_struct_model() {
        let rejection = rejection_for(
            "#[singleton]\n#[provides_route_parameter]\nstruct UnitBinder;\nimpl HttpRouteParameterBinder for UnitBinder {\n    type Model = ();\n    async fn bind(&self, value: String) -> anyhow::Result<RouteParameterBindingOutcome<()>> {}\n}\n",
        );

        assert_eq!(
            discriminant(
                binding_rejection(&rejection)
                    .expect("the responder is rejected by its request bindings")
            ),
            discriminant(&RequestBindingError::RouteParameterBinderModel { binder: any_text() })
        );
    }

    #[test]
    fn reports_a_binder_whose_model_resolves_to_a_non_struct() {
        let rejection = rejection_for(
            "trait Marker {}\n\n#[singleton]\n#[provides_route_parameter]\nstruct MarkerBinder;\nimpl HttpRouteParameterBinder for MarkerBinder {\n    type Model = Marker;\n    async fn bind(&self, value: String) -> anyhow::Result<RouteParameterBindingOutcome<Marker>> {}\n}\n",
        );

        assert_eq!(
            discriminant(
                binding_rejection(&rejection)
                    .expect("the responder is rejected by its request bindings")
            ),
            discriminant(&RequestBindingError::RouteParameterBinderModel { binder: any_text() })
        );
    }

    #[test]
    fn rejects_a_route_parameter_of_an_unknown_type() {
        let rejection = rejection_for(
            "#[singleton]
#[responds_to_http(method = \"get\", path = \"/things/{thing}\", server = \"public\")]\nstruct GetThing;\nimpl GetThing {\n    #[process]\n    fn respond(&self, #[route_parameter(from = \"thing\")] thing: Unknown) -> anyhow::Result<Response> {}\n}\n",
        );

        assert_eq!(
            discriminant(
                binding_rejection(&rejection)
                    .expect("the responder is rejected by its request bindings")
            ),
            discriminant(&RequestBindingError::MissingRouteParameterResolution {
                subject: any_text(),
                parameter: any_text(),
                written: any_text()
            })
        );
    }

    #[test]
    fn propagates_malformed_route_parameter_arguments() {
        let rejection = rejection_for(
            "#[singleton]
#[responds_to_http(method = \"get\", path = \"/things/{thing}\", server = \"public\")]\nstruct GetThing;\nimpl GetThing {\n    #[process]\n    fn respond(&self, #[route_parameter(= 5)] thing: String) -> anyhow::Result<Response> {}\n}\n",
        );

        assert_eq!(
            discriminant(
                binding_rejection(&rejection)
                    .expect("the responder is rejected by its request bindings")
            ),
            discriminant(&RequestBindingError::Attribute {
                source: AttributeError::GlobImport { file: any_text() }
            })
        );
    }

    #[test]
    fn rejects_two_binders_for_the_same_model() {
        let rejection = rejection_for(
            "struct User;\n\n#[singleton]\n#[provides_route_parameter]\nstruct First;\nimpl HttpRouteParameterBinder for First {\n    type Model = User;\n    async fn bind(&self, value: String) -> anyhow::Result<RouteParameterBindingOutcome<User>> {}\n}\n\n#[singleton]\n#[provides_route_parameter]\nstruct Second;\nimpl HttpRouteParameterBinder for Second {\n    type Model = User;\n    async fn bind(&self, value: String) -> anyhow::Result<RouteParameterBindingOutcome<User>> {}\n}\n",
        );

        assert_eq!(
            discriminant(
                binding_rejection(&rejection)
                    .expect("the responder is rejected by its request bindings")
            ),
            discriminant(&RequestBindingError::AmbiguousRouteParameterBinder {
                model: any_text(),
                first: any_text(),
                second: any_text()
            })
        );
    }

    #[test]
    fn rejects_a_non_struct_route_parameter_binder() {
        let rejection = rejection_for(
            "struct User;\n\n#[provides_route_parameter]\nenum Binder {}\nimpl HttpRouteParameterBinder for Binder {\n    type Model = User;\n    async fn bind(&self, value: String) -> anyhow::Result<RouteParameterBindingOutcome<User>> {}\n}\n",
        );

        assert_eq!(
            discriminant(
                binding_rejection(&rejection)
                    .expect("the responder is rejected by its request bindings")
            ),
            discriminant(&RequestBindingError::RouteParameterBinderNotAStruct {
                binder: any_text()
            })
        );
    }

    #[test]
    fn rejects_a_model_parameter_without_a_binder() {
        let rejection = rejection_for(
            "struct User;\n\n#[singleton]
#[responds_to_http(method = \"get\", path = \"/users/{user}\", server = \"public\")]\nstruct GetUser;\nimpl GetUser {\n    #[process]\n    fn respond(&self, #[route_parameter(from = \"user\")] user: User) -> anyhow::Result<Response> {}\n}\n",
        );

        assert_eq!(
            discriminant(
                binding_rejection(&rejection)
                    .expect("the responder is rejected by its request bindings")
            ),
            discriminant(&RequestBindingError::MissingRouteParameterResolution {
                subject: any_text(),
                parameter: any_text(),
                written: any_text()
            })
        );
    }

    #[test]
    fn rejects_a_responder_without_a_process_method() {
        let rejection = rejection_for(
            "#[singleton]
#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\nstruct Bare;\n",
        );

        assert_eq!(
            discriminant(
                injection_rejection(&rejection)
                    .expect("the responder is rejected by its #[process] runner")
            ),
            discriminant(&InjectionError::MissingProcessMethod { item: any_text() })
        );
    }

    #[test]
    fn rejects_an_unmarked_responder_parameter() {
        let rejection = rejection_for(
            "#[singleton]
#[responds_to_http(method = \"get\", path = \"/x/{id}\", server = \"public\")]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn respond(&self, id: String) -> anyhow::Result<Response> {}\n}\n",
        );

        assert_eq!(
            discriminant(
                binding_rejection(&rejection)
                    .expect("the responder is rejected by its request bindings")
            ),
            discriminant(&RequestBindingError::UnmarkedParameter {
                subject: any_text(),
                parameter: any_text()
            })
        );
    }

    #[test]
    fn rejects_a_non_string_route_parameter_source() {
        let error = rejection_for(
            "#[singleton]
#[responds_to_http(method = \"get\", path = \"/x/{id}\", server = \"public\")]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn respond(&self, #[route_parameter(from = 5)] id: String) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(matches!(
            error,
            HttpCodegenError::Binding {
                source: RequestBindingError::AttributeArguments {
                    source: AttributeArgumentsError::UnexpectedArgument {
                        ref key,
                        ref expected,
                        ..
                    }
                }
            } if key == "from" && expected == "string literal"
        ));
    }

    #[test]
    fn rejects_a_route_parameter_without_from() {
        let rejection = rejection_for(
            "#[singleton]
#[responds_to_http(method = \"get\", path = \"/x/{id}\", server = \"public\")]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn respond(&self, #[route_parameter] id: String) -> anyhow::Result<Response> {}\n}\n",
        );

        assert_eq!(
            discriminant(
                binding_rejection(&rejection)
                    .expect("the responder is rejected by its request bindings")
            ),
            discriminant(&RequestBindingError::RouteParameterMissingFrom {
                subject: any_text(),
                parameter: any_text()
            })
        );
    }

    #[test]
    fn rejects_a_route_parameter_absent_from_the_path() {
        let rejection = rejection_for(
            "#[singleton]
#[responds_to_http(method = \"get\", path = \"/users/{id}\", server = \"public\")]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn respond(&self, #[route_parameter(from = \"slug\")] slug: String) -> anyhow::Result<Response> {}\n}\n",
        );

        assert_eq!(
            discriminant(
                binding_rejection(&rejection)
                    .expect("the responder is rejected by its request bindings")
            ),
            discriminant(&RequestBindingError::RouteParameterNotInPath {
                subject: any_text(),
                parameter: any_text(),
                path: any_text()
            })
        );
    }

    #[test]
    fn rejects_a_malformed_route_path() {
        let rejection = rejection_for(
            "#[singleton]
#[responds_to_http(method = \"get\", path = \"/users/{id\", server = \"public\")]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert_eq!(
            discriminant(&rejection),
            discriminant(&HttpCodegenError::InvalidRoutePath {
                responder: any_text(),
                path: any_text(),
                source: matchit::InsertError::Conflict { with: any_text() }
            })
        );
    }

    #[test]
    fn propagates_an_index_failure() {
        assert_eq!(
            discriminant(&rejection_for("use other::*;\n")),
            discriminant(&HttpCodegenError::Index {
                source: AttributeError::GlobImport { file: any_text() }
            })
        );
    }

    #[test]
    fn rejects_responds_to_http_on_a_non_struct() {
        let rejection = rejection_for(
            "#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\nenum Bad {}\n",
        );

        assert_eq!(
            discriminant(&rejection),
            discriminant(&HttpCodegenError::RespondsToHttpNotOnStruct { target: any_text() })
        );
    }

    #[test]
    fn propagates_malformed_responder_arguments() {
        assert_eq!(
            discriminant(&rejection_for(
                "#[singleton]
#[responds_to_http(= 5, server = \"public\")]\nstruct Bad;\n"
            )),
            discriminant(&HttpCodegenError::Index {
                source: AttributeError::GlobImport { file: any_text() }
            })
        );
    }

    #[test]
    fn propagates_a_non_string_method_argument() {
        let error = rejection_for(
            "#[singleton]
#[responds_to_http(method = 5, path = \"/x\", server = \"public\")]\nstruct Bad;\n",
        );

        assert!(matches!(
            error,
            HttpCodegenError::AttributeArguments {
                source: AttributeArgumentsError::UnexpectedArgument {
                    ref key,
                    ref expected,
                    ..
                }
            } if key == "method" && expected == "string literal"
        ));
    }

    #[test]
    fn rejects_a_malformed_method() {
        let rejection = rejection_for(
            "#[singleton]
#[responds_to_http(method = \"in valid\", path = \"/x\", server = \"public\")]\nstruct Bad;\n",
        );

        assert_eq!(
            discriminant(&rejection),
            discriminant(&HttpCodegenError::InvalidHttpMethod {
                responder: any_text(),
                method: any_text(),
                source: invalid_method()
            })
        );
    }

    #[test]
    fn accepts_the_query_verb() {
        let source = source_for(
            "#[singleton]
#[responds_to_http(method = \"query\", path = \"/search\", server = \"public\")]\nstruct Search;\nimpl Search {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(source.contains(
            "margaret::framework::http::route_entry::RouteEntry::new(\"/search\",::std::vec::Vec::from([margaret::framework::http::method_handler::MethodHandler::anonymous(\"QUERY\",margaret::framework::http::body_intake::BodyIntake::Ignored,"
        ));
    }

    #[test]
    fn rejects_a_responder_without_a_method() {
        assert_eq!(
            discriminant(&rejection_for(
                "#[singleton]
#[responds_to_http(path = \"/x\", server = \"public\")]\nstruct Bad;\n"
            )),
            discriminant(&HttpCodegenError::MissingHttpMethod {
                responder: any_text()
            })
        );
    }

    #[test]
    fn propagates_a_non_string_path_argument() {
        let error = rejection_for(
            "#[singleton]
#[responds_to_http(method = \"get\", path = 5, server = \"public\")]\nstruct Bad;\n",
        );

        assert!(matches!(
            error,
            HttpCodegenError::AttributeArguments {
                source: AttributeArgumentsError::UnexpectedArgument {
                    ref key,
                    ref expected,
                    ..
                }
            } if key == "path" && expected == "string literal"
        ));
    }

    #[test]
    fn rejects_a_responder_without_a_path() {
        assert_eq!(
            discriminant(&rejection_for(
                "#[singleton]
#[responds_to_http(method = \"get\", server = \"public\")]\nstruct Bad;\n"
            )),
            discriminant(&HttpCodegenError::MissingHttpPath {
                responder: any_text()
            })
        );
    }

    #[test]
    fn rejects_http_middleware_on_a_non_struct() {
        let rejection =
            rejection_for("#[handles_middleware_attribute(attribute = x)]\nenum Bad {}\n");

        assert_eq!(
            discriminant(
                middleware_rejection(&rejection)
                    .expect("the responder is rejected by its middleware")
            ),
            discriminant(&MiddlewareCodegenError::MiddlewareHandlerNotOnStruct {
                target: any_text()
            })
        );
    }

    #[test]
    fn propagates_malformed_middleware_arguments() {
        assert_eq!(
            discriminant(
                middleware_rejection(&rejection_for(
                    "#[handles_middleware_attribute(= 5)]\nstruct Bad;\n"
                ))
                .expect("the responder is rejected by its middleware")
            ),
            discriminant(&MiddlewareCodegenError::Index {
                source: AttributeError::GlobImport { file: any_text() }
            })
        );
    }

    #[test]
    fn rejects_middleware_without_handles() {
        assert_eq!(
            discriminant(
                middleware_rejection(&rejection_for(
                    "#[handles_middleware_attribute]\nstruct Bad;\n"
                ))
                .expect("the responder is rejected by its middleware")
            ),
            discriminant(&MiddlewareCodegenError::MissingMiddlewareHandles {
                middleware: any_text()
            })
        );
    }

    #[test]
    fn propagates_a_non_path_handles_argument() {
        let error =
            rejection_for("#[handles_middleware_attribute(attribute = \"x\")]\nstruct Bad;\n");

        assert!(matches!(
            error,
            HttpCodegenError::Middleware {
                source: MiddlewareCodegenError::AttributeArguments {
                    source: AttributeArgumentsError::UnexpectedArgument {
                        ref key,
                        ref expected,
                        ..
                    }
                }
            } if key == "attribute" && expected == "path"
        ));
    }

    #[test]
    fn rejects_a_middleware_without_a_process_method() {
        let rejection =
            rejection_for("#[handles_middleware_attribute(attribute = guard)]\nstruct Bad;\n");

        assert_eq!(
            discriminant(
                middleware_injection_rejection(
                    middleware_rejection(&rejection)
                        .expect("the responder is rejected by its middleware")
                )
                .expect("the middleware is rejected by its #[process] runner")
            ),
            discriminant(&InjectionError::MissingProcessMethod { item: any_text() })
        );
    }

    #[test]
    fn rejects_an_unclassifiable_middleware_parameter() {
        let rejection = rejection_for(
            "#[handles_middleware_attribute(attribute = guard)]\nstruct Bad;\nimpl Bad {\n    #[process]\n    fn process(&self, flag: bool) -> anyhow::Result<ResponseContinuation> {}\n}\n",
        );

        assert_eq!(
            discriminant(
                middleware_binding_rejection(
                    middleware_rejection(&rejection)
                        .expect("the responder is rejected by its middleware")
                )
                .expect("the middleware is rejected by its request bindings")
            ),
            discriminant(&RequestBindingError::UnmarkedMiddlewareParameter {
                subject: any_text(),
                parameter: any_text()
            })
        );
    }

    #[test]
    fn rejects_a_middleware_attribute_without_a_tag() {
        let rejection = rejection_for(
            "#[singleton]
#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\n#[middleware]\nstruct Bad;\nimpl Bad {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert_eq!(
            discriminant(
                middleware_tag_rejection(
                    middleware_rejection(&rejection)
                        .expect("the responder is rejected by its middleware")
                )
                .expect("the middleware is rejected by its tag")
            ),
            discriminant(&TagError::MalformedReference { site: any_text() })
        );
    }

    #[test]
    fn rejects_an_unknown_middleware_tag() {
        let rejection = rejection_for(
            "#[singleton]
#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\n#[middleware(missing)]\nstruct Bad;\nimpl Bad {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert_eq!(
            discriminant(
                middleware_rejection(&rejection)
                    .expect("the responder is rejected by its middleware")
            ),
            discriminant(&MiddlewareCodegenError::UnknownMiddleware {
                site: any_text(),
                tag: any_text()
            })
        );
    }

    #[test]
    fn propagates_malformed_middleware_attribute_arguments() {
        let rejection = rejection_for(
            "#[singleton]
#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\n#[middleware(= 5)]\nstruct Bad;\nimpl Bad {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert_eq!(
            discriminant(
                middleware_rejection(&rejection)
                    .expect("the responder is rejected by its middleware")
            ),
            discriminant(&MiddlewareCodegenError::Index {
                source: AttributeError::GlobImport { file: any_text() }
            })
        );
    }

    #[test]
    fn never_generates_a_route_name_enum_and_omits_names_for_plain_routes() {
        let source = source_for(RESPONDERS_AND_MIDDLEWARE);

        assert!(!source.contains("enumRouteName"));
        assert!(!source.contains("route_with_name"));
        assert!(source.contains("margaret::framework::http::route_entry::RouteEntry::new(\"/open\",::std::vec::Vec::from([margaret::framework::http::method_handler::MethodHandler::anonymous(\"GET\",margaret::framework::http::body_intake::BodyIntake::Ignored,"));
    }

    const REQUEST_BODY_ROUTE: &str = r#"
use margaret::framework::http::bytes::Bytes;

#[singleton]
#[responds_to_http(method = "put", path = "/crates/new", server = "public")]
struct PublishCrate;

impl PublishCrate {
    #[process]
    fn respond(&self, #[request_body] body: &Bytes) -> anyhow::Result<Response> {}
}
"#;

    #[test]
    fn collects_the_body_only_for_a_route_that_declares_it() {
        let source = source_for(REQUEST_BODY_ROUTE);

        assert!(source.contains("letbody=&request.inputs.body;"));
        assert!(source.contains(
            "margaret::framework::http::method_handler::MethodHandler::anonymous(\"PUT\",margaret::framework::http::body_intake::BodyIntake::Collected,"
        ));
    }

    const BODY_INTAKE_CONFLICT_MIDDLEWARE: &str = r#"
use margaret::framework::http::bytes::Bytes;
use margaret::framework::http::next::Next;

struct Credentials;

#[singleton]
#[handles_middleware_attribute(attribute = guard)]
struct Guard;

impl Guard {
    #[process]
    fn process(&self, next: Next, #[form_request(from = Json)] credentials: Credentials) -> anyhow::Result<ResponseContinuation> {}
}

#[singleton]
#[responds_to_http(method = "put", path = "/crates/new", server = "public")]
#[middleware(guard)]
struct PublishCrate;

impl PublishCrate {
    #[process]
    fn respond(&self, #[request_body] body: &Bytes) -> anyhow::Result<Response> {}
}
"#;

    const BODY_INTAKE_CONFLICT_PROVIDER: &str = r#"
use margaret::framework::http::bytes::Bytes;
use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;

struct Credentials;

struct User;

#[singleton]
#[infers_authenticated_user(user_model = User)]
struct SessionUserProvider;

impl SessionUserProvider {
    #[infer_from_request]
    fn infer(&self, #[form_request(from = Json)] credentials: Credentials) -> anyhow::Result<AuthenticatedUserOutcome<User>> {}
}

#[singleton]
#[responds_to_http(method = "put", path = "/crates/new", server = "public")]
struct PublishCrate;

impl PublishCrate {
    #[process]
    fn respond(&self, #[authenticated_user] user: User, #[request_body] body: &Bytes) -> anyhow::Result<Response> {}
}
"#;

    #[test]
    fn rejects_a_middleware_that_parses_a_body_the_route_collects() {
        assert_eq!(
            discriminant(
                binding_rejection(&rejection_for(BODY_INTAKE_CONFLICT_MIDDLEWARE))
                    .expect("the responder is rejected by its request bindings"),
            ),
            discriminant(&RequestBindingError::ConflictingRequestBodyIntake {
                subject: any_text()
            })
        );
    }

    #[test]
    fn rejects_a_provider_that_parses_a_body_the_route_collects() {
        assert_eq!(
            discriminant(
                binding_rejection(&rejection_for(BODY_INTAKE_CONFLICT_PROVIDER))
                    .expect("the responder is rejected by its request bindings"),
            ),
            discriminant(&RequestBindingError::ConflictingRequestBodyIntake {
                subject: any_text()
            })
        );
    }

    const NAMED_ROUTE: &str = r#"
#[singleton]
#[responds_to_http(
    method = "get",
    name = "get_greeting",
    path = "/greeting",
    server = "public"
)]
struct GetGreeting;

impl GetGreeting {
    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}
"#;

    #[test]
    fn registers_a_route_under_an_explicit_name() {
        let source = source_for(NAMED_ROUTE);

        assert!(!source.contains("enumRouteName"));
        assert!(!source.contains("route_with_name"));
        assert!(source.contains("margaret::framework::http::route_entry::RouteEntry::new(\"/greeting\",::std::vec::Vec::from([margaret::framework::http::method_handler::MethodHandler::named(\"GET\",margaret::framework::http::body_intake::BodyIntake::Ignored,\"get_greeting\","));
    }

    #[test]
    fn rejects_a_route_name_that_is_not_snake_case() {
        let rejection = rejection_for(
            "#[singleton]
#[responds_to_http(method = \"get\", name = \"getArticle\", path = \"/a\", server = \"public\")]\nstruct A;\nimpl A {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert_eq!(
            discriminant(&rejection),
            discriminant(&HttpCodegenError::InvalidRouteName {
                name: any_text(),
                responder: any_text()
            })
        );
    }

    #[test]
    fn rejects_a_server_name_that_is_not_snake_case() {
        let rejection = rejection_for(
            "#[singleton]
#[responds_to_http(method = \"get\", path = \"/a\", server = \"PublicApi\")]\nstruct A;\nimpl A {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert_eq!(
            discriminant(&rejection),
            discriminant(&HttpCodegenError::InvalidServerName {
                responder: any_text(),
                server: any_text()
            })
        );
    }

    #[test]
    fn disambiguates_server_names_that_derive_the_same_routes_type() {
        let source = routes_source_for(
            "#[singleton]
#[responds_to_http(method = \"get\", path = \"/x\", server = \"a1\")]\nstruct X;\nimpl X {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n\n#[singleton]
#[responds_to_http(method = \"get\", path = \"/y\", server = \"a_1\")]\nstruct Y;\nimpl Y {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(source.contains("pubstructA1{"));
        assert!(source.contains("pubstructA12{"));
    }

    #[test]
    fn disambiguates_a_route_named_origin_from_the_internal_origin_field() {
        let source = routes_source_for(
            "#[singleton]
#[responds_to_http(method = \"get\", name = \"origin\", path = \"/o\", server = \"public\")]\nstruct O;\nimpl O {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n\n#[singleton]
#[responds_to_http(method = \"get\", name = \"get_x\", path = \"/x/{id}\", server = \"public\")]\nstruct GetX;\nimpl GetX {\n    #[process]\n    fn respond(&self, #[route_parameter(from = \"id\")] id: String) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(source.contains("origin_2:::std::sync::Arc<str>,"));
        assert!(
            source.contains(
                "puborigin:margaret::framework::http::forwardable_route::ForwardableRoute,"
            )
        );
        assert!(source.contains("self.origin_2.clone()"));
    }

    #[test]
    fn disambiguates_a_route_named_new_from_the_generated_constructor() {
        let source = routes_source_for(
            "#[singleton]
#[responds_to_http(method = \"get\", name = \"new\", path = \"/n/{id}\", server = \"public\")]\nstruct New;\nimpl New {\n    #[process]\n    fn respond(&self, #[route_parameter(from = \"id\")] id: String) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(source.contains("pub(crate)fnnew_2(origin:::std::sync::Arc<str>)"));
        assert!(source.contains(
            "pubfnnew(&self,id:String,)->margaret::framework::http::forwardable_route::ForwardableRoute"
        ));
        assert!(source.contains("servers::public::Public::new_2(origin_public)"));
    }

    #[test]
    fn rejects_two_routes_sharing_a_name() {
        let rejection = rejection_for(
            "#[singleton]
#[responds_to_http(method = \"get\", name = \"shared\", path = \"/a\", server = \"public\")]\nstruct A;\nimpl A {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n\n#[singleton]
#[responds_to_http(method = \"get\", name = \"shared\", path = \"/b\", server = \"public\")]\nstruct B;\nimpl B {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert_eq!(
            discriminant(&rejection),
            discriminant(&HttpCodegenError::DuplicateRouteName {
                name: any_text(),
                first: any_text(),
                second: any_text()
            })
        );
    }

    #[test]
    fn propagates_a_non_string_name_argument() {
        let error = rejection_for(
            "#[singleton]
#[responds_to_http(method = \"get\", name = crate::symbols::RouteName::Shared, path = \"/a\", server = \"public\")]\nstruct A;\nimpl A {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(matches!(
            error,
            HttpCodegenError::AttributeArguments {
                source: AttributeArgumentsError::UnexpectedArgument {
                    ref key,
                    ref expected,
                    ..
                }
            } if key == "name" && expected == "string literal"
        ));
    }

    #[test]
    fn wraps_the_responder_return_in_a_response_continuation() {
        let source = source_for(
            "#[singleton]
#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\nstruct Page;\nimpl Page {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(source.contains("responder.respond()"));
        assert!(source.contains(
            "margaret::framework::http::response_continuation::ResponseContinuation::from"
        ));
        assert!(
            source.contains("margaret::framework::http::handler_error::HandlerError::consumer")
        );
    }

    const MULTIPLE_SERVERS: &str = r#"
#[singleton]
#[responds_to_http(method = "get", path = "/", server = "public")]
struct GetIndex;

impl GetIndex {
    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}

#[singleton]
#[responds_to_http(method = "get", path = "/metrics", server = "internal")]
struct GetMetrics;

impl GetMetrics {
    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}
"#;

    #[test]
    fn rejects_a_route_without_a_server() {
        let rejection = rejection_for(
            "#[singleton]
#[responds_to_http(method = \"get\", path = \"/\")]\nstruct GetIndex;\nimpl GetIndex {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert_eq!(
            discriminant(&rejection),
            discriminant(&HttpCodegenError::MissingHttpServer {
                responder: any_text()
            })
        );
    }

    #[test]
    fn groups_each_route_under_its_named_server() {
        let source = source_for(MULTIPLE_SERVERS);

        assert!(source.contains(
            "server_public(container:&super::super::container::Container,routes:&::std::sync::Arc<super::super::routes::Routes>,)->::std::result::Result<margaret::framework::http::server_routes::ServerRoutes,margaret::framework::http::matchit::InsertError,>{margaret::framework::http::server_routes::ServerRoutes::build(::std::vec::Vec::from([margaret::framework::http::route_entry::RouteEntry::new(\"/\","
        ));
        assert!(source.contains(
            "server_internal(container:&super::super::container::Container,routes:&::std::sync::Arc<super::super::routes::Routes>,)->::std::result::Result<margaret::framework::http::server_routes::ServerRoutes,margaret::framework::http::matchit::InsertError,>{margaret::framework::http::server_routes::ServerRoutes::build(::std::vec::Vec::from([margaret::framework::http::route_entry::RouteEntry::new(\"/metrics\","
        ));
    }

    #[test]
    fn propagates_a_non_string_server_argument() {
        let error = rejection_for(
            "#[singleton]
#[responds_to_http(method = \"get\", path = \"/\", server = ServerMarker)]\nstruct Page;\nimpl Page {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(matches!(
            error,
            HttpCodegenError::AttributeArguments {
                source: AttributeArgumentsError::UnexpectedArgument {
                    ref key,
                    ref expected,
                    ..
                }
            } if key == "server" && expected == "string literal"
        ));
    }

    #[test]
    fn injects_a_form_request_from_the_form_source() {
        let source = source_for(
            "use margaret::framework::validation::validation_result::ValidationResult;\n\n#[singleton]
#[responds_to_http(method = \"post\", path = \"/data\", server = \"public\")]\nstruct PostData;\nimpl PostData {\n    #[process]\n    fn respond(&self, #[form_request(from = Form)] data: ValidationResult<Data>) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(source.contains(
            "margaret::framework::http_validation::validate_input::validate_input(request,"
        ));
        assert!(
            source.contains(
                "margaret::framework::http_validation::request_input::RequestInput::Form"
            )
        );
        assert!(source.contains("responder.respond(data)"));
    }

    #[test]
    fn injects_a_form_request_from_the_query_source() {
        let source = source_for(
            "use margaret::framework::validation::validation_result::ValidationResult;\n\n#[singleton]
#[responds_to_http(method = \"get\", path = \"/data\", server = \"public\")]\nstruct GetData;\nimpl GetData {\n    #[process]\n    fn respond(&self, #[form_request(from = Query)] data: ValidationResult<Data>) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(
            source.contains(
                "margaret::framework::http_validation::request_input::RequestInput::Query"
            )
        );
    }

    #[test]
    fn injects_a_form_request_from_the_json_source() {
        let source = source_for(
            "use margaret::framework::validation::validation_result::ValidationResult;\n\n#[singleton]
#[responds_to_http(method = \"post\", path = \"/data\", server = \"public\")]\nstruct ImportData;\nimpl ImportData {\n    #[process]\n    fn respond(&self, #[form_request(from = Json)] data: ValidationResult<Data>) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(
            source.contains(
                "margaret::framework::http_validation::request_input::RequestInput::Json"
            )
        );
    }

    #[test]
    fn injects_a_form_request_from_the_cookie_source() {
        let source = source_for(
            "use margaret::framework::validation::validation_result::ValidationResult;\n\n#[singleton]
#[responds_to_http(method = \"get\", path = \"/data\", server = \"public\")]\nstruct GetData;\nimpl GetData {\n    #[process]\n    fn respond(&self, #[form_request(from = Cookie)] data: ValidationResult<Data>) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(
            source.contains(
                "margaret::framework::http_validation::request_input::RequestInput::Cookie"
            )
        );
    }

    #[test]
    fn rejects_a_form_request_without_a_source() {
        let rejection = rejection_for(
            "#[singleton]
#[responds_to_http(method = \"post\", path = \"/data\", server = \"public\")]\nstruct PostData;\nimpl PostData {\n    #[process]\n    fn respond(&self, #[form_request] data: ValidationResult<Data>) -> anyhow::Result<Response> {}\n}\n",
        );

        assert_eq!(
            discriminant(
                binding_rejection(&rejection)
                    .expect("the responder is rejected by its request bindings")
            ),
            discriminant(&RequestBindingError::FormRequestMissingSource {
                subject: any_text(),
                parameter: any_text()
            })
        );
    }

    #[test]
    fn rejects_a_form_request_with_an_unknown_source() {
        let rejection = rejection_for(
            "#[singleton]
#[responds_to_http(method = \"post\", path = \"/data\", server = \"public\")]\nstruct PostData;\nimpl PostData {\n    #[process]\n    fn respond(&self, #[form_request(from = Headers)] data: ValidationResult<Data>) -> anyhow::Result<Response> {}\n}\n",
        );

        assert_eq!(
            discriminant(
                binding_rejection(&rejection)
                    .expect("the responder is rejected by its request bindings")
            ),
            discriminant(&RequestBindingError::UnknownRequestInput {
                subject: any_text(),
                parameter: any_text(),
                written: any_text()
            })
        );
    }

    #[test]
    fn rejects_an_argument_with_conflicting_markers() {
        let rejection = rejection_for(
            "#[singleton]
#[responds_to_http(method = \"post\", path = \"/data\", server = \"public\")]\nstruct PostData;\nimpl PostData {\n    #[process]\n    fn respond(&self, #[route_parameter(from = \"x\")] #[form_request(from = Form)] data: ValidationResult<Data>) -> anyhow::Result<Response> {}\n}\n",
        );

        assert_eq!(
            discriminant(
                binding_rejection(&rejection)
                    .expect("the responder is rejected by its request bindings")
            ),
            discriminant(&RequestBindingError::ConflictingArgumentMarkers {
                subject: any_text(),
                parameter: any_text()
            })
        );
    }

    #[test]
    fn rejects_a_non_path_form_request_source() {
        let error = rejection_for(
            "#[singleton]
#[responds_to_http(method = \"post\", path = \"/data\", server = \"public\")]\nstruct PostData;\nimpl PostData {\n    #[process]\n    fn respond(&self, #[form_request(from = 5)] data: ValidationResult<Data>) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(matches!(
            error,
            HttpCodegenError::Binding {
                source: RequestBindingError::AttributeArguments {
                    source: AttributeArgumentsError::UnexpectedArgument {
                        ref key,
                        ref expected,
                        ..
                    }
                }
            } if key == "from" && expected == "path"
        ));
    }

    #[test]
    fn propagates_malformed_form_request_arguments() {
        let rejection = rejection_for(
            "#[singleton]
#[responds_to_http(method = \"post\", path = \"/data\", server = \"public\")]\nstruct PostData;\nimpl PostData {\n    #[process]\n    fn respond(&self, #[form_request(= 5)] data: ValidationResult<Data>) -> anyhow::Result<Response> {}\n}\n",
        );

        assert_eq!(
            discriminant(
                binding_rejection(&rejection)
                    .expect("the responder is rejected by its request bindings")
            ),
            discriminant(&RequestBindingError::Attribute {
                source: AttributeError::GlobImport { file: any_text() }
            })
        );
    }

    #[test]
    fn injects_a_guarded_form_request_as_a_bare_model() {
        let source = source_for(
            "#[singleton]
#[responds_to_http(method = \"post\", path = \"/data\", server = \"public\")]\nstruct PostData;\nimpl PostData {\n    #[process]\n    fn respond(&self, #[form_request(from = Form)] data: Data) -> anyhow::Result<Response> {}\n}\n",
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
        assert!(source.contains("responder.respond(data)"));
    }

    #[test]
    fn weaves_views_into_a_views_injecting_middleware_onion() {
        let source = source_for_with_views(
            r#"
use margaret::framework::http::next::Next;

#[singleton]
#[responds_to_http(method = "get", path = "/page", server = "public")]
#[middleware(traced)]
struct Page;

impl Page {
    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}

#[handles_middleware_attribute(attribute = traced)]
struct Tracer;

impl Tracer {
    #[process]
    fn process(&self, next: Next, views: &crate::margaret::views::Views) -> anyhow::Result<ResponseContinuation> {}
}
"#,
        );

        assert!(source.contains(
            "std::sync::Arc::new(super::super::middleware::Tracer{inner:container.tracer(),views:views.clone()"
        ));
        assert!(source.contains("views:&::std::sync::Arc<super::super::views::Views>"));
    }

    #[test]
    fn rejects_middleware_view_injection_without_declared_views() {
        let rejection = rejection_for(
            r#"
use margaret::framework::http::next::Next;

#[singleton]
#[responds_to_http(method = "get", path = "/page", server = "public")]
#[middleware(traced)]
struct Page;

impl Page {
    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}

#[handles_middleware_attribute(attribute = traced)]
struct Tracer;

impl Tracer {
    #[process]
    fn process(&self, next: Next, views: &crate::margaret::views::Views) -> anyhow::Result<ResponseContinuation> {}
}
"#,
        );

        assert_eq!(
            discriminant(
                middleware_binding_rejection(
                    middleware_rejection(&rejection)
                        .expect("the responder is rejected by its middleware")
                )
                .expect("the middleware is rejected by its request bindings")
            ),
            discriminant(&RequestBindingError::ViewsUnavailable {
                subject: any_text(),
                parameter: any_text()
            })
        );
    }

    #[test]
    fn pins_a_server_to_mutual_tls_when_a_middleware_reads_the_peer_spiffe_id() {
        let index = index_for(
            r#"
use margaret::framework::http::next::Next;
use spiffe::spiffe_id::SpiffeId;

#[singleton]
#[responds_to_http(method = "get", path = "/x", server = "internal")]
#[middleware(guard)]
struct GetX;

impl GetX {
    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}

#[handles_middleware_attribute(attribute = guard)]
struct Guard;

impl Guard {
    #[process]
    fn process(&self, peer: &SpiffeId, next: Next) -> anyhow::Result<ResponseContinuation> {}
}
"#,
        );
        let registries = registries_for(&index, false);
        let plans =
            middleware_plans(&index, &registries).expect("the middleware plans are collected");
        let bindings = bindings_for(&index);
        let artifacts = render_http(
            &index,
            false,
            &[],
            &plans,
            &bindings,
            &no_websocket_arguments(),
            &registries,
        )
        .expect("the http source is generated");

        assert!(serves_spiffe(artifacts.servers()));
    }

    #[test]
    fn rejects_the_next_handler_in_a_responder() {
        let rejection = rejection_for(
            r#"
use margaret::framework::http::next::Next;

#[singleton]
#[responds_to_http(method = "get", path = "/x", server = "public")]
struct GetX;

impl GetX {
    #[process]
    fn respond(&self, next: Next) -> anyhow::Result<Response> {}
}
"#,
        );

        assert_eq!(
            discriminant(
                binding_rejection(&rejection)
                    .expect("the responder is rejected by its request bindings")
            ),
            discriminant(&RequestBindingError::NextOutsideMiddleware {
                subject: any_text(),
                parameter: any_text()
            })
        );
    }

    const CONSOLE_ARGUMENT_RESPONDER: &str = r#"
#[singleton]
#[responds_to_http(method = "get", path = "/greeting", server = "public")]
struct Greeting;

impl Greeting {
    #[constructor]
    fn create(#[console_argument(from = "salutation")] salutation: String) -> anyhow::Result<Self> {}

    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}
"#;

    #[test]
    fn weaves_a_serve_input_into_a_responder_and_its_server() {
        let source = source_for(CONSOLE_ARGUMENT_RESPONDER);

        assert!(
            source.contains(
                "pub(crate)fnserver_public(container:&super::super::container::Container,"
            )
        );
        assert!(source.contains("container.greeting()"));
    }

    const CONSOLE_ARGUMENT_BINDER: &str = r#"
struct User;

#[singleton]
#[provides_route_parameter]
struct UserBinder;

impl UserBinder {
    #[constructor]
    fn create(#[console_argument(from = "tenant")] tenant: String) -> anyhow::Result<Self> {}
}

impl HttpRouteParameterBinder for UserBinder {
    type Model = User;
    async fn bind(&self, value: String) -> anyhow::Result<RouteParameterBindingOutcome<User>> {}
}

#[singleton]
#[responds_to_http(method = "get", path = "/users/{user}", server = "public")]
struct GetUser;

impl GetUser {
    #[process]
    fn respond(&self, #[route_parameter(from = "user")] user: User) -> anyhow::Result<Response> {}
}
"#;

    #[test]
    fn reads_a_preconstructed_route_parameter_binder() {
        let source = source_for(CONSOLE_ARGUMENT_BINDER);

        assert!(source.contains("container.user_binder()"));
        assert!(!source.contains("serve_input_"));
    }

    const CONSOLE_ARGUMENT_MIDDLEWARE: &str = r#"
use margaret::framework::http::next::Next;
use margaret::framework::http::request::Request;

#[singleton]
#[responds_to_http(method = "get", path = "/resource", server = "public")]
#[middleware(guard)]
struct Resource;

impl Resource {
    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}

#[singleton]
#[handles_middleware_attribute(attribute = guard)]
struct Guard;

impl Guard {
    #[constructor]
    fn create(#[console_argument(from = "token")] token: String) -> anyhow::Result<Self> {}

    #[process]
    fn process(&self, request: &Request, next: Next) -> anyhow::Result<ResponseContinuation> {}
}
"#;

    #[test]
    fn weaves_a_serve_input_into_a_middleware_and_its_server() {
        let source = source_for(CONSOLE_ARGUMENT_MIDDLEWARE);

        assert!(
            source.contains(
                "pub(crate)fnserver_public(container:&super::super::container::Container,"
            )
        );
        assert!(source.contains("container.guard()"));
    }

    const CONSOLE_ARGUMENT_MIXED_CATEGORIES: &str = r#"
use std::path::PathBuf;

#[singleton]
#[responds_to_http(method = "get", path = "/configured", server = "public")]
struct Configured;

impl Configured {
    #[constructor]
    fn create(
        #[console_argument(from = "label")] label: String,
        #[console_argument(from = "root")] root: PathBuf,
        #[console_argument(from = "tenant")] tenant: String,
        #[console_argument(from = "verbose")] verbose: bool,
        #[console_argument(from = "retries")] retries: Option<u16>,
        #[console_argument(from = "note")] note: Option<String>,
    ) -> anyhow::Result<Self> {}

    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}
"#;

    #[test]
    fn keeps_the_http_boundary_thin_for_a_configured_responder() {
        let source = source_for(CONSOLE_ARGUMENT_MIXED_CATEGORIES);

        assert!(source.contains("container.configured()"));
        assert!(!source.contains("serve_input_"));
    }
}
