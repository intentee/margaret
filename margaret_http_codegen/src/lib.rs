mod authorization;
mod build_registry;
pub mod generate_http_source;
pub mod has_responders;
pub mod http_codegen_error;
mod http_route;
mod http_routes;
mod layer_application;
mod middleware_binding;
mod middleware_bindings;
mod path_parameter_names;
mod registries;
mod render;
pub mod render_http;
mod resolve_struct;
mod responder_method;
mod route_parameter;
mod route_parameter_binding;

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
#[responds_to_http(method = Get, path = "/resource")]
#[traced]
#[guard(crate::action::Action::Read)]
struct Resource;

impl Resource {
    #[responder]
    fn respond(&self) -> Response {}
}

#[responds_to_http(method = Get, path = "/open")]
struct Open;

impl Open {
    #[responder]
    fn respond(&self) -> Response {}
}

#[http_middleware(handles = guard)]
struct Guard;

#[http_middleware(handles = traced)]
struct Tracer;
"#;

    const ROUTE_PARAMETER: &str = r#"
#[responds_to_http(method = Get, path = "/users/{id}")]
struct GetUser;

impl GetUser {
    #[responder]
    fn respond(&self, #[route_parameter] id: String) -> Response {}
}
"#;

    const BOUND_WITHOUT_INTENT: &str = r#"
struct User;

#[route_parameter_binder]
struct UserBinder;

impl RouteParameterBinder for UserBinder {
    type Model = User;
    async fn bind(&self, value: String) -> Option<User> {}
}

#[responds_to_http(method = Get, path = "/users/{user}")]
struct GetUser;

impl GetUser {
    #[responder]
    fn respond(&self, #[route_parameter] user: User) -> Response {}
}
"#;

