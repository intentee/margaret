mod active_servers;
mod build_registry;
mod form_request_arguments;
mod form_request_extraction;
pub mod generate_http_source;
pub mod has_responders;
pub mod http_artifacts;
pub mod http_codegen_error;
mod http_injectable;
mod http_responder_arguments;
mod http_route;
mod http_route_table;
mod http_routes;
pub mod http_server;
mod layer_application;
mod middleware_argument;
mod middleware_attribute_arguments;
mod middleware_plan;
mod middleware_plans;
mod named_route;
mod render;
mod render_forwarders;
pub mod render_http;
mod render_routes;
mod request_input_source;
mod responder_argument;
mod responder_argument_binding;
mod responder_method;
mod responder_selectors;
mod route_group;
mod route_parameter_arguments;
mod route_path;
mod route_url_template;
pub mod server_cookie_policy;
mod server_route_group;
pub mod server_transport_policy;
pub mod serves_spiffe;
mod url_segment;

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;
    use tempfile::tempdir;

    use margaret_attributes::attribute_index::AttributeIndex;
    use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
    use margaret_attributes::crate_root::CrateRoot;

    use crate::generate_http_source::generate_http_source;
    use crate::has_responders::has_responders;

    const RESPONDERS_AND_MIDDLEWARE: &str = r#"
use margaret_http::next::Next;
use margaret_http::request::Request;

#[responds_to_http(method = "get", path = "/resource", server = "public")]
#[middleware(traced)]
#[middleware(guard)]
struct Resource;

impl Resource {
    #[process]
    fn respond(&self) -> Response {}
}

#[responds_to_http(method = "get", path = "/open", server = "public")]
struct Open;

impl Open {
    #[process]
    fn respond(&self) -> Response {}
}

#[handles_middleware_attribute(attribute = guard)]
struct Guard;

impl Guard {
    #[process]
    fn process(&self, request: &Request, next: Next) -> ResponseContinuation {}
}

#[handles_middleware_attribute(attribute = traced)]
struct Tracer;

impl Tracer {
    #[process]
    fn process(&self, request: &Request, next: Next) -> ResponseContinuation {}
}
"#;

    const ROUTE_PARAMETER: &str = r#"
#[responds_to_http(method = "get", path = "/users/{id}", server = "public")]
struct GetUser;

impl GetUser {
    #[process]
    fn respond(&self, #[route_parameter(from = "id")] id: String) -> Response {}
}
"#;

    const BOUND_MODEL: &str = r#"
struct User;

#[provides_route_parameter]
struct UserBinder;

impl HttpRouteParameterBinder for UserBinder {
    type Model = User;
    async fn bind(&self, value: String) -> Option<User> {}
}

#[responds_to_http(method = "get", path = "/users/{user}", server = "public")]
struct GetUser;

impl GetUser {
    #[process]
    fn respond(&self, #[route_parameter(from = "user")] user: User) -> Response {}
}
"#;

    const DESTRUCTURED_ROUTE_PARAMETER: &str = r#"
struct User;

#[provides_route_parameter]
struct UserBinder;

impl HttpRouteParameterBinder for UserBinder {
    type Model = User;
    async fn bind(&self, value: String) -> Option<User> {}
}

#[responds_to_http(method = "get", path = "/users/{id}", server = "public")]
struct GetUser;

impl GetUser {
    #[process]
    fn respond(&self, #[route_parameter(from = "id")] User { name }: User) -> Response {}
}
"#;

