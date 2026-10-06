mod active_servers;
mod content_method_tokens;
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
mod route_content;
mod route_group;
pub mod route_location;
mod route_method_tokens;
mod server_route_group;
mod server_serve_inputs;
pub mod server_transport_policy;
pub mod serves_spiffe;
pub mod web_socket_server_requirements;
pub mod web_socket_session_route;

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
    use margaret_attributes::attribute_index::AttributeIndex;
    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_console_argument_codegen::console_argument::ConsoleArgument;
    use margaret_container::container_bindings::ContainerBindings;
    use margaret_container::render_container::render_container;
    use margaret_middleware_codegen::middleware_plans::MiddlewarePlans;
    use margaret_request_binding_codegen::binding_registries::BindingRegistries;
    use margaret_request_binding_codegen::request_binding_error::RequestBindingError;
    use margaret_request_binding_codegen::views_availability::ViewsAvailability;
    use margaret_serve_input_codegen::scan::scan;
    use margaret_serve_input_codegen::serve_input::ServeInput;
    use margaret_tag_codegen::tag_pool::TagPool;

    use crate::http_artifacts::HttpArtifacts;
    use crate::http_codegen_error::HttpCodegenError;
    use crate::http_plan::HttpPlan;
    use crate::render_http;
    use crate::server_transport_policy::ServerTransportPolicy;
    use crate::serves_spiffe::serves_spiffe;
    use crate::web_socket_server_requirements::WebSocketServerRequirements;
    use crate::web_socket_session_route::WebSocketSessionRoute;

    fn render_http(
        index: &AttributeIndex,
        has_views: bool,
        websocket_servers: &BTreeMap<String, WebSocketServerRequirements>,
        middleware_plans: &MiddlewarePlans,
        bindings: &ContainerBindings,
        registries: &BindingRegistries,
    ) -> Result<HttpArtifacts, HttpCodegenError> {
        HttpPlan::build(
            index,
            has_views,
            websocket_servers,
            middleware_plans,
            bindings,
            registries,
        )
        .map(|plan| render_http::render_http(plan, bindings))
    }

    fn bindings_for(index: &AttributeIndex) -> ContainerBindings {
        let registry = scan(index).expect("the console arguments are scanned");

        render_container(index, &registry, &[])
            .expect("the container renders")
            .bindings
    }

    fn no_websocket_servers() -> BTreeMap<String, WebSocketServerRequirements> {
        BTreeMap::new()
    }

    fn websocket_server(
        name: &str,
        transport_policy: ServerTransportPolicy,
    ) -> BTreeMap<String, WebSocketServerRequirements> {
        BTreeMap::from([(
            name.to_string(),
            WebSocketServerRequirements {
                serve_inputs: Vec::new(),
                sessions: Vec::new(),
                transport_policy,
            },
        )])
    }

    fn views_availability(has_views: bool) -> ViewsAvailability {
        if has_views {
            ViewsAvailability::Available
        } else {
            ViewsAvailability::Unavailable
        }
    }

    fn collect_registries(
        index: &AttributeIndex,
        has_views: bool,
    ) -> Result<BindingRegistries, RequestBindingError> {
        BindingRegistries::collect(
            index,
            views_availability(has_views),
            &TagPool::collect(index).expect("the tags are collected"),
            &bindings_for(&IndexedSource::new("").index),
        )
    }

    fn registries_for(index: &AttributeIndex, has_views: bool) -> BindingRegistries {
        collect_registries(index, has_views).expect("the binding registries are collected")
    }

    fn error_with_container_source(source: &str, container_source: &str) -> String {
        let index = IndexedSource::new(source).index;
        let registries = registries_for(&index, false);
        let tags = TagPool::collect(&index).expect("the tags are collected");
        let plans = MiddlewarePlans::collect(&index, &registries, &tags)
            .expect("the middleware plans are collected");
        let container_index = IndexedSource::new(container_source).index;
        let bindings = bindings_for(&container_index);

        render_http(
            &index,
            false,
            &no_websocket_servers(),
            &plans,
            &bindings,
            &registries,
        )
        .map(drop)
        .expect_err("the HTTP plan and container plan must agree")
        .to_string()
    }

    fn http_source(lib_source: &str, has_views: bool) -> Result<String, HttpCodegenError> {
        let index = IndexedSource::try_new(lib_source)?.index;
        let registries = collect_registries(&index, has_views)?;
        let tags = TagPool::collect(&index).expect("the tags are collected");
        let plans = MiddlewarePlans::collect(&index, &registries, &tags)?;
        let bindings = bindings_for(&index);

        Ok(render_http(
            &index,
            has_views,
            &no_websocket_servers(),
            &plans,
            &bindings,
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
        assert!(error_with_container_source(ROUTE_PARAMETER, "").contains("crate::GetUser"));
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

        assert!(
            error_with_container_source(CONSOLE_ARGUMENT_BINDER, container_source)
                .contains("crate::UserBinder")
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

        assert!(
            error_with_container_source(CONSOLE_ARGUMENT_MIDDLEWARE, container_source)
                .contains("crate::Guard")
        );
    }

    fn web_socket_session_error(lib_source: &str, session_path: &str) -> String {
        let index = IndexedSource::new(lib_source).index;
        let registries = registries_for(&index, false);
        let tags = TagPool::collect(&index).expect("the tags are collected");
        let plans = MiddlewarePlans::collect(&index, &registries, &tags)
            .expect("the middleware plans are collected");

        render_http(
            &index,
            false,
            &BTreeMap::from([(
                "public".to_string(),
                WebSocketServerRequirements {
                    serve_inputs: Vec::new(),
                    sessions: vec![WebSocketSessionRoute {
                        path: session_path.to_string(),
                        session: CanonicalPath::new(vec!["crate".to_string(), "Chat".to_string()]),
                    }],
                    transport_policy: ServerTransportPolicy::Negotiable,
                },
            )]),
            &plans,
            &bindings_for(&index),
            &registries,
        )
        .map(drop)
        .expect_err("the websocket session path is rejected")
        .to_string()
    }

    const GREETING_RESPONDER: &str = "#[singleton]
#[responds_to_http(method = \"get\", path = \"/greeting/{name}\", server = \"public\")]\nstruct GetGreeting;\nimpl GetGreeting {\n    #[process]\n    fn respond(&self, #[route_parameter(from = \"name\")] name: String) -> anyhow::Result<Response> {}\n}\n";

    #[test]
    fn rejects_a_websocket_session_at_the_path_of_a_responder() {
        assert_eq!(
            web_socket_session_error(GREETING_RESPONDER, "/greeting/{name}"),
            "websocket session 'crate::Chat' serves '/greeting/{name}' on server 'public', where a responder already serves that path"
        );
    }

    #[test]
    fn rejects_a_websocket_session_path_conflicting_with_a_route() {
        assert_eq!(
            web_socket_session_error(GREETING_RESPONDER, "/greeting/{other}"),
            "websocket session 'crate::Chat' serves '/greeting/{other}' on server 'public', which conflicts with the already registered route path '/greeting/{name}'"
        );
    }

    #[test]
    fn rejects_a_malformed_websocket_session_path() {
        assert!(web_socket_session_error("", "/chat/{unclosed").starts_with(
            "websocket session 'crate::Chat' has a malformed path '/chat/{unclosed': "
        ));
    }

    #[test]
    fn rejects_websocket_arguments_absent_from_the_container_plan() {
        let index = IndexedSource::new("").index;
        let bindings = bindings_for(&index);
        let registries = registries_for(&index, false);
        let websocket_servers = BTreeMap::from([(
            "public".to_string(),
            WebSocketServerRequirements {
                serve_inputs: vec![ServeInput::ConsoleArgument(ConsoleArgument::Flag {
                    name: "missing".to_string(),
                })],
                sessions: Vec::new(),
                transport_policy: ServerTransportPolicy::Negotiable,
            },
        )]);
        let tags = TagPool::collect(&index).expect("the tags are collected");
        let middleware_plans = MiddlewarePlans::collect(&index, &registries, &tags)
            .expect("the middleware plans are collected");
        let error = render_http(
            &index,
            false,
            &websocket_servers,
            &middleware_plans,
            &bindings,
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
            "letsession_user_provider=::std::sync::Arc::new(super::super::authenticated_users::SessionUserProvider{inner:container.session_user_provider(),routes:routes.clone(),});"
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
        assert!(
            error_for(
                "struct User;\n\n#[provides_route_parameter]\nstruct UserBinder;\nimpl HttpRouteParameterBinder for UserBinder {\n    type Model = User;\n    async fn bind(&self, value: String) -> anyhow::Result<RouteParameterBindingOutcome<User>> {}\n}\n"
            )
            .contains("must also be declared as a #[singleton]")
        );
    }

    #[test]
    fn rejects_an_authenticated_user_in_a_middleware() {
        assert!(
            error_for(
                "use margaret::framework::http::next::Next;\nuse margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;\n\nstruct User;\n\n#[singleton]\n#[infers_authenticated_user(user_model = User)]\nstruct SessionUserProvider;\n\nimpl SessionUserProvider {\n    #[infer_from_request]\n    fn infer(&self) -> anyhow::Result<AuthenticatedUserOutcome<User>> {}\n}\n\n#[singleton]\n#[handles_middleware_attribute(attribute = guard)]\nstruct Guard;\n\nimpl Guard {\n    #[process]\n    fn process(&self, #[authenticated_user] user: User, next: Next) -> anyhow::Result<ResponseContinuation> {}\n}\n"
            )
            .contains("only available in an HTTP responder")
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
            "letsession_2=::std::sync::Arc::new(super::super::authenticated_users::Session{inner:container.session(),});"
        ));
        assert!(source.contains("letsession_2=session_2.clone();"));
        assert!(source.contains("letsession=request;"));
        assert!(source.contains(
            "margaret::framework::identity::infers_authenticated_user::InfersAuthenticatedUser::infer(session_2.as_ref(),request,)"
        ));
    }

    const COLLIDING_BINDER: &str = r#"use margaret::framework::http_validation::request_input::RequestInput;


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
        #[form_request(from = RequestInput::Query)] store: Filters,
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
            "letstore=::std::sync::Arc::new(super::super::authenticated_users::Store{inner:container.store(),});"
        ));
        assert!(source.contains("letstore_2=container.store();"));
        assert!(source.contains(
            "margaret::framework::identity::infers_authenticated_user::InfersAuthenticatedUser::infer(store.as_ref(),request,)"
        ));
        assert!(source.contains(
            "margaret::framework::http::require_bound_route_parameter::require_bound_route_parameter(request,\"article\",store_2.as_ref(),)"
        ));
    }

    #[test]
    fn authenticates_the_user_before_binding_a_route_model() {
        let source: String = source_for(PROVIDER_THAT_ALSO_BINDS)
            .split_whitespace()
            .collect();
        let authentication = source
            .find("InfersAuthenticatedUser::infer(")
            .expect("the user is authenticated");
        let binding = source
            .find("require_bound_route_parameter(")
            .expect("the article is bound");

        assert!(authentication < binding);
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

    fn source_for(lib_source: &str) -> String {
        http_source(lib_source, false)
            .expect("the http source is generated")
            .split_whitespace()
            .collect()
    }

    fn source_for_with_views(lib_source: &str) -> String {
        http_source(lib_source, true)
            .expect("the http source is generated")
            .split_whitespace()
            .collect()
    }

    fn rejection_for(lib_source: &str) -> HttpCodegenError {
        http_source(lib_source, false).expect_err("the http source fails to generate")
    }

    fn error_for(lib_source: &str) -> String {
        rejection_for(lib_source).to_string()
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
        let message = error_for(VIEWS_INJECTION);

        assert!(message.contains("requests the views, but this crate generates none"));
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

    fn websocket_http_source(lib_source: &str, websocket_server_name: &str) -> String {
        websocket_http_source_with_views(lib_source, websocket_server_name, false)
    }

    fn websocket_http_source_with_views(
        lib_source: &str,
        websocket_server_name: &str,
        has_views: bool,
    ) -> String {
        let index = IndexedSource::new(lib_source).index;
        let registries = registries_for(&index, false);
        let tags = TagPool::collect(&index).expect("the tags are collected");
        let plans = MiddlewarePlans::collect(&index, &registries, &tags)
            .expect("the middleware plans are collected");
        let bindings = bindings_for(&index);

        render_http(
            &index,
            has_views,
            &websocket_server(websocket_server_name, ServerTransportPolicy::Negotiable),
            &plans,
            &bindings,
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
        let source = websocket_http_source(HEALTH_RESPONDER, "public");

        assert!(source.contains("super::super::websocket::public_routes(container,routes)"));
    }

    #[test]
    fn generates_a_server_module_for_a_websocket_only_server() {
        let source = websocket_http_source(HEALTH_RESPONDER, "realtime");

        assert!(source.contains("pub(crate)fnserver_realtime"));
        assert!(source.contains("super::super::websocket::realtime_routes(container,routes)"));
    }

    #[test]
    fn keeps_the_views_out_of_the_websocket_routes_of_a_server_with_views() {
        let source = websocket_http_source_with_views(HEALTH_RESPONDER, "public", true);

        assert!(source.contains("super::super::websocket::public_routes(container,routes)"));
        assert!(source.contains("_views:&::std::sync::Arc<super::super::views::Views>"));
    }

    fn transport_policies(
        lib_source: &str,
        websocket_servers: &BTreeMap<String, WebSocketServerRequirements>,
    ) -> BTreeMap<String, ServerTransportPolicy> {
        let index = IndexedSource::new(lib_source).index;
        let registries = registries_for(&index, false);
        let tags = TagPool::collect(&index).expect("the tags are collected");
        let plans = MiddlewarePlans::collect(&index, &registries, &tags)
            .expect("the middleware plans are collected");
        let bindings = bindings_for(&index);

        render_http(
            &index,
            false,
            websocket_servers,
            &plans,
            &bindings,
            &registries,
        )
        .expect("the http source is generated")
        .servers()
        .iter()
        .map(|server| (server.name().to_string(), server.transport_policy()))
        .collect()
    }

    #[test]
    fn pins_a_websocket_only_server_whose_sessions_read_the_peer_spiffe_id() {
        let policies = transport_policies(
            HEALTH_RESPONDER,
            &websocket_server("realtime", ServerTransportPolicy::PinnedSpiffeMtls),
        );

        assert_eq!(
            policies,
            BTreeMap::from([
                ("public".to_string(), ServerTransportPolicy::Negotiable),
                (
                    "realtime".to_string(),
                    ServerTransportPolicy::PinnedSpiffeMtls
                ),
            ])
        );
    }

    #[test]
    fn pins_a_shared_server_whose_sessions_read_the_peer_spiffe_id() {
        let policies = transport_policies(
            HEALTH_RESPONDER,
            &websocket_server("public", ServerTransportPolicy::PinnedSpiffeMtls),
        );

        assert_eq!(
            policies,
            BTreeMap::from([(
                "public".to_string(),
                ServerTransportPolicy::PinnedSpiffeMtls
            )])
        );
    }

    #[test]
    fn pins_a_server_whose_route_infers_the_user_from_the_peer_spiffe_id() {
        let policies = transport_policies(
            r#"
use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use spiffe::spiffe_id::SpiffeId;

struct Workload;

#[singleton]
#[infers_authenticated_user(user_model = Workload)]
struct WorkloadProvider;

impl WorkloadProvider {
    #[infer_from_request]
    fn infer(&self, peer: &SpiffeId) -> anyhow::Result<AuthenticatedUserOutcome<Workload>> {}
}

#[singleton]
#[responds_to_http(method = "get", path = "/workload", server = "internal")]
struct GetWorkload;

impl GetWorkload {
    #[process]
    fn respond(&self, #[authenticated_user] workload: Workload) -> anyhow::Result<Response> {}
}
"#,
            &no_websocket_servers(),
        );

        assert_eq!(
            policies,
            BTreeMap::from([(
                "internal".to_string(),
                ServerTransportPolicy::PinnedSpiffeMtls
            )])
        );
    }

    fn routes_source_for(lib_source: &str) -> String {
        let index = IndexedSource::new(lib_source).index;
        let registries = registries_for(&index, false);
        let tags = TagPool::collect(&index).expect("the tags are collected");
        let plans = MiddlewarePlans::collect(&index, &registries, &tags)
            .expect("the middleware plans are collected");
        let bindings = bindings_for(&index);

        render_http(
            &index,
            false,
            &no_websocket_servers(),
            &plans,
            &bindings,
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
    fn moves_the_origin_into_the_last_route_of_a_server_without_parameterized_routes() {
        let source = routes_source_for(
            r#"
#[singleton]
#[responds_to_http(method = "get", name = "get_login", path = "/login", server = "identity")]
struct GetLogin;
impl GetLogin { #[process] fn respond(&self) -> anyhow::Result<Response> {} }

#[singleton]
#[responds_to_http(method = "post", name = "post_consent", path = "/consent", server = "identity")]
struct PostConsent;
impl PostConsent { #[process] fn respond(&self) -> anyhow::Result<Response> {} }
"#,
        );

        assert!(source.contains(
            "implIdentity{pub(crate)fnnew(origin:::std::sync::Arc<str>)->Self{Self{get_login:margaret::framework::http::forwardable_route::ForwardableRoute::new(origin.clone(),::std::vec::Vec::from([margaret::framework::http::url_segment::UrlSegment::Literal(\"/login\"),]),),post_consent:margaret::framework::http::route_reference::RouteReference::new(origin,::std::vec::Vec::from([margaret::framework::http::url_segment::UrlSegment::Literal(\"/consent\",),]),),}}}"
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

        assert!(
            source
                .contains("responder.respond(super::super::forwarders::public::Forwarder::new())")
        );
    }

    #[test]
    fn rejects_a_user_type_named_routes_that_shadows_the_injectable() {
        let message = error_for(
            "mod app {\n    pub struct Routes;\n}\n\nuse crate::app::Routes;\n\n#[singleton]
#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\nstruct GetX;\nimpl GetX {\n    #[process]\n    fn respond(&self, routes: &Routes) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("must be a route parameter"));
    }

    #[test]
    fn rejects_a_forwarder_imported_from_a_foreign_server() {
        let message = error_for(
            "use crate::margaret::forwarders::public::Forwarder;\n\n#[singleton]
#[responds_to_http(method = \"get\", path = \"/x\", server = \"internal\")]\nstruct GetX;\nimpl GetX {\n    #[process]\n    fn respond(&self, forward: Forwarder) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("must be a route parameter"));
    }

    #[test]
    fn rejects_a_peer_spiffe_id_parameter_that_also_carries_a_marker() {
        let message = error_for(
            "use spiffe::spiffe_id::SpiffeId;\n\n#[singleton]
#[responds_to_http(method = \"get\", path = \"/x/{id}\", server = \"internal\")]\nstruct GetX;\nimpl GetX {\n    #[process]\n    fn respond(&self, #[route_parameter(from = \"id\")] peer: &SpiffeId) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("peer SPIFFE id and must not"));
    }

    #[test]
    fn rejects_a_responder_with_multiple_peer_spiffe_id_parameters() {
        let message = error_for(
            "use spiffe::spiffe_id::SpiffeId;\n\n#[singleton]
#[responds_to_http(method = \"get\", path = \"/x\", server = \"internal\")]\nstruct GetX;\nimpl GetX {\n    #[process]\n    fn respond(&self, first: &SpiffeId, second: &SpiffeId) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("more than one peer SPIFFE id"));
    }

    #[test]
    fn rejects_a_route_name_that_is_not_an_identifier() {
        let message = error_for(
            "#[singleton]
#[responds_to_http(method = \"get\", name = \"not an identifier\", path = \"/x\", server = \"public\")]\nstruct Bad;\nimpl Bad {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("must be a snake_case identifier"));
    }

    #[test]
    fn rejects_route_paths_that_conflict_on_the_same_server() {
        let message = error_for(
            "#[singleton]
#[responds_to_http(method = \"get\", path = \"/articles/{article}\", server = \"public\")]\nstruct First;\nimpl First {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n\n#[singleton]
#[responds_to_http(method = \"get\", path = \"/articles/{id}\", server = \"public\")]\nstruct Second;\nimpl Second {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("conflicts with"));
    }

    #[test]
    fn rejects_two_responders_registering_the_same_method_and_path() {
        let message = error_for(
            "#[singleton]
#[responds_to_http(method = \"get\", path = \"/articles\", server = \"public\")]\nstruct First;\nimpl First {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n\n#[singleton]
#[responds_to_http(method = \"get\", path = \"/articles\", server = \"public\")]\nstruct Second;\nimpl Second {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("already registered"));
    }

    #[test]
    fn accepts_the_same_route_path_with_different_methods() {
        let source = source_for(
            "#[singleton]
#[responds_to_http(method = \"get\", path = \"/articles/{article}\", server = \"public\")]\nstruct Read;\nimpl Read {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n\n#[singleton]
#[responds_to_http(method = \"delete\", path = \"/articles/{article}\", server = \"public\")]\nstruct Remove;\nimpl Remove {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(
            source.contains("margaret::framework::route_method::route_method::RouteMethod::Get")
        );
        assert!(
            source.contains("margaret::framework::route_method::route_method::RouteMethod::Delete")
        );
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
        assert!(
            source.contains("margaret::framework::route_method::route_method::RouteMethod::Get")
        );
        assert!(source.contains(
            "margaret::framework::http::head_responder::head_responder(container.open()"
        ));
        assert!(source.contains(
            "|responder:std::sync::Arc<crate::Open>,_request:&margaret::framework::http::request::Request,|"
        ));
        assert!(source.contains("responder.respond()"));
        assert!(source.contains(
            "margaret::framework::http::layer::layer(std::sync::Arc::new(super::super::middleware::Guard{inner:container.guard(),}),margaret::framework::http::head_responder::head_responder(container.resource()"
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
        assert!(source.contains("margaret::framework::http::requirement::Requirement::Met(value"));
        assert!(
            source.contains("margaret::framework::http::requirement::Requirement::Unmet(response")
        );
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
        let message = error_for(
            "#[route_parameter_value]\nstruct ProjectSlug(String);\n\n#[singleton]\n#[responds_to_http(method = \"get\", path = \"/projects/{slug}\", server = \"public\")]\nstruct GetProject;\nimpl GetProject {\n    #[process]\n    fn respond(&self, #[route_parameter(from = \"slug\")] slug: &ProjectSlug) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("is taken by reference"));
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
        let message = error_for("#[singleton]\n#[provides_route_parameter]\nstruct Bare;\n");

        assert!(message.contains("type Model"));
    }

    #[test]
    fn reports_a_binder_with_a_non_struct_model() {
        let message = error_for(
            "#[singleton]\n#[provides_route_parameter]\nstruct UnitBinder;\nimpl HttpRouteParameterBinder for UnitBinder {\n    type Model = ();\n    async fn bind(&self, value: String) -> anyhow::Result<RouteParameterBindingOutcome<()>> {}\n}\n",
        );

        assert!(message.contains("type Model"));
    }

    #[test]
    fn reports_a_binder_whose_model_resolves_to_a_non_struct() {
        let message = error_for(
            "trait Marker {}\n\n#[singleton]\n#[provides_route_parameter]\nstruct MarkerBinder;\nimpl HttpRouteParameterBinder for MarkerBinder {\n    type Model = Marker;\n    async fn bind(&self, value: String) -> anyhow::Result<RouteParameterBindingOutcome<Marker>> {}\n}\n",
        );

        assert!(message.contains("type Model"));
    }

    #[test]
    fn rejects_a_route_parameter_of_an_unknown_type() {
        let message = error_for(
            "#[singleton]
#[responds_to_http(method = \"get\", path = \"/things/{thing}\", server = \"public\")]\nstruct GetThing;\nimpl GetThing {\n    #[process]\n    fn respond(&self, #[route_parameter(from = \"thing\")] thing: Unknown) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("neither declared as a #[route_parameter_value] nor provided by a #[provides_route_parameter]"));
    }

    #[test]
    fn propagates_malformed_route_parameter_arguments() {
        let message = error_for(
            "#[singleton]
#[responds_to_http(method = \"get\", path = \"/things/{thing}\", server = \"public\")]\nstruct GetThing;\nimpl GetThing {\n    #[process]\n    fn respond(&self, #[route_parameter(= 5)] thing: String) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("failed to read a binding attribute"));
    }

    #[test]
    fn rejects_two_binders_for_the_same_model() {
        let message = error_for(
            "struct User;\n\n#[singleton]\n#[provides_route_parameter]\nstruct First;\nimpl HttpRouteParameterBinder for First {\n    type Model = User;\n    async fn bind(&self, value: String) -> anyhow::Result<RouteParameterBindingOutcome<User>> {}\n}\n\n#[singleton]\n#[provides_route_parameter]\nstruct Second;\nimpl HttpRouteParameterBinder for Second {\n    type Model = User;\n    async fn bind(&self, value: String) -> anyhow::Result<RouteParameterBindingOutcome<User>> {}\n}\n",
        );

        assert!(message.contains("more than one route parameter binder"));
    }

    #[test]
    fn rejects_a_non_struct_route_parameter_binder() {
        let message = error_for(
            "struct User;\n\n#[provides_route_parameter]\nenum Binder {}\nimpl HttpRouteParameterBinder for Binder {\n    type Model = User;\n    async fn bind(&self, value: String) -> anyhow::Result<RouteParameterBindingOutcome<User>> {}\n}\n",
        );

        assert!(message.contains("is only supported on structs"));
    }

    #[test]
    fn rejects_a_model_parameter_without_a_binder() {
        let message = error_for(
            "struct User;\n\n#[singleton]
#[responds_to_http(method = \"get\", path = \"/users/{user}\", server = \"public\")]\nstruct GetUser;\nimpl GetUser {\n    #[process]\n    fn respond(&self, #[route_parameter(from = \"user\")] user: User) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("neither declared as a #[route_parameter_value] nor provided by a #[provides_route_parameter]"));
    }

    #[test]
    fn rejects_a_responder_without_a_process_method() {
        let message = error_for(
            "#[singleton]
#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\nstruct Bare;\n",
        );

        assert!(message.contains("no #[process] method"));
    }

    #[test]
    fn rejects_an_unmarked_responder_parameter() {
        let message = error_for(
            "#[singleton]
#[responds_to_http(method = \"get\", path = \"/x/{id}\", server = \"public\")]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn respond(&self, id: String) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("must be a route parameter"));
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
        let message = error_for(
            "#[singleton]
#[responds_to_http(method = \"get\", path = \"/x/{id}\", server = \"public\")]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn respond(&self, #[route_parameter] id: String) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("must name the path parameter it binds"));
    }

    #[test]
    fn rejects_a_route_parameter_absent_from_the_path() {
        let message = error_for(
            "#[singleton]
#[responds_to_http(method = \"get\", path = \"/users/{id}\", server = \"public\")]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn respond(&self, #[route_parameter(from = \"slug\")] slug: String) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("does not appear in the route path"));
    }

    #[test]
    fn rejects_a_malformed_route_path() {
        let message = error_for(
            "#[singleton]
#[responds_to_http(method = \"get\", path = \"/users/{id\", server = \"public\")]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("malformed route path"));
    }

    #[test]
    fn propagates_an_index_failure() {
        assert!(error_for("use other::*;\n").contains("failed to index"));
    }

    #[test]
    fn rejects_responds_to_http_on_a_non_struct() {
        let message = error_for(
            "#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\nenum Bad {}\n",
        );

        assert!(message.contains("#[responds_to_http]"));
    }

    #[test]
    fn propagates_malformed_responder_arguments() {
        assert!(
            error_for(
                "#[singleton]
#[responds_to_http(= 5, server = \"public\")]\nstruct Bad;\n"
            )
            .contains("failed to index")
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
    fn rejects_a_method_outside_the_routable_methods() {
        let error = rejection_for(
            "#[singleton]
#[responds_to_http(method = \"options\", path = \"/x\", server = \"public\")]\nstruct Bad;\n",
        );

        assert!(matches!(
            error,
            HttpCodegenError::UnsupportedHttpMethod { ref method, .. } if method == "options"
        ));
    }

    #[test]
    fn accepts_the_query_verb() {
        let source = source_for(
            "#[singleton]
#[responds_to_http(method = \"query\", path = \"/search\", server = \"public\")]\nstruct Search;\nimpl Search {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(source.contains(
            "margaret::framework::http::route_entry::RouteEntry::new(\"/search\",::std::vec::Vec::from([margaret::framework::http::method_handler::MethodHandler::head(margaret::framework::route_method::route_method::RouteMethod::Query,"
        ));
    }

    #[test]
    fn rejects_a_responder_without_a_method() {
        assert!(
            error_for(
                "#[singleton]
#[responds_to_http(path = \"/x\", server = \"public\")]\nstruct Bad;\n"
            )
            .contains("missing the 'method'")
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
        assert!(
            error_for(
                "#[singleton]
#[responds_to_http(method = \"get\", server = \"public\")]\nstruct Bad;\n"
            )
            .contains("missing the 'path'")
        );
    }

    #[test]
    fn rejects_http_middleware_on_a_non_struct() {
        let message = error_for("#[handles_middleware_attribute(attribute = x)]\nenum Bad {}\n");

        assert!(message.contains("#[handles_middleware_attribute]"));
    }

    #[test]
    fn rejects_a_middleware_without_a_process_method() {
        let message =
            error_for("#[handles_middleware_attribute(attribute = guard)]\nstruct Bad;\n");

        assert!(message.contains("no #[process] method"));
    }

    #[test]
    fn rejects_an_unclassifiable_middleware_parameter() {
        let message = error_for(
            "#[handles_middleware_attribute(attribute = guard)]\nstruct Bad;\nimpl Bad {\n    #[process]\n    fn process(&self, flag: bool) -> anyhow::Result<ResponseContinuation> {}\n}\n",
        );

        assert!(message.contains("must be the current request, the next handler"));
    }

    #[test]
    fn rejects_a_middleware_attribute_without_a_tag() {
        let message = error_for(
            "#[singleton]
#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\n#[middleware]\nstruct Bad;\nimpl Bad {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("must reference exactly one tag"));
    }

    #[test]
    fn rejects_an_unknown_middleware_tag() {
        let message = error_for(
            "#[singleton]
#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\n#[middleware(missing)]\nstruct Bad;\nimpl Bad {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("which no middleware handler declares"));
    }

    #[test]
    fn propagates_malformed_middleware_attribute_arguments() {
        let message = error_for(
            "#[singleton]
#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\n#[middleware(= 5)]\nstruct Bad;\nimpl Bad {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("failed to index"));
    }

    #[test]
    fn never_generates_a_route_name_enum_and_omits_names_for_plain_routes() {
        let source = source_for(RESPONDERS_AND_MIDDLEWARE);

        assert!(!source.contains("enumRouteName"));
        assert!(!source.contains("route_with_name"));
        assert!(source.contains("margaret::framework::http::route_entry::RouteEntry::new(\"/open\",::std::vec::Vec::from([margaret::framework::http::method_handler::MethodHandler::head(margaret::framework::route_method::route_method::RouteMethod::Get,"));
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
        assert!(source.contains("margaret::framework::http::route_entry::RouteEntry::new(\"/greeting\",::std::vec::Vec::from([margaret::framework::http::method_handler::MethodHandler::forwardable(\"get_greeting\","));
    }

    #[test]
    fn rejects_a_route_name_that_is_not_snake_case() {
        let message = error_for(
            "#[singleton]
#[responds_to_http(method = \"get\", name = \"getArticle\", path = \"/a\", server = \"public\")]\nstruct A;\nimpl A {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("must be a snake_case identifier"));
    }

    #[test]
    fn rejects_a_server_name_that_is_not_snake_case() {
        let message = error_for(
            "#[singleton]
#[responds_to_http(method = \"get\", path = \"/a\", server = \"PublicApi\")]\nstruct A;\nimpl A {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("must be a snake_case identifier"));
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
        let message = error_for(
            "#[singleton]
#[responds_to_http(method = \"get\", name = \"shared\", path = \"/a\", server = \"public\")]\nstruct A;\nimpl A {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n\n#[singleton]
#[responds_to_http(method = \"get\", name = \"shared\", path = \"/b\", server = \"public\")]\nstruct B;\nimpl B {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("both declare the route name"));
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

        assert!(
            source.contains("margaret::framework::http::responded::responded(responder.respond())")
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
        let message = error_for(
            "#[singleton]
#[responds_to_http(method = \"get\", path = \"/\")]\nstruct GetIndex;\nimpl GetIndex {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("missing the 'server' argument"));
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
            "use margaret::framework::http_validation::request_input::RequestInput;\n\nuse margaret::framework::validation::validation_result::ValidationResult;\n\n#[singleton]
#[responds_to_http(max_body_bytes = 1024, method = \"post\", path = \"/data\", server = \"public\")]\nstruct PostData;\nimpl PostData {\n    #[process]\n    fn respond(&self, #[form_request(from = RequestInput::Form)] data: ValidationResult<Data>) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(source.contains(
            "letcontent=matchmargaret::framework::http::read_form_fields::read_form_fields(request,body,margaret::framework::http::body_limit::BodyLimit::new(1_024),).await"
        ));
        assert!(
            source
                .contains("letdata=margaret::framework::validation::validate::validate(&content);")
        );
        assert!(source.contains("responder.respond(data)"));
    }

    #[test]
    fn injects_a_form_request_from_the_query_source() {
        let source = source_for(
            "use margaret::framework::http_validation::request_input::RequestInput;\n\nuse margaret::framework::validation::validation_result::ValidationResult;\n\n#[singleton]
#[responds_to_http(method = \"get\", path = \"/data\", server = \"public\")]\nstruct GetData;\nimpl GetData {\n    #[process]\n    fn respond(&self, #[form_request(from = RequestInput::Query)] data: ValidationResult<Data>) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(source.contains(
            "letdata=margaret::framework::validation::validate::validate(&request.inputs.query,);"
        ));
    }

    #[test]
    fn injects_a_form_request_from_the_json_source() {
        let source = source_for(
            "use margaret::framework::http_validation::request_input::RequestInput;\n\nuse margaret::framework::validation::validation_result::ValidationResult;\n\n#[singleton]
#[responds_to_http(max_body_bytes = 2048, method = \"post\", path = \"/data\", server = \"public\")]\nstruct ImportData;\nimpl ImportData {\n    #[process]\n    fn respond(&self, #[form_request(from = RequestInput::Json)] data: ValidationResult<Data>) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(source.contains(
            "letcontent=matchmargaret::framework::http::read_json_value::read_json_value(request,body,margaret::framework::http::body_limit::BodyLimit::new(2_048),).await"
        ));
        assert!(source.contains(
            "letdata=margaret::framework::validation::validate_json::validate_json(&content,);"
        ));
    }

    #[test]
    fn injects_a_form_request_from_the_cookie_source() {
        let source = source_for(
            "use margaret::framework::http_validation::request_input::RequestInput;\n\nuse margaret::framework::validation::validation_result::ValidationResult;\n\n#[singleton]
#[responds_to_http(method = \"get\", path = \"/data\", server = \"public\")]\nstruct GetData;\nimpl GetData {\n    #[process]\n    fn respond(&self, #[form_request(from = RequestInput::Cookie)] data: ValidationResult<Data>) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(source.contains(
            "letdata=margaret::framework::validation::validate::validate(&request.inputs.cookies,);"
        ));
    }

    #[test]
    fn rejects_a_form_request_without_a_source() {
        let message = error_for(
            "#[singleton]
#[responds_to_http(method = \"post\", path = \"/data\", server = \"public\")]\nstruct PostData;\nimpl PostData {\n    #[process]\n    fn respond(&self, #[form_request] data: ValidationResult<Data>) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("must name the request input source it validates"));
    }

    #[test]
    fn rejects_a_form_request_with_an_unknown_source() {
        let message = error_for(
            "#[singleton]
#[responds_to_http(method = \"post\", path = \"/data\", server = \"public\")]\nstruct PostData;\nimpl PostData {\n    #[process]\n    fn respond(&self, #[form_request(from = Headers)] data: ValidationResult<Data>) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("unknown request input source 'Headers'"));
    }

    #[test]
    fn rejects_an_argument_with_conflicting_markers() {
        let message = error_for(
            "use margaret::framework::http_validation::request_input::RequestInput;\n\n#[singleton]
#[responds_to_http(method = \"post\", path = \"/data\", server = \"public\")]\nstruct PostData;\nimpl PostData {\n    #[process]\n    fn respond(&self, #[route_parameter(from = \"x\")] #[form_request(from = RequestInput::Form)] data: ValidationResult<Data>) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("both #[route_parameter] and #[form_request]"));
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
        let message = error_for(
            "#[singleton]
#[responds_to_http(method = \"post\", path = \"/data\", server = \"public\")]\nstruct PostData;\nimpl PostData {\n    #[process]\n    fn respond(&self, #[form_request(= 5)] data: ValidationResult<Data>) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("failed to read a binding attribute"));
    }

    #[test]
    fn injects_a_guarded_form_request_as_a_bare_model() {
        let source = source_for(
            "use margaret::framework::http_validation::request_input::RequestInput;\n\n#[singleton]
#[responds_to_http(max_body_bytes = 1024, method = \"post\", path = \"/data\", server = \"public\")]\nstruct PostData;\nimpl PostData {\n    #[process]\n    fn respond(&self, #[form_request(from = RequestInput::Form)] data: Data) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(source.contains(
            "letdata=matchmargaret::framework::http_validation::require_input::require_input(margaret::framework::validation::validate::validate(&content),)"
        ));
        assert!(source.contains("margaret::framework::http::requirement::Requirement::Met(model)"));
        assert!(source.contains("=>return::std::result::Result::Ok(response)"));
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
        let message = error_for(
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

        assert!(message.contains("requests the views, but this crate generates none"));
    }

    #[test]
    fn pins_a_server_to_mutual_tls_when_a_middleware_reads_the_peer_spiffe_id() {
        let index = IndexedSource::new(
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
        )
        .index;
        let registries = registries_for(&index, false);
        let tags = TagPool::collect(&index).expect("the tags are collected");
        let plans = MiddlewarePlans::collect(&index, &registries, &tags)
            .expect("the middleware plans are collected");
        let bindings = bindings_for(&index);
        let artifacts = render_http(
            &index,
            false,
            &no_websocket_servers(),
            &plans,
            &bindings,
            &registries,
        )
        .expect("the http source is generated");

        assert!(serves_spiffe(artifacts.servers()));
    }

    #[test]
    fn rejects_the_next_handler_in_a_responder() {
        let message = error_for(
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

        assert!(message.contains("only available inside an HTTP middleware"));
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
    const UPLOADED_FILES_IMPORT: &str =
        "use margaret::framework::http_uploaded_file::uploaded_files::UploadedFiles;\n";

    const REQUEST_BODY_STREAM_IMPORT: &str =
        "use margaret::framework::http::request_body_stream::RequestBodyStream;\n";

    #[test]
    fn reads_uploaded_files_after_the_head_bindings() {
        let source = source_for(&format!(
            "{UPLOADED_FILES_IMPORT}#[singleton]
#[responds_to_http(max_body_bytes = 4096, method = \"post\", path = \"/files/{{name}}\", server = \"public\")]\nstruct Upload;\nimpl Upload {{\n    #[process]\n    fn respond(&self, files: UploadedFiles, #[route_parameter(from = \"name\")] name: String) -> anyhow::Result<Response> {{}}\n}}\n"
        ));
        let read = source
            .find("letfiles=matchmargaret::framework::http::read_uploaded_files::read_uploaded_files(request,body,margaret::framework::http::body_limit::BodyLimit::new(4_096),)")
            .expect("the uploaded files are read");
        let route_parameter = source
            .find("letname=matchmargaret::framework::http::require_route_parameter::require_route_parameter(")
            .expect("the route parameter is required");

        assert!(route_parameter < read);
        assert!(source.contains("responder.respond(files,name)"));
    }

    #[test]
    fn reads_form_fields_and_uploaded_files_from_one_multipart_body() {
        let source = source_for(&format!(
            "use margaret::framework::http_validation::request_input::RequestInput;\n{UPLOADED_FILES_IMPORT}#[singleton]
#[responds_to_http(max_body_bytes = 4096, method = \"post\", path = \"/files\", server = \"public\")]\nstruct Upload;\nimpl Upload {{\n    #[process]\n    fn respond(&self, #[form_request(from = RequestInput::Form)] form: Data, files: UploadedFiles) -> anyhow::Result<Response> {{}}\n}}\n"
        ));

        assert!(source.contains(
            "letmargaret::framework::http::multipart_content::MultipartContent{fields:content,files:files,}=matchmargaret::framework::http::read_multipart::read_multipart(request,body,margaret::framework::http::body_limit::BodyLimit::new(4_096),)"
        ));
        assert!(source.contains(
            "margaret::framework::http_validation::require_input::require_input(margaret::framework::validation::validate::validate(&content),)"
        ));
    }

    #[test]
    fn opens_a_raw_body_stream() {
        let source = source_for(&format!(
            "{REQUEST_BODY_STREAM_IMPORT}#[singleton]
#[responds_to_http(max_body_bytes = 1073741824, method = \"put\", path = \"/artifacts\", server = \"public\")]\nstruct Store;\nimpl Store {{\n    #[process]\n    async fn respond(&self, stream: RequestBodyStream) -> anyhow::Result<Response> {{}}\n}}\n"
        ));

        assert!(source.contains(
            "letstream=matchmargaret::framework::http::request_body_stream::RequestBodyStream::open(body,margaret::framework::http::body_limit::BodyLimit::new(1_073_741_824,),).into_requirement()"
        ));
    }

    #[test]
    fn registers_a_named_post_route_as_not_forwardable() {
        let source = source_for(
            "use margaret::framework::http_validation::request_input::RequestInput;\n#[singleton]
#[responds_to_http(max_body_bytes = 64, method = \"post\", name = \"post_note\", path = \"/notes\", server = \"public\")]\nstruct PostNote;\nimpl PostNote {\n    #[process]\n    fn respond(&self, #[form_request(from = RequestInput::Form)] note: Data) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(source.contains(
            "margaret::framework::http::method_handler::MethodHandler::content(margaret::framework::route_method::content_method::ContentMethod::Post,"
        ));
        assert!(!source.contains("forwardable"));
    }

    #[test]
    fn rejects_a_get_route_that_reads_the_request_body() {
        let rejection = rejection_for(&format!(
                "{UPLOADED_FILES_IMPORT}#[singleton]
#[responds_to_http(max_body_bytes = 64, method = \"get\", path = \"/files\", server = \"public\")]\nstruct Upload;\nimpl Upload {{\n    #[process]\n    fn respond(&self, files: UploadedFiles) -> anyhow::Result<Response> {{}}\n}}\n"
            ));

        assert!(matches!(
            rejection,
            HttpCodegenError::ContentOnGetRoute { ref responder } if responder == "crate::Upload"
        ));
    }

    #[test]
    fn rejects_a_route_that_reads_the_request_body_without_a_limit() {
        let rejection = rejection_for(&format!(
                "{UPLOADED_FILES_IMPORT}#[singleton]
#[responds_to_http(method = \"post\", path = \"/files\", server = \"public\")]\nstruct Upload;\nimpl Upload {{\n    #[process]\n    fn respond(&self, files: UploadedFiles) -> anyhow::Result<Response> {{}}\n}}\n"
            ));

        assert!(matches!(
            rejection,
            HttpCodegenError::MissingBodyLimit { ref responder } if responder == "crate::Upload"
        ));
    }

    #[test]
    fn rejects_a_body_limit_on_a_route_that_reads_no_body() {
        let rejection = rejection_for(
                "#[singleton]
#[responds_to_http(max_body_bytes = 64, method = \"post\", path = \"/ping\", server = \"public\")]\nstruct Ping;\nimpl Ping {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n"
            );

        assert!(matches!(
            rejection,
            HttpCodegenError::UnusedBodyLimit { ref responder } if responder == "crate::Ping"
        ));
    }

    #[test]
    fn rejects_a_zero_body_limit() {
        let rejection = rejection_for(&format!(
                "{UPLOADED_FILES_IMPORT}#[singleton]
        #[responds_to_http(max_body_bytes = 0, method = \"post\", path = \"/files\", server = \"public\")]\nstruct Upload;\nimpl Upload {{\n    #[process]\n    fn respond(&self, files: UploadedFiles) -> anyhow::Result<Response> {{}}\n}}\n"
            ));

        assert!(matches!(
            rejection,
            HttpCodegenError::AttributeArguments {
                source: AttributeArgumentsError::MalformedUnsignedInteger { ref key, .. }
            } if key == "max_body_bytes"
        ));
    }

    #[test]
    fn rejects_a_route_that_reads_its_body_in_two_ways() {
        let rejection = rejection_for(&format!(
                "use margaret::framework::http_validation::request_input::RequestInput;\n{REQUEST_BODY_STREAM_IMPORT}#[singleton]
#[responds_to_http(max_body_bytes = 64, method = \"post\", path = \"/notes\", server = \"public\")]\nstruct PostNote;\nimpl PostNote {{\n    #[process]\n    fn respond(&self, #[form_request(from = RequestInput::Json)] note: Data, stream: RequestBodyStream) -> anyhow::Result<Response> {{}}\n}}\n"
            ));

        assert!(matches!(
            rejection,
            HttpCodegenError::Binding {
                source: RequestBindingError::ConflictingContentBindings { ref subject }
            } if subject == "responder 'crate::PostNote'"
        ));
    }
    #[test]
    fn authenticates_the_user_before_reading_the_uploaded_files() {
        let source: String = source_for(
            r#"
use margaret::framework::http_uploaded_file::uploaded_files::UploadedFiles;
use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;

struct User;

#[singleton]
#[infers_authenticated_user(user_model = User)]
struct SessionUserProvider;

impl SessionUserProvider {
    #[infer_from_request]
    fn infer(&self) -> anyhow::Result<AuthenticatedUserOutcome<User>> {}
}

#[singleton]
#[responds_to_http(max_body_bytes = 4096, method = "post", path = "/uploads", server = "public")]
struct Upload;

impl Upload {
    #[process]
    fn respond(&self, files: UploadedFiles, #[authenticated_user] user: User) -> anyhow::Result<Response> {}
}
"#,
        )
        .split_whitespace()
        .collect();
        let authentication = source
            .find("InfersAuthenticatedUser::infer(")
            .expect("the user is authenticated");
        let reading = source
            .find("read_uploaded_files::read_uploaded_files(")
            .expect("the uploaded files are read");

        assert!(authentication < reading);
    }
}
