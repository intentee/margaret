mod active_servers;
mod authenticated_actor_store;
mod build_registry;
mod can_arguments;
mod crud_gate_subjects;
pub mod generate_http_source;
pub mod has_responders;
pub mod http_artifacts;
pub mod http_codegen_error;
mod http_responder_arguments;
mod http_route;
mod http_routes;
pub mod http_server;
mod interceptor_bindings;
mod layer_application;
mod marker_arguments;
mod middleware_attribute_arguments;
mod middleware_binding;
mod middleware_bindings;
mod path_parameter_names;
mod registries;
mod render;
pub mod render_http;
mod responder_method;
mod responder_output;
mod responder_signature;
mod route_parameter;
mod route_parameter_arguments;
mod route_parameter_binding;
mod site_action_arguments;
mod site_action_gates;

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
#[responds_to_http(method = Get, path = "/resource", server = "public")]
#[traced]
#[guard(crate::action::Action::Read)]
struct Resource;

impl Resource {
    #[responder]
    fn respond(&self) -> Response {}
}

#[responds_to_http(method = Get, path = "/open", server = "public")]
struct Open;

impl Open {
    #[responder]
    fn respond(&self) -> Response {}
}

#[handles_middleware_attribute(attribute = guard)]
struct Guard;

#[handles_middleware_attribute(attribute = traced)]
struct Tracer;
"#;

    const ROUTE_PARAMETER: &str = r#"
#[responds_to_http(method = Get, path = "/users/{id}", server = "public")]
struct GetUser;

impl GetUser {
    #[responder]
    fn respond(&self, #[route_parameter(from = "id")] id: String) -> Response {}
}
"#;

    const BOUND_WITHOUT_INTENT: &str = r#"
struct User;

#[provides_route_parameter]
struct UserBinder;

impl HttpRouteParameterBinder for UserBinder {
    type Model = User;
    async fn bind(&self, value: String) -> Option<User> {}
}

#[responds_to_http(method = Get, path = "/users/{user}", server = "public")]
struct GetUser;

impl GetUser {
    #[responder]
    fn respond(&self, #[route_parameter(from = "user")] user: User) -> Response {}
}
"#;

    const MODEL_PARAMETER: &str = r#"
struct User;

#[provides_authenticated_actor]
struct SessionStore;

impl AuthenticatedActorStore for SessionStore {
    type Actor = User;
    async fn get_authenticated_actor(&self, request: &Request) -> Option<User> {}
}

#[provides_route_parameter]
struct UserBinder;

impl HttpRouteParameterBinder for UserBinder {
    type Model = User;
    async fn bind(&self, value: String) -> Option<User> {}
}

#[decides_crud_action]
struct UserGate;

impl UserGate {
    #[decides]
    async fn can(&self, authenticated_actor: &AuthenticatedActor<User>, subject: &User, action: CrudAction) -> bool {}
}

#[responds_to_http(method = Get, path = "/profiles/{user}", server = "public")]
struct GetProfile;

impl GetProfile {
    #[responder]
    fn respond(&self, #[route_parameter(from = "user", intent = CrudAction::Read)] user: User) -> Response {}
}
"#;

    const AUTHENTICATED_ACTOR: &str = r#"
struct User;

#[provides_authenticated_actor]
struct SessionStore;

impl AuthenticatedActorStore for SessionStore {
    type Actor = User;
    async fn get_authenticated_actor(&self, request: &Request) -> Option<AuthenticatedActor<User>> {}
}

#[responds_to_http(method = Get, path = "/account", server = "public")]
struct GetAccount;

impl GetAccount {
    #[responder]
    fn respond(&self, user: AuthenticatedActor<User>) -> Response {}
}

#[responds_to_http(method = Get, path = "/maybe", server = "public")]
struct GetMaybe;

impl GetMaybe {
    #[responder]
    fn respond(&self, user: Option<AuthenticatedActor<User>>) -> Response {}
}
"#;

