mod active_servers;
pub mod has_responders;
pub mod http_artifacts;
pub mod http_codegen_error;
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
mod route_group;
mod server_route_group;
pub mod server_transport_policy;
pub mod serves_spiffe;

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::fs;
    use std::path::Path;

    use tempfile::TempDir;
    use tempfile::tempdir;

    use margaret_attributes::attribute_index::AttributeIndex;
    use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
    use margaret_attributes::crate_root::CrateRoot;
    use margaret_console_argument_codegen::console_argument::ConsoleArgument;
    use margaret_console_argument_codegen::scan::scan;
    use margaret_container::container_bindings::ContainerBindings;
    use margaret_container::render_container::render_container;
    use margaret_middleware_codegen::middleware_plans::middleware_plans;
    use margaret_request_binding_codegen::binding_registries::BindingRegistries;
    use margaret_request_binding_codegen::views_availability::ViewsAvailability;

    use crate::has_responders::has_responders;
    use crate::http_codegen_error::HttpCodegenError;
    use crate::render_http::render_http;
    use crate::serves_spiffe::serves_spiffe;

    fn bindings_for(index: &AttributeIndex) -> ContainerBindings {
        let registry = scan(index).expect("the console arguments are scanned");

        render_container(index, &registry, &[])
            .expect("the container renders")
            .bindings
    }

    fn no_websocket_arguments() -> BTreeMap<String, Vec<ConsoleArgument>> {
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

#[responds_to_http(method = "get", path = "/resource", server = "public")]
#[middleware(traced)]
#[middleware(guard)]
struct Resource;

impl Resource {
    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}

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
#[responds_to_http(method = "get", path = "/users/{id}", server = "public")]
struct GetUser;

impl GetUser {
    #[process]
    fn respond(&self, #[route_parameter(from = "id")] id: String) -> anyhow::Result<Response> {}
}
"#;

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
            "letsession_user_provider=std::sync::Arc::new(super::super::authenticated_users::SessionUserProvider{inner:container.session_user_provider(console_argument_0.to_owned()).await?,routes:routes.clone(),});"
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
    fn registers_the_console_arguments_of_an_authenticated_user_provider() {
        let source: String = source_for(AUTHENTICATED_RESPONDER)
            .split_whitespace()
            .collect();

        assert!(source.contains("console_argument_0:&str,"));
    }

    #[test]
    fn calls_the_process_method_by_the_name_the_responder_declares() {
        let source: String = source_for(AUTHENTICATED_RESPONDER)
            .split_whitespace()
            .collect();

        assert!(source.contains("responder.present(user).await"));
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
            "letsession_2=std::sync::Arc::new(super::super::authenticated_users::Session{inner:container.session().await?,});"
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

        assert!(source.contains("letstore_2=container.store().await?;"));
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
            "letstore=std::sync::Arc::new(super::super::authenticated_users::Store{inner:container.store().await?,});"
        ));
        assert!(source.contains("letstore_2=container.store().await?;"));
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
                .matches("letuser_binder=container.user_binder().await?;")
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
            "margaret::framework::http::join_route_parameter_bindings::join_route_parameter_bindings("
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

#[responds_to_http(method = "get", path = "/users/{id}", server = "public")]
struct GetUser;

impl GetUser {
    #[process]
    fn respond(&self, #[route_parameter(from = "id")] User { name }: User) -> anyhow::Result<Response> {}
}
"#;

    const CURRENT_REQUEST: &str = r#"
use margaret::framework::http::request::Request;

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

    fn error_for(lib_source: &str) -> String {
        let directory = crate_with(lib_source);

        generate_http_source("crate", &directory.path().join("src"))
            .expect_err("the http source fails to generate")
            .to_string()
    }

    const VIEWS_INJECTION: &str = r#"
#[responds_to_http(method = "get", path = "/card", server = "public")]
struct GetCard;

impl GetCard {
    #[process]
    fn respond(&self, views: &crate::margaret::views::Views) -> anyhow::Result<Response> {}
}

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
        .collect::<Vec<String>>()
        .join("")
        .split_whitespace()
        .collect()
    }

    #[test]
    fn splices_websocket_routes_into_a_server_with_http_routes() {
        let source = websocket_http_source(HEALTH_RESPONDER, &["public".to_string()]);

        assert!(source.contains("super::super::websocket::public_routes(container,routes).await"));
    }

    #[test]
    fn generates_a_server_module_for_a_websocket_only_server() {
        let source = websocket_http_source(HEALTH_RESPONDER, &["realtime".to_string()]);

        assert!(source.contains("pubasyncfnserver_realtime"));
        assert!(
            source.contains("super::super::websocket::realtime_routes(container,routes).await")
        );
    }

    #[test]
    fn keeps_the_views_out_of_the_websocket_routes_of_a_server_with_views() {
        let source =
            websocket_http_source_with_views(HEALTH_RESPONDER, &["public".to_string()], true);

        assert!(source.contains("super::super::websocket::public_routes(container,routes).await"));
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

        crate::render_http::render_http(
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
#[responds_to_http(method = "get", name = "get_greeting", path = "/greeting", server = "public")]
struct GetGreeting;
impl GetGreeting { #[process] fn respond(&self) -> anyhow::Result<Response> {} }

#[responds_to_http(method = "get", name = "get_article", path = "/articles/{article}", server = "public")]
struct GetArticle;
impl GetArticle { #[process] fn respond(&self, #[route_parameter(from = "article")] article: String) -> anyhow::Result<Response> {} }

#[responds_to_http(method = "post", name = "post_ping", path = "/ping", server = "public")]
struct PostPing;
impl PostPing { #[process] fn respond(&self) -> anyhow::Result<Response> {} }

#[responds_to_http(method = "patch", name = "patch_article", path = "/articles/{article}", server = "public")]
struct PatchArticle;
impl PatchArticle { #[process] fn respond(&self, #[route_parameter(from = "article")] article: String) -> anyhow::Result<Response> {} }

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
            "use crate::margaret::routes::Routes;\n\n#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\nstruct GetX;\nimpl GetX {\n    #[process]\n    fn respond(&self, routes: &Routes) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(source.contains("&::std::sync::Arc<super::super::routes::Routes>"));
        assert!(source.contains("letroutes=routes.clone();"));
        assert!(source.contains("responder.respond(routes.as_ref()).await"));
    }

    #[test]
    fn injects_a_fresh_asset_bag_into_a_responder_by_value() {
        let source = source_for(
            "use margaret::framework::asset_bag::asset_bag::AssetBag;\n\n#[responds_to_http(method = \"get\", path = \"/page\", server = \"public\")]\nstruct GetPage;\nimpl GetPage {\n    #[process]\n    fn respond(&self, asset_bag: AssetBag) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(source.contains(
            "|responder:std::sync::Arc<crate::GetPage>,_request:&margaret::framework::http::request::Request"
        ));
        assert!(source.contains(
            "letasset_bag=::margaret::framework::asset_bag::asset_bag::AssetBag::new();"
        ));
        assert!(source.contains("responder.respond(asset_bag).await"));
    }

    #[test]
    fn injects_the_peer_spiffe_id_into_a_responder_by_type() {
        let source = source_for(
            "use spiffe::spiffe_id::SpiffeId;\n\n#[responds_to_http(method = \"get\", path = \"/x\", server = \"internal\")]\nstruct GetX;\nimpl GetX {\n    #[process]\n    fn respond(&self, peer: &SpiffeId) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(source.contains(
            "margaret::framework::http::require_peer_spiffe_id::require_peer_spiffe_id("
        ));
    }

    #[test]
    fn injects_the_scoped_forwarder_into_a_responder_by_type() {
        let source = source_for(
            "use crate::margaret::forwarders::public::Forwarder;\n\n#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\nstruct GetX;\nimpl GetX {\n    #[process]\n    fn respond(&self, forward: Forwarder) -> anyhow::Result<Forward> {}\n}\n",
        );

        assert!(source.contains("responder.respond(super::super::forwarders::public::Forwarder)"));
    }

    #[test]
    fn rejects_a_user_type_named_routes_that_shadows_the_injectable() {
        let message = error_for(
            "mod app {\n    pub struct Routes;\n}\n\nuse crate::app::Routes;\n\n#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\nstruct GetX;\nimpl GetX {\n    #[process]\n    fn respond(&self, routes: &Routes) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("must be a route parameter"));
    }

    #[test]
    fn rejects_a_forwarder_imported_from_a_foreign_server() {
        let message = error_for(
            "use crate::margaret::forwarders::public::Forwarder;\n\n#[responds_to_http(method = \"get\", path = \"/x\", server = \"internal\")]\nstruct GetX;\nimpl GetX {\n    #[process]\n    fn respond(&self, forward: Forwarder) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("must be a route parameter"));
    }

    #[test]
    fn rejects_a_peer_spiffe_id_parameter_that_also_carries_a_marker() {
        let message = error_for(
            "use spiffe::spiffe_id::SpiffeId;\n\n#[responds_to_http(method = \"get\", path = \"/x/{id}\", server = \"internal\")]\nstruct GetX;\nimpl GetX {\n    #[process]\n    fn respond(&self, #[route_parameter(from = \"id\")] peer: &SpiffeId) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("peer SPIFFE id and must not"));
    }

    #[test]
    fn rejects_a_responder_with_multiple_peer_spiffe_id_parameters() {
        let message = error_for(
            "use spiffe::spiffe_id::SpiffeId;\n\n#[responds_to_http(method = \"get\", path = \"/x\", server = \"internal\")]\nstruct GetX;\nimpl GetX {\n    #[process]\n    fn respond(&self, first: &SpiffeId, second: &SpiffeId) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("more than one peer SPIFFE id"));
    }

    #[test]
    fn rejects_a_route_name_that_is_not_an_identifier() {
        let message = error_for(
            "#[responds_to_http(method = \"get\", name = \"not an identifier\", path = \"/x\", server = \"public\")]\nstruct Bad;\nimpl Bad {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("must be a snake_case identifier"));
    }

    #[test]
    fn rejects_route_paths_that_conflict_on_the_same_server() {
        let message = error_for(
            "#[responds_to_http(method = \"get\", path = \"/articles/{article}\", server = \"public\")]\nstruct First;\nimpl First {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n\n#[responds_to_http(method = \"get\", path = \"/articles/{id}\", server = \"public\")]\nstruct Second;\nimpl Second {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("conflicts with"));
    }

    #[test]
    fn rejects_two_responders_registering_the_same_method_and_path() {
        let message = error_for(
            "#[responds_to_http(method = \"get\", path = \"/articles\", server = \"public\")]\nstruct First;\nimpl First {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n\n#[responds_to_http(method = \"get\", path = \"/articles\", server = \"public\")]\nstruct Second;\nimpl Second {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("already registered"));
    }

    #[test]
    fn accepts_the_same_route_path_with_different_methods() {
        let source = source_for(
            "#[responds_to_http(method = \"get\", path = \"/articles/{article}\", server = \"public\")]\nstruct Read;\nimpl Read {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n\n#[responds_to_http(method = \"delete\", path = \"/articles/{article}\", server = \"public\")]\nstruct Remove;\nimpl Remove {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(source.contains("\"GET\""));
        assert!(source.contains("\"DELETE\""));
    }

    #[test]
    fn accepts_the_same_route_path_on_different_servers() {
        let source = source_for(
            "#[responds_to_http(method = \"get\", path = \"/articles/{article}\", server = \"public\")]\nstruct First;\nimpl First {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n\n#[responds_to_http(method = \"get\", path = \"/articles/{id}\", server = \"internal\")]\nstruct Second;\nimpl Second {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(source.contains("\"/articles/{article}\""));
        assert!(source.contains("\"/articles/{id}\""));
    }

    #[test]
    fn wraps_a_responder_with_its_middleware_in_attribute_order() {
        let source = source_for(RESPONDERS_AND_MIDDLEWARE);

        assert!(source.contains("pubasyncfnserver"));
        assert!(source.contains("container:&super::super::container::Container"));
        assert!(source.contains("\"GET\""));
        assert!(source.contains(
            "margaret::framework::http::responder_handler::responder_handler(container.open().await"
        ));
        assert!(source.contains(
            "|responder:std::sync::Arc<crate::Open>,_request:&margaret::framework::http::request::Request"
        ));
        assert!(source.contains("responder.respond().await"));
        assert!(source.contains(
            "margaret::framework::http::layer::layer(std::sync::Arc::new(super::super::middleware::Guard{inner:container.guard().await?,}),margaret::framework::http::responder_handler::responder_handler(container.resource().await?"
        ));
        assert!(source.contains(
            "margaret::framework::http::layer::layer(std::sync::Arc::new(super::super::middleware::Tracer{inner:container.tracer().await?,}),margaret::framework::http::layer::layer(std::sync::Arc::new(super::super::middleware::Guard{"
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
            "use crate::margaret::routes::Routes;\nuse margaret::framework::http::next::Next;\nuse margaret::framework::http::request::Request;\n\n#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\n#[middleware(traced)]\nstruct Page;\nimpl Page {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n\n#[handles_middleware_attribute(attribute = traced)]\nstruct Tracer;\nimpl Tracer {\n    #[process]\n    fn process(&self, request: &Request, next: Next, routes: &Routes) -> anyhow::Result<ResponseContinuation> {}\n}\n",
        );

        assert!(source.contains(
            "std::sync::Arc::new(super::super::middleware::Tracer{inner:container.tracer().await?,routes:routes.clone()"
        ));
    }

    #[test]
    fn injects_route_parameters_into_the_responder() {
        let source = source_for(ROUTE_PARAMETER);

        assert!(source.contains("\"/users/{id}\""));
        assert!(source.contains("container.get_user().await"));
        assert!(source.contains(
            "|responder:std::sync::Arc<crate::GetUser>,request:&margaret::framework::http::request::Request"
        ));
        assert!(source.contains(
            r#"letid=matchmargaret::framework::http::require_route_parameter::require_route_parameter(request,"id""#
        ));
        assert!(source.contains("Ok(value)=>value"));
        assert!(source.contains("::std::result::Result::Ok(response.into())"));
        assert!(source.contains("responder.respond(id).await"));
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
        assert!(source.contains("responder.respond(request,id).await"));
    }

    #[test]
    fn binds_a_current_request_parameter_under_a_custom_name() {
        let source = source_for(
            "use margaret::framework::http::request::Request;\n\n#[responds_to_http(method = \"get\", path = \"/echo\", server = \"public\")]\nstruct Echo;\n\nimpl Echo {\n    #[process]\n    fn respond(&self, incoming: &Request) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(source.contains("letincoming=request;"));
        assert!(source.contains("responder.respond(incoming).await"));
    }

    #[test]
    fn disambiguates_a_route_parameter_named_request_from_the_request_binding() {
        let source = source_for(
            "#[responds_to_http(method = \"get\", path = \"/x/{request}\", server = \"public\")]\nstruct GetX;\nimpl GetX {\n    #[process]\n    fn respond(&self, #[route_parameter(from = \"request\")] request: String) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(source.contains("request_2:&margaret::framework::http::request::Request"));
        assert!(source.contains(
            r#"letrequest=matchmargaret::framework::http::require_route_parameter::require_route_parameter(request_2,"request""#
        ));
        assert!(source.contains("responder.respond(request).await"));
    }

    #[test]
    fn disambiguates_a_route_parameter_named_responder_from_the_responder_binding() {
        let source = source_for(
            "#[responds_to_http(method = \"get\", path = \"/x/{responder}\", server = \"public\")]\nstruct GetX;\nimpl GetX {\n    #[process]\n    fn respond(&self, #[route_parameter(from = \"responder\")] responder: String) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(source.contains("|responder_2:std::sync::Arc<crate::GetX>"));
        assert!(source.contains(
            r#"letresponder=matchmargaret::framework::http::require_route_parameter::require_route_parameter(request,"responder""#
        ));
        assert!(source.contains("responder_2.respond(responder).await"));
    }

    #[test]
    fn binds_a_destructured_route_parameter_named_by_from() {
        let source = source_for(DESTRUCTURED_ROUTE_PARAMETER);

        assert!(source.contains(
            r#"margaret::framework::http::require_bound_route_parameter::require_bound_route_parameter(request,"id",user_binder.as_ref()"#
        ));
        assert!(source.contains("responder.respond(argument_1).await"));
    }

    #[test]
    fn injects_a_bound_model() {
        let source = source_for(BOUND_MODEL);

        assert!(source.contains("container.user_binder().await"));
        assert!(source.contains(
            r#"margaret::framework::http::require_bound_route_parameter::require_bound_route_parameter(request,"user",user_binder.as_ref()"#
        ));
        assert!(!source.contains("http_route_parameter_binder::HttpRouteParameterBinder"));
        assert!(source.contains("responder.respond(user).await"));
        assert!(!source.contains("forbidden"));
    }

    #[test]
    fn ignores_an_unrelated_trait_impl_when_resolving_the_binder_model() {
        let source = source_for(
            "struct User;\n\ntrait Marker {}\n\n#[singleton]\n#[provides_route_parameter]\nstruct UserBinder;\n\nimpl Marker for UserBinder {}\n\nimpl HttpRouteParameterBinder for UserBinder {\n    type Model = User;\n    async fn bind(&self, value: String) -> anyhow::Result<RouteParameterBindingOutcome<User>> {}\n}\n\n#[responds_to_http(method = \"get\", path = \"/users/{user}\", server = \"public\")]\nstruct GetUser;\n\nimpl GetUser {\n    #[process]\n    fn respond(&self, #[route_parameter(from = \"user\")] user: User) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(source.contains("container.user_binder().await"));
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
            "#[responds_to_http(method = \"get\", path = \"/things/{thing}\", server = \"public\")]\nstruct GetThing;\nimpl GetThing {\n    #[process]\n    fn respond(&self, #[route_parameter(from = \"thing\")] thing: Unknown) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("no #[provides_route_parameter]"));
    }

    #[test]
    fn propagates_malformed_route_parameter_arguments() {
        let message = error_for(
            "#[responds_to_http(method = \"get\", path = \"/things/{thing}\", server = \"public\")]\nstruct GetThing;\nimpl GetThing {\n    #[process]\n    fn respond(&self, #[route_parameter(= 5)] thing: String) -> anyhow::Result<Response> {}\n}\n",
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
            "struct User;\n\n#[responds_to_http(method = \"get\", path = \"/users/{user}\", server = \"public\")]\nstruct GetUser;\nimpl GetUser {\n    #[process]\n    fn respond(&self, #[route_parameter(from = \"user\")] user: User) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("no #[provides_route_parameter]"));
    }

    #[test]
    fn rejects_a_responder_without_a_process_method() {
        let message = error_for(
            "#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\nstruct Bare;\n",
        );

        assert!(message.contains("no #[process] method"));
    }

    #[test]
    fn rejects_an_unmarked_responder_parameter() {
        let message = error_for(
            "#[responds_to_http(method = \"get\", path = \"/x/{id}\", server = \"public\")]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn respond(&self, id: String) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("must be a route parameter"));
    }

    #[test]
    fn rejects_a_non_string_route_parameter_source() {
        let message = error_for(
            "#[responds_to_http(method = \"get\", path = \"/x/{id}\", server = \"public\")]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn respond(&self, #[route_parameter(from = 5)] id: String) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("failed to read a binding attribute"));
    }

    #[test]
    fn rejects_a_route_parameter_without_from() {
        let message = error_for(
            "#[responds_to_http(method = \"get\", path = \"/x/{id}\", server = \"public\")]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn respond(&self, #[route_parameter] id: String) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("must name the path parameter it binds"));
    }

    #[test]
    fn rejects_a_route_parameter_absent_from_the_path() {
        let message = error_for(
            "#[responds_to_http(method = \"get\", path = \"/users/{id}\", server = \"public\")]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn respond(&self, #[route_parameter(from = \"slug\")] slug: String) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("does not appear in the route path"));
    }

    #[test]
    fn rejects_a_malformed_route_path() {
        let message = error_for(
            "#[responds_to_http(method = \"get\", path = \"/users/{id\", server = \"public\")]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
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
            error_for("#[responds_to_http(= 5, server = \"public\")]\nstruct Bad;\n")
                .contains("failed to index")
        );
    }

    #[test]
    fn propagates_a_non_string_method_argument() {
        let message = error_for(
            "#[responds_to_http(method = 5, path = \"/x\", server = \"public\")]\nstruct Bad;\n",
        );

        assert!(message.contains("failed to index"));
    }

    #[test]
    fn rejects_a_malformed_method() {
        let message = error_for(
            "#[responds_to_http(method = \"in valid\", path = \"/x\", server = \"public\")]\nstruct Bad;\n",
        );

        assert!(message.contains("invalid HTTP method"));
    }

    #[test]
    fn accepts_the_query_verb() {
        let source = source_for(
            "#[responds_to_http(method = \"query\", path = \"/search\", server = \"public\")]\nstruct Search;\nimpl Search {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(source.contains(
            "margaret::framework::http::route_entry::RouteEntry::new(\"/search\",::std::vec::Vec::from([margaret::framework::http::method_handler::MethodHandler::new(\"QUERY\","
        ));
    }

    #[test]
    fn rejects_a_responder_without_a_method() {
        assert!(
            error_for("#[responds_to_http(path = \"/x\", server = \"public\")]\nstruct Bad;\n")
                .contains("missing the 'method'")
        );
    }

    #[test]
    fn propagates_a_non_string_path_argument() {
        let message = error_for(
            "#[responds_to_http(method = \"get\", path = 5, server = \"public\")]\nstruct Bad;\n",
        );

        assert!(message.contains("failed to index"));
    }

    #[test]
    fn rejects_a_responder_without_a_path() {
        assert!(
            error_for("#[responds_to_http(method = \"get\", server = \"public\")]\nstruct Bad;\n")
                .contains("missing the 'path'")
        );
    }

    #[test]
    fn rejects_http_middleware_on_a_non_struct() {
        let message = error_for("#[handles_middleware_attribute(attribute = x)]\nenum Bad {}\n");

        assert!(message.contains("#[handles_middleware_attribute]"));
    }

    #[test]
    fn propagates_malformed_middleware_arguments() {
        assert!(
            error_for("#[handles_middleware_attribute(= 5)]\nstruct Bad;\n")
                .contains("failed to index")
        );
    }

    #[test]
    fn rejects_middleware_without_handles() {
        assert!(
            error_for("#[handles_middleware_attribute]\nstruct Bad;\n")
                .contains("missing the 'attribute'")
        );
    }

    #[test]
    fn propagates_a_non_path_handles_argument() {
        let message =
            error_for("#[handles_middleware_attribute(attribute = \"x\")]\nstruct Bad;\n");

        assert!(message.contains("failed to index"));
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
            "#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\n#[middleware]\nstruct Bad;\nimpl Bad {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("must reference exactly one tag"));
    }

    #[test]
    fn rejects_an_unknown_middleware_tag() {
        let message = error_for(
            "#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\n#[middleware(missing)]\nstruct Bad;\nimpl Bad {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("no #[handles_middleware_attribute] handles it"));
    }

    #[test]
    fn propagates_malformed_middleware_attribute_arguments() {
        let message = error_for(
            "#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\n#[middleware(= 5)]\nstruct Bad;\nimpl Bad {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("failed to index"));
    }

    #[test]
    fn reports_responders_present() {
        let index = index_for(
            "#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\nstruct R;\n",
        );

        assert!(has_responders(&index));
    }

    #[test]
    fn reports_no_responders() {
        let index = index_for("#[singleton]\nstruct S;\n");

        assert!(!has_responders(&index));
    }

    #[test]
    fn never_generates_a_route_name_enum_and_omits_names_for_plain_routes() {
        let source = source_for(RESPONDERS_AND_MIDDLEWARE);

        assert!(!source.contains("enumRouteName"));
        assert!(!source.contains("route_with_name"));
        assert!(source.contains("margaret::framework::http::route_entry::RouteEntry::new(\"/open\",::std::vec::Vec::from([margaret::framework::http::method_handler::MethodHandler::new(\"GET\","));
    }

    const NAMED_ROUTE: &str = r#"
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
        assert!(source.contains("margaret::framework::http::route_entry::RouteEntry::new(\"/greeting\",::std::vec::Vec::from([margaret::framework::http::method_handler::MethodHandler::new(\"GET\","));
        assert!(source.contains(
            "margaret::framework::http::named_handler::NamedHandler::new(\"get_greeting\","
        ));
    }

    #[test]
    fn rejects_a_route_name_that_is_not_snake_case() {
        let message = error_for(
            "#[responds_to_http(method = \"get\", name = \"getArticle\", path = \"/a\", server = \"public\")]\nstruct A;\nimpl A {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("must be a snake_case identifier"));
    }

    #[test]
    fn rejects_a_server_name_that_is_not_snake_case() {
        let message = error_for(
            "#[responds_to_http(method = \"get\", path = \"/a\", server = \"PublicApi\")]\nstruct A;\nimpl A {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("must be a snake_case identifier"));
    }

    #[test]
    fn disambiguates_server_names_that_derive_the_same_routes_type() {
        let source = routes_source_for(
            "#[responds_to_http(method = \"get\", path = \"/x\", server = \"a1\")]\nstruct X;\nimpl X {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n\n#[responds_to_http(method = \"get\", path = \"/y\", server = \"a_1\")]\nstruct Y;\nimpl Y {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(source.contains("pubstructA1{"));
        assert!(source.contains("pubstructA12{"));
    }

    #[test]
    fn disambiguates_a_route_named_origin_from_the_internal_origin_field() {
        let source = routes_source_for(
            "#[responds_to_http(method = \"get\", name = \"origin\", path = \"/o\", server = \"public\")]\nstruct O;\nimpl O {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n\n#[responds_to_http(method = \"get\", name = \"get_x\", path = \"/x/{id}\", server = \"public\")]\nstruct GetX;\nimpl GetX {\n    #[process]\n    fn respond(&self, #[route_parameter(from = \"id\")] id: String) -> anyhow::Result<Response> {}\n}\n",
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
            "#[responds_to_http(method = \"get\", name = \"new\", path = \"/n/{id}\", server = \"public\")]\nstruct New;\nimpl New {\n    #[process]\n    fn respond(&self, #[route_parameter(from = \"id\")] id: String) -> anyhow::Result<Response> {}\n}\n",
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
            "#[responds_to_http(method = \"get\", name = \"shared\", path = \"/a\", server = \"public\")]\nstruct A;\nimpl A {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n\n#[responds_to_http(method = \"get\", name = \"shared\", path = \"/b\", server = \"public\")]\nstruct B;\nimpl B {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("both declare the route name"));
    }

    #[test]
    fn propagates_a_non_string_name_argument() {
        let message = error_for(
            "#[responds_to_http(method = \"get\", name = crate::symbols::RouteName::Shared, path = \"/a\", server = \"public\")]\nstruct A;\nimpl A {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("failed to index"));
    }

    #[test]
    fn wraps_the_responder_return_in_a_response_continuation() {
        let source = source_for(
            "#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\nstruct Page;\nimpl Page {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(source.contains("responder.respond().await"));
        assert!(source.contains(
            "margaret::framework::http::response_continuation::ResponseContinuation::from"
        ));
        assert!(
            source.contains("margaret::framework::http::handler_error::HandlerError::consumer")
        );
    }

    const MULTIPLE_SERVERS: &str = r#"
#[responds_to_http(method = "get", path = "/", server = "public")]
struct GetIndex;

impl GetIndex {
    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}

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
            "#[responds_to_http(method = \"get\", path = \"/\")]\nstruct GetIndex;\nimpl GetIndex {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("missing the 'server' argument"));
    }

    #[test]
    fn groups_each_route_under_its_named_server() {
        let source = source_for(MULTIPLE_SERVERS);

        assert!(source.contains(
            "server_public(container:&super::super::container::Container,_routes:&::std::sync::Arc<super::super::routes::Routes>,)->::std::result::Result<margaret::framework::http::server_routes::ServerRoutes,margaret::framework::http::matchit::InsertError,>{margaret::framework::http::router::Router::build(::std::vec::Vec::from([margaret::framework::http::route_entry::RouteEntry::new(\"/\",::std::vec::Vec::from([margaret::framework::http::method_handler::MethodHandler::new(\"GET\","
        ));
        assert!(source.contains(
            "server_internal(container:&super::super::container::Container,_routes:&::std::sync::Arc<super::super::routes::Routes>,)->::std::result::Result<margaret::framework::http::server_routes::ServerRoutes,margaret::framework::http::matchit::InsertError,>{margaret::framework::http::router::Router::build(::std::vec::Vec::from([margaret::framework::http::route_entry::RouteEntry::new(\"/metrics\",::std::vec::Vec::from([margaret::framework::http::method_handler::MethodHandler::new(\"GET\","
        ));
    }

    #[test]
    fn propagates_a_non_string_server_argument() {
        let message = error_for(
            "#[responds_to_http(method = \"get\", path = \"/\", server = ServerMarker)]\nstruct Page;\nimpl Page {\n    #[process]\n    fn respond(&self) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("failed to index"));
    }

    #[test]
    fn injects_a_form_request_from_the_form_source() {
        let source = source_for(
            "use margaret::framework::validation::validation_result::ValidationResult;\n\n#[responds_to_http(method = \"post\", path = \"/data\", server = \"public\")]\nstruct PostData;\nimpl PostData {\n    #[process]\n    fn respond(&self, #[form_request(from = Form)] data: ValidationResult<Data>) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(source.contains(
            "margaret::framework::http_validation::validate_input::validate_input(request,"
        ));
        assert!(
            source.contains(
                "margaret::framework::http_validation::request_input::RequestInput::Form"
            )
        );
        assert!(source.contains("responder.respond(data).await"));
    }

    #[test]
    fn injects_a_form_request_from_the_query_source() {
        let source = source_for(
            "use margaret::framework::validation::validation_result::ValidationResult;\n\n#[responds_to_http(method = \"get\", path = \"/data\", server = \"public\")]\nstruct GetData;\nimpl GetData {\n    #[process]\n    fn respond(&self, #[form_request(from = Query)] data: ValidationResult<Data>) -> anyhow::Result<Response> {}\n}\n",
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
            "use margaret::framework::validation::validation_result::ValidationResult;\n\n#[responds_to_http(method = \"post\", path = \"/data\", server = \"public\")]\nstruct ImportData;\nimpl ImportData {\n    #[process]\n    fn respond(&self, #[form_request(from = Json)] data: ValidationResult<Data>) -> anyhow::Result<Response> {}\n}\n",
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
            "use margaret::framework::validation::validation_result::ValidationResult;\n\n#[responds_to_http(method = \"get\", path = \"/data\", server = \"public\")]\nstruct GetData;\nimpl GetData {\n    #[process]\n    fn respond(&self, #[form_request(from = Cookie)] data: ValidationResult<Data>) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(
            source.contains(
                "margaret::framework::http_validation::request_input::RequestInput::Cookie"
            )
        );
    }

    #[test]
    fn rejects_a_form_request_without_a_source() {
        let message = error_for(
            "#[responds_to_http(method = \"post\", path = \"/data\", server = \"public\")]\nstruct PostData;\nimpl PostData {\n    #[process]\n    fn respond(&self, #[form_request] data: ValidationResult<Data>) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("must name the request input source it validates"));
    }

    #[test]
    fn rejects_a_form_request_with_an_unknown_source() {
        let message = error_for(
            "#[responds_to_http(method = \"post\", path = \"/data\", server = \"public\")]\nstruct PostData;\nimpl PostData {\n    #[process]\n    fn respond(&self, #[form_request(from = Headers)] data: ValidationResult<Data>) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("unknown request input source 'Headers'"));
    }

    #[test]
    fn rejects_an_argument_with_conflicting_markers() {
        let message = error_for(
            "#[responds_to_http(method = \"post\", path = \"/data\", server = \"public\")]\nstruct PostData;\nimpl PostData {\n    #[process]\n    fn respond(&self, #[route_parameter(from = \"x\")] #[form_request(from = Form)] data: ValidationResult<Data>) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("both #[route_parameter] and #[form_request]"));
    }

    #[test]
    fn rejects_a_non_path_form_request_source() {
        let message = error_for(
            "#[responds_to_http(method = \"post\", path = \"/data\", server = \"public\")]\nstruct PostData;\nimpl PostData {\n    #[process]\n    fn respond(&self, #[form_request(from = 5)] data: ValidationResult<Data>) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("failed to read a binding attribute"));
    }

    #[test]
    fn propagates_malformed_form_request_arguments() {
        let message = error_for(
            "#[responds_to_http(method = \"post\", path = \"/data\", server = \"public\")]\nstruct PostData;\nimpl PostData {\n    #[process]\n    fn respond(&self, #[form_request(= 5)] data: ValidationResult<Data>) -> anyhow::Result<Response> {}\n}\n",
        );

        assert!(message.contains("failed to read a binding attribute"));
    }

    #[test]
    fn injects_a_guarded_form_request_as_a_bare_model() {
        let source = source_for(
            "#[responds_to_http(method = \"post\", path = \"/data\", server = \"public\")]\nstruct PostData;\nimpl PostData {\n    #[process]\n    fn respond(&self, #[form_request(from = Form)] data: Data) -> anyhow::Result<Response> {}\n}\n",
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
        assert!(source.contains("responder.respond(data).await"));
    }

    #[test]
    fn weaves_views_into_a_views_injecting_middleware_onion() {
        let source = source_for_with_views(
            r#"
use margaret::framework::http::next::Next;

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
            "std::sync::Arc::new(super::super::middleware::Tracer{inner:container.tracer().await?,views:views.clone()"
        ));
        assert!(source.contains("views:&::std::sync::Arc<super::super::views::Views>"));
    }

    #[test]
    fn rejects_middleware_view_injection_without_declared_views() {
        let message = error_for(
            r#"
use margaret::framework::http::next::Next;

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
        let index = index_for(
            r#"
use margaret::framework::http::next::Next;
use spiffe::spiffe_id::SpiffeId;

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
        let message = error_for(
            r#"
use margaret::framework::http::next::Next;

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
    fn weaves_a_console_argument_into_a_responder_and_its_server() {
        let source = source_for(CONSOLE_ARGUMENT_RESPONDER);

        assert!(source.contains(
            "pubasyncfnserver_public(container:&super::super::container::Container,console_argument_0:&str,"
        ));
        assert!(source.contains("container.greeting(console_argument_0.to_owned())"));
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

#[responds_to_http(method = "get", path = "/users/{user}", server = "public")]
struct GetUser;

impl GetUser {
    #[process]
    fn respond(&self, #[route_parameter(from = "user")] user: User) -> anyhow::Result<Response> {}
}
"#;

    #[test]
    fn weaves_a_console_argument_into_a_route_parameter_binder() {
        let source = source_for(CONSOLE_ARGUMENT_BINDER);

        assert!(source.contains("console_argument_0:&str,"));
        assert!(source.contains("container.user_binder(console_argument_0.to_owned())"));
    }

    const CONSOLE_ARGUMENT_MIDDLEWARE: &str = r#"
use margaret::framework::http::next::Next;
use margaret::framework::http::request::Request;

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
    fn weaves_a_console_argument_into_a_middleware_and_its_server() {
        let source = source_for(CONSOLE_ARGUMENT_MIDDLEWARE);

        assert!(source.contains(
            "pubasyncfnserver_public(container:&super::super::container::Container,console_argument_0:&str,"
        ));
        assert!(source.contains("container.guard(console_argument_0.to_owned())"));
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
    fn expects_too_many_arguments_and_weaves_each_category_by_its_type() {
        let source = source_for(CONSOLE_ARGUMENT_MIXED_CATEGORIES);

        assert!(source.contains("#[expect(clippy::too_many_arguments"));
        assert!(source.contains(":&str,"));
        assert!(source.contains(":&::std::path::Path,"));
        assert!(source.contains(":&bool,"));
        assert!(source.contains(":&::std::option::Option<u16>,"));
        assert!(source.contains(":&::std::option::Option<std::string::String>,"));
        assert!(source.contains(
            "container.configured(console_argument_0.to_owned(),console_argument_1.to_owned(),console_argument_2.to_owned(),*console_argument_3,*console_argument_4,console_argument_5.clone(),)"
        ));
    }
}