    const MODEL_PARAMETER: &str = r#"
struct User;

#[route_parameter_binder]
struct UserBinder;

impl RouteParameterBinder for UserBinder {
    type Model = User;
    async fn bind(&self, value: String) -> Option<User> {}
}

#[crud_gate]
struct UserGate;

impl CrudActionGate for UserGate {
    type Subject = User;
    async fn can(&self, request: &Request, subject: &User, action: CrudAction) -> bool {}
}

#[responds_to_http(method = Get, path = "/profiles/{user}")]
struct GetProfile;

impl GetProfile {
    #[responder]
    fn respond(&self, #[route_parameter(intent = CrudAction::Read)] user: User) -> Response {}
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

    #[test]
    fn generates_per_route_typed_marker_middleware() {
        let source = source_for(RESPONDERS_AND_MIDDLEWARE);

        assert!(source.contains("pubasyncfnserver"));
        assert!(source.contains("usesuper::container::Container"));
        assert!(source.contains("margaret_http::method::Method::Get"));
        assert!(source.contains(
            "margaret_http::responder_handler::responder_handler(container.open().await"
        ));
        assert!(source.contains(
            "|responder:std::sync::Arc<crate::Open>,_request:margaret_http::request::Request|"
        ));
        assert!(source.contains("responder.respond().await"));
        assert!(source.contains("container.guard().await,crate::action::Action::Read"));
        assert!(source.contains("container.tracer().await,()"));
        assert!(source.contains("container.resource().await"));

        let guard = source.find("container.guard").expect("the guard is wired");
        let tracer = source
            .find("container.tracer")
            .expect("the tracer is wired");

        assert!(tracer < guard);
    }

    #[test]
    fn injects_route_parameters_into_the_responder() {
        let source = source_for(ROUTE_PARAMETER);

        assert!(source.contains("\"/users/{id}\""));
        assert!(source.contains("container.get_user().await"));
        assert!(source.contains(
            "|responder:std::sync::Arc<crate::GetUser>,request:margaret_http::request::Request|"
        ));
        assert!(source.contains(r#"letid=request.path_param("id").expect"#));
        assert!(source.contains(".to_string();"));
        assert!(source.contains("responder.respond(id).await"));
    }

    #[test]
    fn injects_a_bound_model_with_an_authorization_gate() {
        let source = source_for(MODEL_PARAMETER);

        assert!(source.contains("usemargaret_http::crud_action::CrudAction;"));
        assert!(source.contains("usemargaret_http::route_parameter_binder::RouteParameterBinder;"));
        assert!(source.contains("usemargaret_http::crud_action_gate::CrudActionGate;"));
        assert!(source.contains("container.user_binder().await"));
        assert!(source.contains("container.user_gate().await"));
        assert!(source.contains(r#"user_binder.bind(request.path_param("user").expect"#));
        assert!(source.contains("margaret_http::response::Response::not_found()"));
        assert!(source.contains("user_gate.can(&request,&user,CrudAction::Read).await"));
        assert!(source.contains("margaret_http::response::Response::forbidden()"));
        assert!(source.contains("responder.respond(user).await"));
    }

    #[test]
    fn injects_a_bound_model_without_authorization() {
        let source = source_for(BOUND_WITHOUT_INTENT);

        assert!(source.contains("usemargaret_http::route_parameter_binder::RouteParameterBinder;"));
        assert!(source.contains("container.user_binder().await"));
        assert!(source.contains(r#"user_binder.bind(request.path_param("user").expect"#));
        assert!(source.contains("margaret_http::response::Response::not_found()"));
        assert!(source.contains("responder.respond(user).await"));
        assert!(!source.contains("CrudAction"));
        assert!(!source.contains("forbidden"));
    }

    #[test]
    fn reports_a_binder_without_a_model_associated_type() {
        let message = error_for("#[route_parameter_binder]\nstruct Bare;\n");

        assert!(message.contains("type Model"));
    }

    #[test]
    fn reports_a_binder_with_a_non_struct_model() {
        let message = error_for(
            "#[route_parameter_binder]\nstruct UnitBinder;\nimpl RouteParameterBinder for UnitBinder {\n    type Model = ();\n    async fn bind(&self, value: String) -> Option<()> {}\n}\n",
        );

        assert!(message.contains("type Model"));
    }

    #[test]
    fn rejects_a_route_parameter_of_an_unknown_type() {
        let message = error_for(
            "#[responds_to_http(method = Get, path = \"/things/{thing}\")]\nstruct GetThing;\nimpl GetThing {\n    #[responder]\n    fn respond(&self, #[route_parameter] thing: Unknown) -> Response {}\n}\n",
        );

        assert!(message.contains("no #[route_parameter_binder]"));
    }

    #[test]
    fn propagates_malformed_route_parameter_arguments() {
        let message = error_for(
            "#[responds_to_http(method = Get, path = \"/things/{thing}\")]\nstruct GetThing;\nimpl GetThing {\n    #[responder]\n    fn respond(&self, #[route_parameter(= 5)] thing: String) -> Response {}\n}\n",
        );

        assert!(message.contains("failed to index"));
    }

    #[test]
    fn rejects_a_non_path_intent() {
        let message = error_for(
            "#[responds_to_http(method = Get, path = \"/things/{thing}\")]\nstruct GetThing;\nimpl GetThing {\n    #[responder]\n    fn respond(&self, #[route_parameter(intent = 5)] thing: String) -> Response {}\n}\n",
        );

        assert!(message.contains("failed to index"));
    }

    #[test]
    fn rejects_two_binders_for_the_same_model() {
        let message = error_for(
            "struct User;\n\n#[route_parameter_binder]\nstruct First;\nimpl RouteParameterBinder for First {\n    type Model = User;\n    async fn bind(&self, value: String) -> Option<User> {}\n}\n\n#[route_parameter_binder]\nstruct Second;\nimpl RouteParameterBinder for Second {\n    type Model = User;\n    async fn bind(&self, value: String) -> Option<User> {}\n}\n",
        );

        assert!(message.contains("more than one route parameter binder"));
    }

    #[test]
    fn reports_a_gate_without_a_subject_associated_type() {
        let message = error_for("#[crud_gate]\nstruct Bare;\n");

        assert!(message.contains("type Subject"));
    }

    #[test]
    fn rejects_two_gates_for_the_same_subject() {
        let message = error_for(
            "struct User;\n\n#[crud_gate]\nstruct First;\nimpl CrudActionGate for First {\n    type Subject = User;\n    async fn can(&self, request: &Request, subject: &User, action: CrudAction) -> bool {}\n}\n\n#[crud_gate]\nstruct Second;\nimpl CrudActionGate for Second {\n    type Subject = User;\n    async fn can(&self, request: &Request, subject: &User, action: CrudAction) -> bool {}\n}\n",
        );

        assert!(message.contains("more than one CRUD gate"));
    }

    #[test]
    fn rejects_a_model_parameter_without_a_binder() {
        let message = error_for(
            "struct User;\n\n#[responds_to_http(method = Get, path = \"/users/{user}\")]\nstruct GetUser;\nimpl GetUser {\n    #[responder]\n    fn respond(&self, #[route_parameter] user: User) -> Response {}\n}\n",
        );

        assert!(message.contains("no #[route_parameter_binder]"));
    }

    #[test]
    fn rejects_an_intent_without_a_gate() {
        let message = error_for(
            "struct User;\n\n#[route_parameter_binder]\nstruct UserBinder;\nimpl RouteParameterBinder for UserBinder {\n    type Model = User;\n    async fn bind(&self, value: String) -> Option<User> {}\n}\n\n#[responds_to_http(method = Get, path = \"/users/{user}\")]\nstruct GetUser;\nimpl GetUser {\n    #[responder]\n    fn respond(&self, #[route_parameter(intent = CrudAction::Read)] user: User) -> Response {}\n}\n",
        );

        assert!(message.contains("no #[crud_gate]"));
    }

    #[test]
    fn rejects_an_intent_on_a_raw_parameter() {
        let message = error_for(
            "#[responds_to_http(method = Get, path = \"/users/{id}\")]\nstruct GetUser;\nimpl GetUser {\n    #[responder]\n    fn respond(&self, #[route_parameter(intent = CrudAction::Read)] id: String) -> Response {}\n}\n",
        );

        assert!(message.contains("intent only applies to model parameters"));
    }

    #[test]
    fn rejects_a_responder_without_a_responder_method() {
        let message = error_for("#[responds_to_http(method = Get, path = \"/x\")]\nstruct Bare;\n");

        assert!(message.contains("no #[responder] method"));
    }

    #[test]
    fn rejects_an_unmarked_responder_parameter() {
        let message = error_for(
            "#[responds_to_http(method = Get, path = \"/x/{id}\")]\nstruct Bad;\n\nimpl Bad {\n    #[responder]\n    fn respond(&self, id: String) -> Response {}\n}\n",
        );

        assert!(message.contains("must be a route parameter"));
    }

    #[test]
    fn rejects_a_non_identifier_route_parameter() {
        let message = error_for(
            "#[responds_to_http(method = Get, path = \"/x/{id}\")]\nstruct Bad;\n\nimpl Bad {\n    #[responder]\n    fn respond(&self, #[route_parameter] (id, extra): (String, String)) -> Response {}\n}\n",
        );

        assert!(message.contains("not a plain identifier"));
    }

    #[test]
    fn rejects_a_route_parameter_absent_from_the_path() {
        let message = error_for(
            "#[responds_to_http(method = Get, path = \"/users/{id}\")]\nstruct Bad;\n\nimpl Bad {\n    #[responder]\n    fn respond(&self, #[route_parameter] slug: String) -> Response {}\n}\n",
        );

        assert!(message.contains("does not appear in the route path"));
    }

    #[test]
    fn rejects_a_malformed_route_path() {
        let message = error_for(
            "#[responds_to_http(method = Get, path = \"/users/{id\")]\nstruct Bad;\n\nimpl Bad {\n    #[responder]\n    fn respond(&self) -> Response {}\n}\n",
        );

        assert!(message.contains("malformed route path"));
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
        let message = error_for("#[http_middleware(handles = x)]\nenum Bad {}\n");

        assert!(message.contains("#[http_middleware]"));
    }

    #[test]
    fn propagates_malformed_middleware_arguments() {
        assert!(error_for("#[http_middleware(= 5)]\nstruct Bad;\n").contains("failed to index"));
    }

    #[test]
    fn rejects_middleware_without_handles() {
        assert!(error_for("#[http_middleware]\nstruct Bad;\n").contains("missing the 'handles'"));
    }

    #[test]
    fn propagates_a_non_path_handles_argument() {
        let message = error_for("#[http_middleware(handles = \"x\")]\nstruct Bad;\n");

        assert!(message.contains("failed to index"));
    }

    #[test]
    fn rejects_a_malformed_marker() {
        let message = error_for(
            "#[responds_to_http(method = Get, path = \"/x\")]\n#[guard(crate::A, crate::B)]\nstruct Bad;\n\n#[http_middleware(handles = guard)]\nstruct Guard;\n",
        );

        assert!(message.contains("must carry zero or one positional argument"));
    }

    #[test]
    fn propagates_malformed_marker_arguments() {
        let message = error_for(
            "#[responds_to_http(method = Get, path = \"/x\")]\n#[guard(= 5)]\nstruct Bad;\n\n#[http_middleware(handles = guard)]\nstruct Guard;\n",
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