    const CURRENT_REQUEST: &str = r#"
use margaret_http::request::Request;

#[responds_to_http(method = "get", path = "/echo/{id}", server = "public")]
struct Echo;

impl Echo {
    #[process]
    fn respond(&self, request: &Request, #[route_parameter(from = "id")] id: String) -> Response {}
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

    fn error_for(lib_source: &str) -> String {
        let directory = crate_with(lib_source);

        generate_http_source("crate", &directory.path().join("src"))
            .expect_err("the http source fails to generate")
            .to_string()
    }

    fn routes_source_for(lib_source: &str) -> String {
        let directory = crate_with(lib_source);
        let index = AttributeIndexBuilder::new()
            .index_crate(&CrateRoot::new("crate", directory.path().join("src")))
            .expect("the crate is indexed")
            .build();

        crate::render_http::render_http(&index)
            .expect("the http source is generated")
            .into_modules()
            .into_iter()
            .filter(|module| module.name() == "routes" || module.name().starts_with("routes/"))
            .map(|module| module.format().source().to_string())
            .collect::<Vec<String>>()
            .join("\n")
            .split_whitespace()
            .collect()
    }

    const ROUTES_FIXTURE: &str = r#"
#[responds_to_http(method = "get", name = "get_greeting", path = "/greeting", server = "public")]
struct GetGreeting;
impl GetGreeting { #[process] fn respond(&self) -> Response {} }

#[responds_to_http(method = "get", name = "get_article", path = "/articles/{article}", server = "public")]
struct GetArticle;
impl GetArticle { #[process] fn respond(&self, #[route_parameter(from = "article")] article: String) -> Response {} }

#[responds_to_http(method = "post", name = "post_ping", path = "/ping", server = "public")]
struct PostPing;
impl PostPing { #[process] fn respond(&self) -> Response {} }

#[responds_to_http(method = "patch", name = "patch_article", path = "/articles/{article}", server = "public")]
struct PatchArticle;
impl PatchArticle { #[process] fn respond(&self, #[route_parameter(from = "article")] article: String) -> Response {} }

#[responds_to_http(method = "get", path = "/health", server = "internal")]
struct GetHealth;
impl GetHealth { #[process] fn respond(&self) -> Response {} }
"#;

    #[test]
    fn generates_a_forwardable_route_field_for_a_named_paramless_get() {
        let source = routes_source_for(ROUTES_FIXTURE);

        assert!(
            source.contains("pubget_greeting:margaret_http::forwardable_route::ForwardableRoute,")
        );
        assert!(source.contains(
            "get_greeting:margaret_http::forwardable_route::ForwardableRoute::new(origin.clone(),&[margaret_http::url_segment::UrlSegment::Literal(\"/greeting\")],::std::vec::Vec::new(),)"
        ));
    }

    #[test]
    fn generates_a_positional_forwardable_method_for_a_parameterized_get() {
        let source = routes_source_for(ROUTES_FIXTURE);

        assert!(source.contains(
            "pubfnget_article(&self,article:String,)->margaret_http::forwardable_route::ForwardableRoute"
        ));
        assert!(source.contains(
            "margaret_http::forwardable_route::ForwardableRoute::new(self.origin.clone(),&[margaret_http::url_segment::UrlSegment::Literal(\"/articles/\"),margaret_http::url_segment::UrlSegment::Parameter(\"article\"),],::std::vec::Vec::from([article]),)"
        ));
        assert!(!source.contains("Params"));
    }

    #[test]
    fn renders_a_named_non_get_route_as_a_plain_route_reference() {
        let source = routes_source_for(ROUTES_FIXTURE);

        assert!(source.contains("pubpost_ping:margaret_http::route_reference::RouteReference,"));
        assert!(source.contains(
            "pubfnpatch_article(&self,article:String,)->margaret_http::route_reference::RouteReference"
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
            "use crate::margaret::routes::Routes;\n\n#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\nstruct GetX;\nimpl GetX {\n    #[process]\n    fn respond(&self, routes: &Routes) -> Response {}\n}\n",
        );

        assert!(source.contains("&::std::sync::Arc<super::super::routes::Routes>"));
        assert!(source.contains("letroutes=routes.clone();"));
        assert!(source.contains("responder.respond(routes.as_ref()).await"));
    }

    #[test]
    fn injects_the_peer_spiffe_id_into_a_responder_by_type() {
        let source = source_for(
            "use spiffe::spiffe_id::SpiffeId;\n\n#[responds_to_http(method = \"get\", path = \"/x\", server = \"internal\")]\nstruct GetX;\nimpl GetX {\n    #[process]\n    fn respond(&self, peer: &SpiffeId) -> Response {}\n}\n",
        );

        assert!(source.contains("margaret_http::require_peer_spiffe_id::require_peer_spiffe_id("));
    }

    #[test]
    fn injects_the_cookie_jar_into_a_responder_by_type() {
        let source = source_for(
            "use margaret_cookie_jar::cookie_jar::CookieJar;\n\n#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\nstruct GetX;\nimpl GetX {\n    #[process]\n    fn respond(&self, cookies: &CookieJar) -> Response {}\n}\n",
        );

        assert!(source.contains("margaret_http::require_cookie_jar::require_cookie_jar("));
        assert!(source.contains("responder.respond(cookies).await"));
    }

    #[test]
    fn never_mentions_the_cookie_jar_when_the_responder_does_not_ask_for_it() {
        let source = source_for(
            "#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\nstruct GetX;\nimpl GetX {\n    #[process]\n    fn respond(&self) -> Response {}\n}\n",
        );

        assert!(!source.contains("require_cookie_jar"));
    }

    #[test]
    fn injects_the_cookie_jar_into_a_middleware_process_method() {
        let source = source_for(
            "use margaret_cookie_jar::cookie_jar::CookieJar;\nuse margaret_http::next::Next;\n\n#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\n#[middleware(traced)]\nstruct Page;\nimpl Page {\n    #[process]\n    fn respond(&self) -> Response {}\n}\n\n#[handles_middleware_attribute(attribute = traced)]\nstruct Tracer;\nimpl Tracer {\n    #[process]\n    fn process(&self, cookies: &CookieJar, next: Next) -> ResponseContinuation {}\n}\n",
        );

        assert!(source.contains("margaret_http::require_cookie_jar::require_cookie_jar("));
        assert!(source.contains("self.inner.process(cookie_jar,next).await"));
    }

    #[test]
    fn disambiguates_a_cookie_jar_parameter_named_after_the_request_binding() {
        let source = source_for(
            "use margaret_cookie_jar::cookie_jar::CookieJar;\nuse margaret_http::request::Request;\n\n#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\nstruct GetX;\nimpl GetX {\n    #[process]\n    fn respond(&self, request: &CookieJar, incoming: &Request) -> Response {}\n}\n",
        );

        assert!(source.contains("request_2:&margaret_http::request::Request"));
        assert!(source.contains("margaret_http::require_cookie_jar::require_cookie_jar("));
        assert!(source.contains("letincoming=request_2;"));
        assert!(source.contains("responder.respond(request,incoming).await"));
    }

    #[test]
    fn binds_the_cookie_jar_alongside_the_request_and_a_route_parameter() {
        let source = source_for(
            "use margaret_cookie_jar::cookie_jar::CookieJar;\nuse margaret_http::request::Request;\n\n#[responds_to_http(method = \"get\", path = \"/x/{id}\", server = \"public\")]\nstruct GetX;\nimpl GetX {\n    #[process]\n    fn respond(&self, request: &Request, cookies: &CookieJar, #[route_parameter(from = \"id\")] id: String) -> Response {}\n}\n",
        );

        assert!(source.contains("responder.respond(request,cookies,id).await"));
    }

    #[test]
    fn disambiguates_a_cookie_jar_parameter_named_after_the_routes_binding() {
        let source = source_for(
            "use crate::margaret::routes::Routes;\nuse margaret_cookie_jar::cookie_jar::CookieJar;\n\n#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\nstruct GetX;\nimpl GetX {\n    #[process]\n    fn respond(&self, routes: &CookieJar, site: &Routes) -> Response {}\n}\n",
        );

        assert!(source.contains("letroutes_2=routes.clone();"));
        assert!(source.contains("responder.respond(routes,routes_2.as_ref()).await"));
    }

    #[test]
    fn accepts_a_middleware_next_handler_written_with_an_anonymous_lifetime() {
        let source = source_for(
            "use margaret_http::next::Next;\n\n#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\n#[middleware(traced)]\nstruct Page;\nimpl Page {\n    #[process]\n    fn respond(&self) -> Response {}\n}\n\n#[handles_middleware_attribute(attribute = traced)]\nstruct Tracer;\nimpl Tracer {\n    #[process]\n    fn process(&self, next: Next<'_>) -> ResponseContinuation {}\n}\n",
        );

        assert!(source.contains("self.inner.process(next).await"));
    }

    #[test]
    fn rejects_a_middleware_with_multiple_cookie_jar_parameters() {
        let message = error_for(
            "use margaret_cookie_jar::cookie_jar::CookieJar;\nuse margaret_http::next::Next;\n\n#[handles_middleware_attribute(attribute = traced)]\nstruct Tracer;\nimpl Tracer {\n    #[process]\n    fn process(&self, first: &CookieJar, second: &CookieJar, next: Next) -> ResponseContinuation {}\n}\n",
        );

        assert!(message.contains("more than one cookie jar"));
    }

    #[test]
    fn rejects_a_user_type_named_cookie_jar_that_shadows_the_injectable() {
        let message = error_for(
            "mod app {\n    pub struct CookieJar;\n}\n\nuse crate::app::CookieJar;\n\n#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\nstruct GetX;\nimpl GetX {\n    #[process]\n    fn respond(&self, cookies: &CookieJar) -> Response {}\n}\n",
        );

        assert!(message.contains("must be a route parameter"));
    }

    #[test]
    fn rejects_a_cookie_jar_parameter_that_also_carries_a_marker() {
        let message = error_for(
            "use margaret_cookie_jar::cookie_jar::CookieJar;\n\n#[responds_to_http(method = \"get\", path = \"/x/{id}\", server = \"public\")]\nstruct GetX;\nimpl GetX {\n    #[process]\n    fn respond(&self, #[route_parameter(from = \"id\")] cookies: &CookieJar) -> Response {}\n}\n",
        );

        assert!(message.contains("cookie jar and must not"));
    }

    #[test]
    fn rejects_a_responder_with_multiple_cookie_jar_parameters() {
        let message = error_for(
            "use margaret_cookie_jar::cookie_jar::CookieJar;\n\n#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\nstruct GetX;\nimpl GetX {\n    #[process]\n    fn respond(&self, first: &CookieJar, second: &CookieJar) -> Response {}\n}\n",
        );

        assert!(message.contains("more than one cookie jar"));
    }

    #[test]
    fn injects_the_scoped_forwarder_into_a_responder_by_type() {
        let source = source_for(
            "use crate::margaret::forwarders::public::Forwarder;\n\n#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\nstruct GetX;\nimpl GetX {\n    #[process]\n    fn respond(&self, forward: Forwarder) -> Forward {}\n}\n",
        );

        assert!(source.contains("responder.respond(super::super::forwarders::public::Forwarder)"));
    }

    #[test]
    fn rejects_a_user_type_named_routes_that_shadows_the_injectable() {
        let message = error_for(
            "mod app {\n    pub struct Routes;\n}\n\nuse crate::app::Routes;\n\n#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\nstruct GetX;\nimpl GetX {\n    #[process]\n    fn respond(&self, routes: &Routes) -> Response {}\n}\n",
        );

        assert!(message.contains("must be a route parameter"));
    }

    #[test]
    fn rejects_a_forwarder_imported_from_a_foreign_server() {
        let message = error_for(
            "use crate::margaret::forwarders::public::Forwarder;\n\n#[responds_to_http(method = \"get\", path = \"/x\", server = \"internal\")]\nstruct GetX;\nimpl GetX {\n    #[process]\n    fn respond(&self, forward: Forwarder) -> Response {}\n}\n",
        );

        assert!(message.contains("must be a route parameter"));
    }

    #[test]
    fn rejects_a_peer_spiffe_id_parameter_that_also_carries_a_marker() {
        let message = error_for(
            "use spiffe::spiffe_id::SpiffeId;\n\n#[responds_to_http(method = \"get\", path = \"/x/{id}\", server = \"internal\")]\nstruct GetX;\nimpl GetX {\n    #[process]\n    fn respond(&self, #[route_parameter(from = \"id\")] peer: &SpiffeId) -> Response {}\n}\n",
        );

        assert!(message.contains("peer SPIFFE id and must not"));
    }

    #[test]
    fn rejects_a_responder_with_multiple_peer_spiffe_id_parameters() {
        let message = error_for(
            "use spiffe::spiffe_id::SpiffeId;\n\n#[responds_to_http(method = \"get\", path = \"/x\", server = \"internal\")]\nstruct GetX;\nimpl GetX {\n    #[process]\n    fn respond(&self, first: &SpiffeId, second: &SpiffeId) -> Response {}\n}\n",
        );

        assert!(message.contains("more than one peer SPIFFE id"));
    }

    #[test]
    fn rejects_a_route_name_that_is_not_an_identifier() {
        let message = error_for(
            "#[responds_to_http(method = \"get\", name = \"not an identifier\", path = \"/x\", server = \"public\")]\nstruct Bad;\nimpl Bad {\n    #[process]\n    fn respond(&self) -> Response {}\n}\n",
        );

        assert!(message.contains("must be a snake_case identifier"));
    }

    #[test]
    fn rejects_route_paths_that_conflict_on_the_same_server() {
        let message = error_for(
            "#[responds_to_http(method = \"get\", path = \"/articles/{article}\", server = \"public\")]\nstruct First;\nimpl First {\n    #[process]\n    fn respond(&self) -> Response {}\n}\n\n#[responds_to_http(method = \"get\", path = \"/articles/{id}\", server = \"public\")]\nstruct Second;\nimpl Second {\n    #[process]\n    fn respond(&self) -> Response {}\n}\n",
        );

        assert!(message.contains("conflicts with"));
    }

    #[test]
    fn rejects_two_responders_registering_the_same_method_and_path() {
        let message = error_for(
            "#[responds_to_http(method = \"get\", path = \"/articles\", server = \"public\")]\nstruct First;\nimpl First {\n    #[process]\n    fn respond(&self) -> Response {}\n}\n\n#[responds_to_http(method = \"get\", path = \"/articles\", server = \"public\")]\nstruct Second;\nimpl Second {\n    #[process]\n    fn respond(&self) -> Response {}\n}\n",
        );

        assert!(message.contains("already registered"));
    }

    #[test]
    fn accepts_the_same_route_path_with_different_methods() {
        let source = source_for(
            "#[responds_to_http(method = \"get\", path = \"/articles/{article}\", server = \"public\")]\nstruct Read;\nimpl Read {\n    #[process]\n    fn respond(&self) -> Response {}\n}\n\n#[responds_to_http(method = \"delete\", path = \"/articles/{article}\", server = \"public\")]\nstruct Remove;\nimpl Remove {\n    #[process]\n    fn respond(&self) -> Response {}\n}\n",
        );

        assert!(source.contains("\"GET\""));
        assert!(source.contains("\"DELETE\""));
    }

    #[test]
    fn accepts_the_same_route_path_on_different_servers() {
        let source = source_for(
            "#[responds_to_http(method = \"get\", path = \"/articles/{article}\", server = \"public\")]\nstruct First;\nimpl First {\n    #[process]\n    fn respond(&self) -> Response {}\n}\n\n#[responds_to_http(method = \"get\", path = \"/articles/{id}\", server = \"internal\")]\nstruct Second;\nimpl Second {\n    #[process]\n    fn respond(&self) -> Response {}\n}\n",
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
            "margaret_http::responder_handler::responder_handler(container.open().await"
        ));
        assert!(source.contains(
            "|responder:std::sync::Arc<crate::Open>,_request:&margaret_http::request::Request"
        ));
        assert!(source.contains("responder.respond().await"));
        assert!(source.contains(
            "margaret_http::layer::layer(std::sync::Arc::new(super::Guard{inner:container.guard().await,}),margaret_http::responder_handler::responder_handler(container.resource().await"
        ));
        assert!(source.contains(
            "margaret_http::layer::layer(std::sync::Arc::new(super::Tracer{inner:container.tracer().await,}),margaret_http::layer::layer(std::sync::Arc::new(super::Guard{"
        ));

        let guard = source.find("container.guard").expect("the guard is wired");
        let tracer = source
            .find("container.tracer")
            .expect("the tracer is wired");

        assert!(tracer < guard);
    }

    #[test]
    fn generates_a_middleware_wrapper_that_forwards_to_the_process_method() {
        let source = source_for(RESPONDERS_AND_MIDDLEWARE);

        assert!(source.contains("structGuard{inner:std::sync::Arc<crate::Guard>,}"));
        assert!(source.contains("implmargaret_http::http_middleware::HttpMiddlewareforGuard"));
        assert!(source.contains("self.inner.process(request,next).await"));
    }

    #[test]
    fn injects_the_routes_reference_into_a_middleware_process_method() {
        let source = source_for(
            "use crate::margaret::routes::Routes;\nuse margaret_http::next::Next;\nuse margaret_http::request::Request;\n\n#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\n#[middleware(traced)]\nstruct Page;\nimpl Page {\n    #[process]\n    fn respond(&self) -> Response {}\n}\n\n#[handles_middleware_attribute(attribute = traced)]\nstruct Tracer;\nimpl Tracer {\n    #[process]\n    fn process(&self, request: &Request, next: Next, routes: &Routes) -> ResponseContinuation {}\n}\n",
        );

        assert!(source.contains(
            "structTracer{inner:std::sync::Arc<crate::Tracer>,routes:std::sync::Arc<super::routes::Routes>,}"
        ));
        assert!(source.contains("self.inner.process(request,next,&self.routes).await"));
        assert!(source.contains(
            "std::sync::Arc::new(super::Tracer{inner:container.tracer().await,routes:routes.clone()"
        ));
    }

    #[test]
    fn injects_route_parameters_into_the_responder() {
        let source = source_for(ROUTE_PARAMETER);

        assert!(source.contains("\"/users/{id}\""));
        assert!(source.contains("container.get_user().await"));
        assert!(source.contains(
            "|responder:std::sync::Arc<crate::GetUser>,request:&margaret_http::request::Request"
        ));
        assert!(source.contains(
            r#"letid=matchmargaret_http::require_route_parameter::require_route_parameter(request,"id""#
        ));
        assert!(source.contains("Ok(value)=>value,Err(response)=>returnresponse.into()"));
        assert!(source.contains("responder.respond(id).await"));
    }

    #[test]
    fn binds_the_current_request_alongside_a_route_parameter() {
        let source = source_for(CURRENT_REQUEST);

        assert!(source.contains(
            "|responder:std::sync::Arc<crate::Echo>,request:&margaret_http::request::Request"
        ));
        assert!(source.contains(
            r#"letid=matchmargaret_http::require_route_parameter::require_route_parameter(request,"id""#
        ));
        assert!(source.contains("responder.respond(request,id).await"));
    }

    #[test]
    fn binds_a_current_request_parameter_under_a_custom_name() {
        let source = source_for(
            "use margaret_http::request::Request;\n\n#[responds_to_http(method = \"get\", path = \"/echo\", server = \"public\")]\nstruct Echo;\n\nimpl Echo {\n    #[process]\n    fn respond(&self, incoming: &Request) -> Response {}\n}\n",
        );

        assert!(source.contains("letincoming=request;"));
        assert!(source.contains("responder.respond(incoming).await"));
    }

    #[test]
    fn disambiguates_a_route_parameter_named_request_from_the_request_binding() {
        let source = source_for(
            "#[responds_to_http(method = \"get\", path = \"/x/{request}\", server = \"public\")]\nstruct GetX;\nimpl GetX {\n    #[process]\n    fn respond(&self, #[route_parameter(from = \"request\")] request: String) -> Response {}\n}\n",
        );

        assert!(source.contains("request_2:&margaret_http::request::Request"));
        assert!(source.contains(
            r#"letrequest=matchmargaret_http::require_route_parameter::require_route_parameter(request_2,"request""#
        ));
        assert!(source.contains("responder.respond(request).await"));
    }

    #[test]
    fn disambiguates_a_route_parameter_named_responder_from_the_responder_binding() {
        let source = source_for(
            "#[responds_to_http(method = \"get\", path = \"/x/{responder}\", server = \"public\")]\nstruct GetX;\nimpl GetX {\n    #[process]\n    fn respond(&self, #[route_parameter(from = \"responder\")] responder: String) -> Response {}\n}\n",
        );

        assert!(source.contains("|responder_2:std::sync::Arc<crate::GetX>"));
        assert!(source.contains(
            r#"letresponder=matchmargaret_http::require_route_parameter::require_route_parameter(request,"responder""#
        ));
        assert!(source.contains("responder_2.respond(responder).await"));
    }

    #[test]
    fn binds_a_destructured_route_parameter_named_by_from() {
        let source = source_for(DESTRUCTURED_ROUTE_PARAMETER);

        assert!(source.contains(
            r#"margaret_http::require_bound_route_parameter::require_bound_route_parameter(request,"id",user_binder.as_ref()"#
        ));
        assert!(source.contains("responder.respond(argument_1).await"));
    }

    #[test]
    fn injects_a_bound_model() {
        let source = source_for(BOUND_MODEL);

        assert!(source.contains("container.user_binder().await"));
        assert!(source.contains(
            r#"margaret_http::require_bound_route_parameter::require_bound_route_parameter(request,"user",user_binder.as_ref()"#
        ));
        assert!(!source.contains("http_route_parameter_binder::HttpRouteParameterBinder"));
        assert!(source.contains("responder.respond(user).await"));
        assert!(!source.contains("forbidden"));
    }

    #[test]
    fn reports_a_binder_without_a_model_associated_type() {
        let message = error_for("#[provides_route_parameter]\nstruct Bare;\n");

        assert!(message.contains("type Model"));
    }

    #[test]
    fn reports_a_binder_with_a_non_struct_model() {
        let message = error_for(
            "#[provides_route_parameter]\nstruct UnitBinder;\nimpl HttpRouteParameterBinder for UnitBinder {\n    type Model = ();\n    async fn bind(&self, value: String) -> Option<()> {}\n}\n",
        );

        assert!(message.contains("type Model"));
    }

    #[test]
    fn reports_a_binder_whose_model_resolves_to_a_non_struct() {
        let message = error_for(
            "trait Marker {}\n\n#[provides_route_parameter]\nstruct MarkerBinder;\nimpl HttpRouteParameterBinder for MarkerBinder {\n    type Model = Marker;\n    async fn bind(&self, value: String) -> Option<Marker> {}\n}\n",
        );

        assert!(message.contains("type Model"));
    }

    #[test]
    fn rejects_a_route_parameter_of_an_unknown_type() {
        let message = error_for(
            "#[responds_to_http(method = \"get\", path = \"/things/{thing}\", server = \"public\")]\nstruct GetThing;\nimpl GetThing {\n    #[process]\n    fn respond(&self, #[route_parameter(from = \"thing\")] thing: Unknown) -> Response {}\n}\n",
        );

        assert!(message.contains("no #[provides_route_parameter]"));
    }

    #[test]
    fn propagates_malformed_route_parameter_arguments() {
        let message = error_for(
            "#[responds_to_http(method = \"get\", path = \"/things/{thing}\", server = \"public\")]\nstruct GetThing;\nimpl GetThing {\n    #[process]\n    fn respond(&self, #[route_parameter(= 5)] thing: String) -> Response {}\n}\n",
        );

        assert!(message.contains("failed to index"));
    }

    #[test]
    fn rejects_two_binders_for_the_same_model() {
        let message = error_for(
            "struct User;\n\n#[provides_route_parameter]\nstruct First;\nimpl HttpRouteParameterBinder for First {\n    type Model = User;\n    async fn bind(&self, value: String) -> Option<User> {}\n}\n\n#[provides_route_parameter]\nstruct Second;\nimpl HttpRouteParameterBinder for Second {\n    type Model = User;\n    async fn bind(&self, value: String) -> Option<User> {}\n}\n",
        );

        assert!(message.contains("more than one route parameter binder"));
    }

    #[test]
    fn rejects_a_model_parameter_without_a_binder() {
        let message = error_for(
            "struct User;\n\n#[responds_to_http(method = \"get\", path = \"/users/{user}\", server = \"public\")]\nstruct GetUser;\nimpl GetUser {\n    #[process]\n    fn respond(&self, #[route_parameter(from = \"user\")] user: User) -> Response {}\n}\n",
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
            "#[responds_to_http(method = \"get\", path = \"/x/{id}\", server = \"public\")]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn respond(&self, id: String) -> Response {}\n}\n",
        );

        assert!(message.contains("must be a route parameter"));
    }

    #[test]
    fn rejects_a_non_string_route_parameter_source() {
        let message = error_for(
            "#[responds_to_http(method = \"get\", path = \"/x/{id}\", server = \"public\")]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn respond(&self, #[route_parameter(from = 5)] id: String) -> Response {}\n}\n",
        );

        assert!(message.contains("failed to index"));
    }

    #[test]
    fn rejects_a_route_parameter_without_from() {
        let message = error_for(
            "#[responds_to_http(method = \"get\", path = \"/x/{id}\", server = \"public\")]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn respond(&self, #[route_parameter] id: String) -> Response {}\n}\n",
        );

        assert!(message.contains("must name the path parameter it binds"));
    }

    #[test]
    fn rejects_a_route_parameter_absent_from_the_path() {
        let message = error_for(
            "#[responds_to_http(method = \"get\", path = \"/users/{id}\", server = \"public\")]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn respond(&self, #[route_parameter(from = \"slug\")] slug: String) -> Response {}\n}\n",
        );

        assert!(message.contains("does not appear in the route path"));
    }

    #[test]
    fn rejects_a_malformed_route_path() {
        let message = error_for(
            "#[responds_to_http(method = \"get\", path = \"/users/{id\", server = \"public\")]\nstruct Bad;\n\nimpl Bad {\n    #[process]\n    fn respond(&self) -> Response {}\n}\n",
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
            "#[responds_to_http(method = \"query\", path = \"/search\", server = \"public\")]\nstruct Search;\nimpl Search {\n    #[process]\n    fn respond(&self) -> Response {}\n}\n",
        );

        assert!(source.contains(
            "margaret_http::route_entry::RouteEntry::new(\"/search\",::std::vec::Vec::from([margaret_http::method_handler::MethodHandler::new(\"QUERY\","
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
            "#[handles_middleware_attribute(attribute = guard)]\nstruct Bad;\nimpl Bad {\n    #[process]\n    fn process(&self, flag: bool) -> ResponseContinuation {}\n}\n",
        );

        assert!(message.contains(
            "must be the current request, the cookie jar, the next handler, or the routes"
        ));
    }

    #[test]
    fn rejects_a_middleware_attribute_without_a_tag() {
        let message = error_for(
            "#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\n#[middleware]\nstruct Bad;\nimpl Bad {\n    #[process]\n    fn respond(&self) -> Response {}\n}\n",
        );

        assert!(message.contains("must name exactly one middleware tag"));
    }

    #[test]
    fn rejects_an_unknown_middleware_tag() {
        let message = error_for(
            "#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\n#[middleware(missing)]\nstruct Bad;\nimpl Bad {\n    #[process]\n    fn respond(&self) -> Response {}\n}\n",
        );

        assert!(message.contains("no #[handles_middleware_attribute] handles it"));
    }

    #[test]
    fn propagates_malformed_middleware_attribute_arguments() {
        let message = error_for(
            "#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\n#[middleware(= 5)]\nstruct Bad;\nimpl Bad {\n    #[process]\n    fn respond(&self) -> Response {}\n}\n",
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
        assert!(source.contains("margaret_http::route_entry::RouteEntry::new(\"/open\",::std::vec::Vec::from([margaret_http::method_handler::MethodHandler::new(\"GET\","));
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
    fn respond(&self) -> Response {}
}
"#;

    #[test]
    fn registers_a_route_under_an_explicit_name() {
        let source = source_for(NAMED_ROUTE);

        assert!(!source.contains("enumRouteName"));
        assert!(!source.contains("route_with_name"));
        assert!(source.contains("margaret_http::route_entry::RouteEntry::new(\"/greeting\",::std::vec::Vec::from([margaret_http::method_handler::MethodHandler::new(\"GET\","));
        assert!(
            source.contains("margaret_http::named_handler::NamedHandler::new(\"get_greeting\",")
        );
    }

    #[test]
    fn rejects_a_route_name_that_is_not_snake_case() {
        let message = error_for(
            "#[responds_to_http(method = \"get\", name = \"getArticle\", path = \"/a\", server = \"public\")]\nstruct A;\nimpl A {\n    #[process]\n    fn respond(&self) -> Response {}\n}\n",
        );

        assert!(message.contains("must be a snake_case identifier"));
    }

    #[test]
    fn rejects_a_server_name_that_is_not_snake_case() {
        let message = error_for(
            "#[responds_to_http(method = \"get\", path = \"/a\", server = \"PublicApi\")]\nstruct A;\nimpl A {\n    #[process]\n    fn respond(&self) -> Response {}\n}\n",
        );

        assert!(message.contains("must be a snake_case identifier"));
    }

    #[test]
    fn disambiguates_server_names_that_derive_the_same_routes_type() {
        let source = routes_source_for(
            "#[responds_to_http(method = \"get\", path = \"/x\", server = \"a1\")]\nstruct X;\nimpl X {\n    #[process]\n    fn respond(&self) -> Response {}\n}\n\n#[responds_to_http(method = \"get\", path = \"/y\", server = \"a_1\")]\nstruct Y;\nimpl Y {\n    #[process]\n    fn respond(&self) -> Response {}\n}\n",
        );

        assert!(source.contains("pubstructA1{"));
        assert!(source.contains("pubstructA12{"));
    }

    #[test]
    fn disambiguates_a_route_named_origin_from_the_internal_origin_field() {
        let source = routes_source_for(
            "#[responds_to_http(method = \"get\", name = \"origin\", path = \"/o\", server = \"public\")]\nstruct O;\nimpl O {\n    #[process]\n    fn respond(&self) -> Response {}\n}\n\n#[responds_to_http(method = \"get\", name = \"get_x\", path = \"/x/{id}\", server = \"public\")]\nstruct GetX;\nimpl GetX {\n    #[process]\n    fn respond(&self, #[route_parameter(from = \"id\")] id: String) -> Response {}\n}\n",
        );

        assert!(source.contains("origin_2:::std::sync::Arc<str>,"));
        assert!(source.contains("puborigin:margaret_http::forwardable_route::ForwardableRoute,"));
        assert!(source.contains("self.origin_2.clone()"));
    }

    #[test]
    fn disambiguates_a_route_named_new_from_the_generated_constructor() {
        let source = routes_source_for(
            "#[responds_to_http(method = \"get\", name = \"new\", path = \"/n/{id}\", server = \"public\")]\nstruct New;\nimpl New {\n    #[process]\n    fn respond(&self, #[route_parameter(from = \"id\")] id: String) -> Response {}\n}\n",
        );

        assert!(source.contains("pub(crate)fnnew_2(origin:::std::sync::Arc<str>)"));
        assert!(source.contains(
            "pubfnnew(&self,id:String)->margaret_http::forwardable_route::ForwardableRoute"
        ));
        assert!(source.contains("servers::public::Public::new_2(origin_public)"));
    }

    #[test]
    fn rejects_two_routes_sharing_a_name() {
        let message = error_for(
            "#[responds_to_http(method = \"get\", name = \"shared\", path = \"/a\", server = \"public\")]\nstruct A;\nimpl A {\n    #[process]\n    fn respond(&self) -> Response {}\n}\n\n#[responds_to_http(method = \"get\", name = \"shared\", path = \"/b\", server = \"public\")]\nstruct B;\nimpl B {\n    #[process]\n    fn respond(&self) -> Response {}\n}\n",
        );

        assert!(message.contains("both declare the route name"));
    }

    #[test]
    fn propagates_a_non_string_name_argument() {
        let message = error_for(
            "#[responds_to_http(method = \"get\", name = crate::symbols::RouteName::Shared, path = \"/a\", server = \"public\")]\nstruct A;\nimpl A {\n    #[process]\n    fn respond(&self) -> Response {}\n}\n",
        );

        assert!(message.contains("failed to index"));
    }

    #[test]
    fn omits_the_request_from_a_middleware_that_only_delegates() {
        let source = source_for(
            "use margaret_http::next::Next;\n\n#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\n#[middleware(traced)]\nstruct Page;\nimpl Page {\n    #[process]\n    fn respond(&self) -> Response {}\n}\n\n#[handles_middleware_attribute(attribute = traced)]\nstruct Tracer;\nimpl Tracer {\n    #[process]\n    fn process(&self, next: Next) -> ResponseContinuation {}\n}\n",
        );

        assert!(source.contains(
            "asyncfnprocess(&self,_request:&margaret_http::request::Request,next:margaret_http::next::Next,)"
        ));
        assert!(source.contains("self.inner.process(next).await"));
    }

    #[test]
    fn omits_the_next_handler_from_a_short_circuiting_middleware() {
        let source = source_for(
            "use margaret_http::request::Request;\n\n#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\n#[middleware(guard)]\nstruct Page;\nimpl Page {\n    #[process]\n    fn respond(&self) -> Response {}\n}\n\n#[handles_middleware_attribute(attribute = guard)]\nstruct Guard;\nimpl Guard {\n    #[process]\n    fn process(&self, request: &Request) -> ResponseContinuation {}\n}\n",
        );

        assert!(source.contains(
            "asyncfnprocess(&self,request:&margaret_http::request::Request,_next:margaret_http::next::Next,)"
        ));
        assert!(source.contains("self.inner.process(request).await"));
    }

    #[test]
    fn disambiguates_middlewares_that_derive_the_same_wrapper_name() {
        let source = source_for(
            "use margaret_http::next::Next;\n\n#[handles_middleware_attribute(attribute = one)]\nstruct V2;\nimpl V2 {\n    #[process]\n    fn process(&self, next: Next) -> ResponseContinuation {}\n}\n\n#[handles_middleware_attribute(attribute = two)]\nstruct V_2;\nimpl V_2 {\n    #[process]\n    fn process(&self, next: Next) -> ResponseContinuation {}\n}\n",
        );

        assert!(source.contains("structV2{inner:std::sync::Arc<crate::V2>,}"));
        assert!(source.contains("structV22{inner:std::sync::Arc<crate::V_2>,}"));
    }

    #[test]
    fn wraps_the_responder_return_in_a_response_continuation() {
        let source = source_for(
            "#[responds_to_http(method = \"get\", path = \"/x\", server = \"public\")]\nstruct Page;\nimpl Page {\n    #[process]\n    fn respond(&self) -> Response {}\n}\n",
        );

        assert!(
            source.contains("margaret_http::response_continuation::ResponseContinuation::from(responder.respond().await,)")
        );
    }

    const MULTIPLE_SERVERS: &str = r#"
#[responds_to_http(method = "get", path = "/", server = "public")]
struct GetIndex;

impl GetIndex {
    #[process]
    fn respond(&self) -> Response {}
}

#[responds_to_http(method = "get", path = "/metrics", server = "internal")]
struct GetMetrics;

impl GetMetrics {
    #[process]
    fn respond(&self) -> Response {}
}
"#;

    #[test]
    fn rejects_a_route_without_a_server() {
        let message = error_for(
            "#[responds_to_http(method = \"get\", path = \"/\")]\nstruct GetIndex;\nimpl GetIndex {\n    #[process]\n    fn respond(&self) -> Response {}\n}\n",
        );

        assert!(message.contains("missing the 'server' argument"));
    }

    #[test]
    fn groups_each_route_under_its_named_server() {
        let source = source_for(MULTIPLE_SERVERS);

        assert!(source.contains(
            "server_public(container:&super::super::container::Container,_routes:&::std::sync::Arc<super::super::routes::Routes>,)->::std::result::Result<margaret_http::server_routes::ServerRoutes,margaret_http::matchit::InsertError,>{margaret_http::router::Router::build(::std::vec::Vec::from([margaret_http::route_entry::RouteEntry::new(\"/\",::std::vec::Vec::from([margaret_http::method_handler::MethodHandler::new(\"GET\","
        ));
        assert!(source.contains(
            "server_internal(container:&super::super::container::Container,_routes:&::std::sync::Arc<super::super::routes::Routes>,)->::std::result::Result<margaret_http::server_routes::ServerRoutes,margaret_http::matchit::InsertError,>{margaret_http::router::Router::build(::std::vec::Vec::from([margaret_http::route_entry::RouteEntry::new(\"/metrics\",::std::vec::Vec::from([margaret_http::method_handler::MethodHandler::new(\"GET\","
        ));
    }

    #[test]
    fn propagates_a_non_string_server_argument() {
        let message = error_for(
            "#[responds_to_http(method = \"get\", path = \"/\", server = ServerMarker)]\nstruct Page;\nimpl Page {\n    #[process]\n    fn respond(&self) -> Response {}\n}\n",
        );

        assert!(message.contains("failed to index"));
    }

    #[test]
    fn injects_a_form_request_from_the_form_source() {
        let source = source_for(
            "use margaret_validation::validation_result::ValidationResult;\n\n#[responds_to_http(method = \"post\", path = \"/data\", server = \"public\")]\nstruct PostData;\nimpl PostData {\n    #[process]\n    fn respond(&self, #[form_request(from = Form)] data: ValidationResult<Data>) -> Response {}\n}\n",
        );

        assert!(
            source.contains("margaret_http_validation::validate_input::validate_input(request,")
        );
        assert!(source.contains("margaret_http_validation::request_input::RequestInput::Form"));
        assert!(source.contains("responder.respond(data).await"));
    }

    #[test]
    fn injects_a_form_request_from_the_query_source() {
        let source = source_for(
            "use margaret_validation::validation_result::ValidationResult;\n\n#[responds_to_http(method = \"get\", path = \"/data\", server = \"public\")]\nstruct GetData;\nimpl GetData {\n    #[process]\n    fn respond(&self, #[form_request(from = Query)] data: ValidationResult<Data>) -> Response {}\n}\n",
        );

        assert!(source.contains("margaret_http_validation::request_input::RequestInput::Query"));
    }

    #[test]
    fn injects_a_form_request_from_the_json_source() {
        let source = source_for(
            "use margaret_validation::validation_result::ValidationResult;\n\n#[responds_to_http(method = \"post\", path = \"/data\", server = \"public\")]\nstruct ImportData;\nimpl ImportData {\n    #[process]\n    fn respond(&self, #[form_request(from = Json)] data: ValidationResult<Data>) -> Response {}\n}\n",
        );

        assert!(source.contains("margaret_http_validation::request_input::RequestInput::Json"));
    }

    #[test]
    fn rejects_a_form_request_without_a_source() {
        let message = error_for(
            "#[responds_to_http(method = \"post\", path = \"/data\", server = \"public\")]\nstruct PostData;\nimpl PostData {\n    #[process]\n    fn respond(&self, #[form_request] data: ValidationResult<Data>) -> Response {}\n}\n",
        );

        assert!(message.contains("must name the request input source it validates"));
    }

    #[test]
    fn rejects_a_form_request_with_an_unknown_source() {
        let message = error_for(
            "#[responds_to_http(method = \"post\", path = \"/data\", server = \"public\")]\nstruct PostData;\nimpl PostData {\n    #[process]\n    fn respond(&self, #[form_request(from = Cookies)] data: ValidationResult<Data>) -> Response {}\n}\n",
        );

        assert!(message.contains("unknown request input source 'Cookies'"));
    }

    #[test]
    fn rejects_an_argument_with_conflicting_markers() {
        let message = error_for(
            "#[responds_to_http(method = \"post\", path = \"/data\", server = \"public\")]\nstruct PostData;\nimpl PostData {\n    #[process]\n    fn respond(&self, #[route_parameter(from = \"x\")] #[form_request(from = Form)] data: ValidationResult<Data>) -> Response {}\n}\n",
        );

        assert!(message.contains("both #[route_parameter] and #[form_request]"));
    }

    #[test]
    fn rejects_a_non_path_form_request_source() {
        let message = error_for(
            "#[responds_to_http(method = \"post\", path = \"/data\", server = \"public\")]\nstruct PostData;\nimpl PostData {\n    #[process]\n    fn respond(&self, #[form_request(from = 5)] data: ValidationResult<Data>) -> Response {}\n}\n",
        );

        assert!(message.contains("failed to index"));
    }

    #[test]
    fn propagates_malformed_form_request_arguments() {
        let message = error_for(
            "#[responds_to_http(method = \"post\", path = \"/data\", server = \"public\")]\nstruct PostData;\nimpl PostData {\n    #[process]\n    fn respond(&self, #[form_request(= 5)] data: ValidationResult<Data>) -> Response {}\n}\n",
        );

        assert!(message.contains("failed to index"));
    }

    #[test]
    fn injects_a_guarded_form_request_as_a_bare_model() {
        let source = source_for(
            "#[responds_to_http(method = \"post\", path = \"/data\", server = \"public\")]\nstruct PostData;\nimpl PostData {\n    #[process]\n    fn respond(&self, #[form_request(from = Form)] data: Data) -> Response {}\n}\n",
        );

        assert!(source.contains("margaret_http_validation::require_input::require_input(request,"));
        assert!(source.contains("margaret_http_validation::request_input::RequestInput::Form"));
        assert!(source.contains("Ok(model)=>model"));
        assert!(source.contains("Err(response)=>returnresponse.into()"));
        assert!(source.contains("responder.respond(data).await"));
    }
}