    const DESTRUCTURED_ACTOR: &str = r#"
struct User;

#[provides_authenticated_actor]
struct SessionStore;

impl AuthenticatedActorStore for SessionStore {
    type Actor = User;
    async fn get_authenticated_actor(&self, request: &Request) -> Option<AuthenticatedActor<User>> {}
}

#[responds_to_http(method = Get, path = "/account", server = "public")]
struct GetAccount;

impl GetAccount {
    #[responder]
    fn respond(&self, AuthenticatedActor { actor }: AuthenticatedActor<User>) -> Response {}
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

#[responds_to_http(method = Get, path = "/users/{id}", server = "public")]
struct GetUser;

impl GetUser {
    #[responder]
    fn respond(&self, #[route_parameter(from = "id")] User { name }: User) -> Response {}
}
"#;

    const ROUTE_PARAMETER_FROM_UNKNOWN: &str = r#"
struct User;

#[provides_route_parameter]
struct UserBinder;

impl HttpRouteParameterBinder for UserBinder {
    type Model = User;
    async fn bind(&self, value: String) -> Option<User> {}
}

#[responds_to_http(method = Get, path = "/users/{id}", server = "public")]
struct GetUser;

impl GetUser {
    #[responder]
    fn respond(&self, #[route_parameter(from = "missing")] User { name }: User) -> Response {}
}
"#;

    const SITE_ACTION: &str = r#"
struct User;

#[provides_authenticated_actor]
struct SessionStore;

impl AuthenticatedActorStore for SessionStore {
    type Actor = User;
    async fn get_authenticated_actor(&self, request: &Request) -> Option<User> {}
}

#[decides_site_action(crate::action::Action::ViewAdmin)]
struct ViewAdminGate;

impl SiteActionGate for ViewAdminGate {
    type Actor = User;
    async fn can(&self, user: Option<&User>) -> bool {}
}

#[responds_to_http(method = Get, path = "/admin", server = "public")]
#[can(crate::action::Action::ViewAdmin)]
struct GetAdmin;

impl GetAdmin {
    #[responder]
    fn respond(&self, request: &Request) -> Response {}
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

        assert!(source.contains("usemargaret_security::crud_action::CrudAction;"));
        assert!(
            source.contains(
                "usemargaret_http::http_route_parameter_binder::HttpRouteParameterBinder;"
            )
        );
        assert!(!source.contains("CrudActionGateRegistry"));
        assert!(source.contains("container.gatekeeper().await"));
        assert!(source.contains("container.user_binder().await"));
        assert!(source.contains("letauthenticated_actor=gatekeeper.authenticate(&request).await;"));
        assert!(source.contains(r#"user_binder.bind(request.path_param("user").expect"#));
        assert!(source.contains("margaret_http::response::Response::not_found()"));
        assert!(source.contains(
            "if!gatekeeper.can_crud(authenticated_actor.as_ref(),&user,CrudAction::Read,).await{returnmargaret_http::response::Response::forbidden().into();}"
        ));
        assert!(source.contains("responder.respond(user).await"));
        assert!(!source.contains("get_authenticated_actor"));
    }

    #[test]
    fn injects_a_required_authenticated_actor() {
        let source = source_for(AUTHENTICATED_ACTOR);

        assert!(source.contains("container.gatekeeper().await"));
        assert!(source.contains("letauthenticated_actor=gatekeeper.authenticate(&request).await;"));
        assert!(source.contains("letuser=matchauthenticated_actor{Some(authenticated_actor)=>authenticated_actor,None=>{returnmargaret_http::response::Response::forbidden().into();}};"));
        assert!(source.contains("responder.respond(user).await"));
        assert!(!source.contains("AuthenticatedActorStore"));
    }

    #[test]
    fn accepts_a_destructured_authenticated_actor_argument() {
        let source = source_for(DESTRUCTURED_ACTOR);

        assert!(source.contains("letargument_1=matchauthenticated_actor{Some(authenticated_actor)=>authenticated_actor,None=>{returnmargaret_http::response::Response::forbidden().into();}};"));
        assert!(source.contains("responder.respond(argument_1).await"));
    }

    #[test]
    fn binds_a_destructured_route_parameter_named_by_from() {
        let source = source_for(DESTRUCTURED_ROUTE_PARAMETER);

        assert!(source.contains(r#"user_binder.bind(request.path_param("id").expect"#));
        assert!(source.contains("responder.respond(argument_1).await"));
    }

    #[test]
    fn rejects_a_from_that_is_absent_from_the_path() {
        let message = error_for(ROUTE_PARAMETER_FROM_UNKNOWN);

        assert!(message.contains("'missing'"));
        assert!(message.contains("does not appear in the route path"));
    }

    #[test]
    fn injects_an_optional_authenticated_actor() {
        let source = source_for(AUTHENTICATED_ACTOR);

        assert!(source.contains("letuser=authenticated_actor;"));
    }

    #[test]
    fn guards_a_responder_with_a_site_action() {
        let source = source_for(SITE_ACTION);

        assert!(source.contains("container.gatekeeper().await"));
        assert!(source.contains("letauthenticated_actor=gatekeeper.authenticate(&request).await;"));
        assert!(source.contains("if!gatekeeper.can_site_action(authenticated_actor.as_ref(),crate::action::Action::ViewAdmin,).await{returnmargaret_http::response::Response::forbidden().into();}"));
        assert!(source.contains("letrequest=&request;"));
        assert!(source.contains("responder.respond(request).await"));
        assert!(!source.contains("SiteActionGate"));
    }

    #[test]
    fn injects_a_bound_model_without_authorization() {
        let source = source_for(BOUND_WITHOUT_INTENT);

        assert!(
            source.contains(
                "usemargaret_http::http_route_parameter_binder::HttpRouteParameterBinder;"
            )
        );
        assert!(source.contains("container.user_binder().await"));
        assert!(source.contains(r#"user_binder.bind(request.path_param("user").expect"#));
        assert!(source.contains("margaret_http::response::Response::not_found()"));
        assert!(source.contains("responder.respond(user).await"));
        assert!(!source.contains("CrudAction"));
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
    fn rejects_a_route_parameter_of_an_unknown_type() {
        let message = error_for(
            "#[responds_to_http(method = Get, path = \"/things/{thing}\", server = \"public\")]\nstruct GetThing;\nimpl GetThing {\n    #[responder]\n    fn respond(&self, #[route_parameter(from = \"thing\")] thing: Unknown) -> Response {}\n}\n",
        );

        assert!(message.contains("no #[provides_route_parameter]"));
    }

    #[test]
    fn propagates_malformed_route_parameter_arguments() {
        let message = error_for(
            "#[responds_to_http(method = Get, path = \"/things/{thing}\", server = \"public\")]\nstruct GetThing;\nimpl GetThing {\n    #[responder]\n    fn respond(&self, #[route_parameter(= 5)] thing: String) -> Response {}\n}\n",
        );

        assert!(message.contains("failed to index"));
    }

    #[test]
    fn rejects_a_non_path_intent() {
        let message = error_for(
            "#[responds_to_http(method = Get, path = \"/things/{thing}\", server = \"public\")]\nstruct GetThing;\nimpl GetThing {\n    #[responder]\n    fn respond(&self, #[route_parameter(from = \"thing\", intent = 5)] thing: String) -> Response {}\n}\n",
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
    fn reports_a_gate_without_a_subject_associated_type() {
        let message = error_for("#[decides_crud_action]\nstruct Bare;\n");

        assert!(message.contains("type Subject"));
    }

    #[test]
    fn rejects_two_gates_for_the_same_subject() {
        let message = error_for(
            "struct User;\n\n#[decides_crud_action]\nstruct First;\nimpl First {\n    #[decides]\n    async fn can(&self, authenticated_actor: Option<&AuthenticatedActor<User>>, subject: &User, action: CrudAction) -> bool {}\n}\n\n#[decides_crud_action]\nstruct Second;\nimpl Second {\n    #[decides]\n    async fn can(&self, authenticated_actor: Option<&AuthenticatedActor<User>>, subject: &User, action: CrudAction) -> bool {}\n}\n",
        );

        assert!(message.contains("more than one CRUD gate"));
    }

    #[test]
    fn rejects_a_crud_gate_without_a_decision_method() {
        let message = error_for(
            "struct User;\n\n#[decides_crud_action]\nstruct ArticleGate;\nimpl ArticleGate {\n    async fn describe(&self) -> bool {}\n}\n",
        );

        assert!(message.contains("crate::ArticleGate"));
    }

    #[test]
    fn rejects_a_model_parameter_without_a_binder() {
        let message = error_for(
            "struct User;\n\n#[responds_to_http(method = Get, path = \"/users/{user}\", server = \"public\")]\nstruct GetUser;\nimpl GetUser {\n    #[responder]\n    fn respond(&self, #[route_parameter(from = \"user\")] user: User) -> Response {}\n}\n",
        );

        assert!(message.contains("no #[provides_route_parameter]"));
    }

    #[test]
    fn rejects_an_intent_without_a_gate() {
        let message = error_for(
            "struct User;\n\n#[provides_route_parameter]\nstruct UserBinder;\nimpl HttpRouteParameterBinder for UserBinder {\n    type Model = User;\n    async fn bind(&self, value: String) -> Option<User> {}\n}\n\n#[responds_to_http(method = Get, path = \"/users/{user}\", server = \"public\")]\nstruct GetUser;\nimpl GetUser {\n    #[responder]\n    fn respond(&self, #[route_parameter(from = \"user\", intent = CrudAction::Read)] user: User) -> Response {}\n}\n",
        );

        assert!(message.contains("no #[decides_crud_action] gate"));
    }

    #[test]
    fn rejects_an_intent_on_a_raw_parameter() {
        let message = error_for(
            "#[responds_to_http(method = Get, path = \"/users/{id}\", server = \"public\")]\nstruct GetUser;\nimpl GetUser {\n    #[responder]\n    fn respond(&self, #[route_parameter(from = \"id\", intent = CrudAction::Read)] id: String) -> Response {}\n}\n",
        );

        assert!(message.contains("intent only applies to model parameters"));
    }

    #[test]
    fn rejects_a_responder_without_a_responder_method() {
        let message = error_for(
            "#[responds_to_http(method = Get, path = \"/x\", server = \"public\")]\nstruct Bare;\n",
        );

        assert!(message.contains("no #[responder] method"));
    }

    #[test]
    fn rejects_an_unmarked_responder_parameter() {
        let message = error_for(
            "#[responds_to_http(method = Get, path = \"/x/{id}\", server = \"public\")]\nstruct Bad;\n\nimpl Bad {\n    #[responder]\n    fn respond(&self, id: String) -> Response {}\n}\n",
        );

        assert!(message.contains("must be a route parameter"));
    }

    #[test]
    fn rejects_a_non_string_route_parameter_source() {
        let message = error_for(
            "#[responds_to_http(method = Get, path = \"/x/{id}\", server = \"public\")]\nstruct Bad;\n\nimpl Bad {\n    #[responder]\n    fn respond(&self, #[route_parameter(from = 5)] id: String) -> Response {}\n}\n",
        );

        assert!(message.contains("failed to index"));
    }

    #[test]
    fn rejects_a_route_parameter_without_from() {
        let message = error_for(
            "#[responds_to_http(method = Get, path = \"/x/{id}\", server = \"public\")]\nstruct Bad;\n\nimpl Bad {\n    #[responder]\n    fn respond(&self, #[route_parameter] id: String) -> Response {}\n}\n",
        );

        assert!(message.contains("must name the path parameter it binds"));
    }

    #[test]
    fn rejects_a_route_parameter_absent_from_the_path() {
        let message = error_for(
            "#[responds_to_http(method = Get, path = \"/users/{id}\", server = \"public\")]\nstruct Bad;\n\nimpl Bad {\n    #[responder]\n    fn respond(&self, #[route_parameter(from = \"slug\")] slug: String) -> Response {}\n}\n",
        );

        assert!(message.contains("does not appear in the route path"));
    }

    #[test]
    fn rejects_a_malformed_route_path() {
        let message = error_for(
            "#[responds_to_http(method = Get, path = \"/users/{id\", server = \"public\")]\nstruct Bad;\n\nimpl Bad {\n    #[responder]\n    fn respond(&self) -> Response {}\n}\n",
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
            "#[responds_to_http(method = Get, path = \"/x\", server = \"public\")]\nenum Bad {}\n",
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
    fn propagates_a_non_path_method_argument() {
        let message = error_for(
            "#[responds_to_http(method = \"GET\", path = \"/x\", server = \"public\")]\nstruct Bad;\n",
        );

        assert!(message.contains("failed to index"));
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
            "#[responds_to_http(method = Get, path = 5, server = \"public\")]\nstruct Bad;\n",
        );

        assert!(message.contains("failed to index"));
    }

    #[test]
    fn rejects_a_responder_without_a_path() {
        assert!(
            error_for("#[responds_to_http(method = Get, server = \"public\")]\nstruct Bad;\n")
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
    fn rejects_a_malformed_marker() {
        let message = error_for(
            "#[responds_to_http(method = Get, path = \"/x\", server = \"public\")]\n#[guard(crate::A, crate::B)]\nstruct Bad;\n\n#[handles_middleware_attribute(attribute = guard)]\nstruct Guard;\n",
        );

        assert!(message.contains("must carry zero or one positional argument"));
    }

    #[test]
    fn propagates_malformed_marker_arguments() {
        let message = error_for(
            "#[responds_to_http(method = Get, path = \"/x\", server = \"public\")]\n#[guard(= 5)]\nstruct Bad;\n\n#[handles_middleware_attribute(attribute = guard)]\nstruct Guard;\n",
        );

        assert!(message.contains("failed to index"));
    }

    #[test]
    fn reports_responders_present() {
        let index = index_for(
            "#[responds_to_http(method = Get, path = \"/x\", server = \"public\")]\nstruct R;\n",
        );

        assert!(has_responders(&index));
    }

    #[test]
    fn reports_no_responders() {
        let index = index_for("#[singleton]\nstruct S;\n");

        assert!(!has_responders(&index));
    }

    #[test]
    fn rejects_two_authenticated_actor_stores() {
        let message = error_for(
            "struct User;\n\n#[provides_authenticated_actor]\nstruct First;\nimpl AuthenticatedActorStore for First {\n    type Actor = User;\n    async fn get_authenticated_actor(&self, request: &Request) -> Option<User> {}\n}\n\n#[provides_authenticated_actor]\nstruct Second;\nimpl AuthenticatedActorStore for Second {\n    type Actor = User;\n    async fn get_authenticated_actor(&self, request: &Request) -> Option<User> {}\n}\n",
        );

        assert!(message.contains("exactly one is allowed"));
    }

    #[test]
    fn rejects_an_authenticated_actor_responder_without_a_store() {
        let message = error_for(
            "struct User;\n\n#[responds_to_http(method = Get, path = \"/account\", server = \"public\")]\nstruct GetAccount;\nimpl GetAccount {\n    #[responder]\n    fn respond(&self, user: AuthenticatedActor<User>) -> Response {}\n}\n",
        );

        assert!(message.contains("no #[provides_authenticated_actor] store"));
    }

    #[test]
    fn rejects_a_site_action_gate_without_an_action() {
        let message = error_for("#[decides_site_action]\nstruct Bare;\n");

        assert!(message.contains("missing its site action argument"));
    }

    #[test]
    fn rejects_two_gates_for_the_same_site_action() {
        let message = error_for(
            "struct User;\n\n#[decides_site_action(crate::action::Action::ViewAdmin)]\nstruct First;\nimpl SiteActionGate for First {\n    type Actor = User;\n    async fn can(&self, user: Option<&User>) -> bool {}\n}\n\n#[decides_site_action(crate::action::Action::ViewAdmin)]\nstruct Second;\nimpl SiteActionGate for Second {\n    type Actor = User;\n    async fn can(&self, user: Option<&User>) -> bool {}\n}\n",
        );

        assert!(message.contains("more than one gate"));
    }

    #[test]
    fn rejects_a_can_attribute_without_an_action() {
        let message = error_for(
            "#[responds_to_http(method = Get, path = \"/admin\", server = \"public\")]\n#[can]\nstruct GetAdmin;\nimpl GetAdmin {\n    #[responder]\n    fn respond(&self) -> Response {}\n}\n",
        );

        assert!(message.contains("#[can] attribute without a site action"));
    }

    #[test]
    fn rejects_a_can_guard_without_a_matching_gate() {
        let message = error_for(
            "#[responds_to_http(method = Get, path = \"/admin\", server = \"public\")]\n#[can(crate::action::Action::ViewAdmin)]\nstruct GetAdmin;\nimpl GetAdmin {\n    #[responder]\n    fn respond(&self) -> Response {}\n}\n",
        );

        assert!(message.contains("no #[decides_site_action] gate decides it"));
    }

    #[test]
    fn propagates_malformed_can_arguments() {
        let message = error_for(
            "#[responds_to_http(method = Get, path = \"/admin\", server = \"public\")]\n#[can(= 5)]\nstruct GetAdmin;\nimpl GetAdmin {\n    #[responder]\n    fn respond(&self) -> Response {}\n}\n",
        );

        assert!(message.contains("failed to index"));
    }

    #[test]
    fn propagates_malformed_decides_site_action_arguments() {
        let message = error_for("#[decides_site_action(= 5)]\nstruct Bad;\n");

        assert!(message.contains("failed to index"));
    }

    #[test]
    fn never_generates_a_route_name_enum_and_omits_names_for_plain_routes() {
        let source = source_for(RESPONDERS_AND_MIDDLEWARE);

        assert!(!source.contains("enumRouteName"));
        assert!(!source.contains("route_with_name"));
        assert!(source.contains(".route(margaret_http::method::Method::Get,\"/open\","));
    }

    const NAMED_ROUTE: &str = r#"
#[responds_to_http(
    method = Get,
    name = "get_greeting",
    path = "/greeting",
    server = "public"
)]
struct GetGreeting;

impl GetGreeting {
    #[responder]
    fn respond(&self) -> Response {}
}
"#;

    #[test]
    fn registers_a_route_under_an_explicit_name() {
        let source = source_for(NAMED_ROUTE);

        assert!(!source.contains("enumRouteName"));
        assert!(source.contains(
            ".route_with_name(margaret_http::method::Method::Get,\"/greeting\",\"get_greeting\","
        ));
    }

    #[test]
    fn rejects_two_routes_sharing_a_name() {
        let message = error_for(
            "#[responds_to_http(method = Get, name = \"shared\", path = \"/a\", server = \"public\")]\nstruct A;\nimpl A {\n    #[responder]\n    fn respond(&self) -> Response {}\n}\n\n#[responds_to_http(method = Get, name = \"shared\", path = \"/b\", server = \"public\")]\nstruct B;\nimpl B {\n    #[responder]\n    fn respond(&self) -> Response {}\n}\n",
        );

        assert!(message.contains("both declare the route name"));
    }

    #[test]
    fn propagates_a_non_string_name_argument() {
        let message = error_for(
            "#[responds_to_http(method = Get, name = crate::symbols::RouteName::Shared, path = \"/a\", server = \"public\")]\nstruct A;\nimpl A {\n    #[responder]\n    fn respond(&self) -> Response {}\n}\n",
        );

        assert!(message.contains("failed to index"));
    }

    #[test]
    fn routes_an_interceptable_return_through_its_marker_interceptor() {
        let source = source_for(
            "trait View {}\n\n#[intercepts]\nstruct ViewInterceptor;\nimpl HttpInterceptor for ViewInterceptor {\n    type Intercepted = dyn View;\n    async fn intercept(&self, request: &Request, view: Box<dyn View>) -> Response {}\n}\n\n#[responds_to_http(method = Get, path = \"/greeting\", server = \"public\")]\nstruct GetGreeting;\nimpl GetGreeting {\n    #[responder]\n    fn respond(&self) -> Box<dyn View> {}\n}\n",
        );

        assert!(source.contains("container.view_interceptor().await"));
        assert!(source.contains(
            "margaret_http::responded::Responded::Intercept(Box::new(margaret_http::interception::Interception::new(view_interceptor,responder.respond().await"
        ));
    }

    #[test]
    fn reports_an_interceptor_without_an_intercepted_marker_trait() {
        let message = error_for("#[intercepts]\nstruct Bare;\n");

        assert!(message.contains("type Intercepted"));
    }

    #[test]
    fn rejects_two_interceptors_for_the_same_marker_trait() {
        let message = error_for(
            "trait View {}\n\n#[intercepts]\nstruct First;\nimpl HttpInterceptor for First {\n    type Intercepted = dyn View;\n    async fn intercept(&self, request: &Request, view: Box<dyn View>) -> Response {}\n}\n\n#[intercepts]\nstruct Second;\nimpl HttpInterceptor for Second {\n    type Intercepted = dyn View;\n    async fn intercept(&self, request: &Request, view: Box<dyn View>) -> Response {}\n}\n",
        );

        assert!(message.contains("more than one interceptor"));
    }

    #[test]
    fn treats_a_responder_without_a_return_type_as_plain() {
        let source = source_for(
            "#[responds_to_http(method = Get, path = \"/x\", server = \"public\")]\nstruct Page;\nimpl Page {\n    #[responder]\n    fn respond(&self) {}\n}\n",
        );

        assert!(
            source.contains("margaret_http::responded::Responded::from(responder.respond().await)")
        );
    }

    #[test]
    fn treats_an_unregistered_struct_return_as_plain() {
        let source = source_for(
            "struct Card;\n\n#[responds_to_http(method = Get, path = \"/x\", server = \"public\")]\nstruct GetCard;\nimpl GetCard {\n    #[responder]\n    fn respond(&self) -> Card {}\n}\n",
        );

        assert!(
            source.contains("margaret_http::responded::Responded::from(responder.respond().await)")
        );
    }

    #[test]
    fn treats_a_trait_object_return_without_an_interceptor_as_plain() {
        let source = source_for(
            "trait Widget {}\n\n#[responds_to_http(method = Get, path = \"/x\", server = \"public\")]\nstruct GetWidget;\nimpl GetWidget {\n    #[responder]\n    fn respond(&self) -> Box<dyn Widget> {}\n}\n",
        );

        assert!(
            source.contains("margaret_http::responded::Responded::from(responder.respond().await)")
        );
    }

    const MULTIPLE_SERVERS: &str = r#"
#[responds_to_http(method = Get, path = "/", server = "public")]
struct GetIndex;

impl GetIndex {
    #[responder]
    fn respond(&self) -> Response {}
}

#[responds_to_http(method = Get, path = "/metrics", server = "internal")]
struct GetMetrics;

impl GetMetrics {
    #[responder]
    fn respond(&self) -> Response {}
}
"#;

    #[test]
    fn rejects_a_route_without_a_server() {
        let message = error_for(
            "#[responds_to_http(method = Get, path = \"/\")]\nstruct GetIndex;\nimpl GetIndex {\n    #[responder]\n    fn respond(&self) -> Response {}\n}\n",
        );

        assert!(message.contains("missing the 'server' argument"));
    }

    #[test]
    fn groups_each_route_under_its_named_server() {
        let source = source_for(MULTIPLE_SERVERS);

        assert!(source.contains(
            "server_public(container:&Container)->margaret_http::server::Server{letrouter=margaret_http::router::Router::empty().route(margaret_http::method::Method::Get,\"/\","
        ));
        assert!(source.contains(
            "server_internal(container:&Container)->margaret_http::server::Server{letrouter=margaret_http::router::Router::empty().route(margaret_http::method::Method::Get,\"/metrics\","
        ));
    }

    #[test]
    fn normalizes_a_server_name_to_snake_case() {
        let source = source_for(
            "#[responds_to_http(method = Get, path = \"/\", server = \"Public API\")]\nstruct Page;\nimpl Page {\n    #[responder]\n    fn respond(&self) -> Response {}\n}\n",
        );

        assert!(source.contains("pubasyncfnserver_public_api"));
    }

    #[test]
    fn propagates_a_non_string_server_argument() {
        let message = error_for(
            "#[responds_to_http(method = Get, path = \"/\", server = ServerMarker)]\nstruct Page;\nimpl Page {\n    #[responder]\n    fn respond(&self) -> Response {}\n}\n",
        );

        assert!(message.contains("failed to index"));
    }
}
