pub mod has_websocket_sessions;
pub mod render_websocket;
pub mod websocket_artifacts;
pub mod websocket_codegen_error;

mod build_for_session_method;
mod build_websocket_plan;
mod discovered_handler;
mod handler_binding;
mod handler_kind;
mod message_cardinality;
mod message_kind;
mod render_messages;
mod render_server_routes;
mod render_sessions;
mod session_arguments;
mod session_console_arguments;
mod session_handler_plan;
mod session_plan;
mod websocket_handlers;
mod websocket_message;
mod websocket_messages;
mod websocket_plan;
mod websocket_session;
mod websocket_sessions;

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use margaret_attributes::attribute_index::AttributeIndex;
    use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
    use margaret_attributes::crate_root::CrateRoot;
    use margaret_container::container_bindings::ContainerBindings;
    use margaret_container::render_container::render_container;
    use margaret_middleware_codegen::middleware_plans::middleware_plans;
    use margaret_request_binding_codegen::binding_registries::BindingRegistries;
    use margaret_request_binding_codegen::views_availability::ViewsAvailability;

    use crate::has_websocket_sessions::has_websocket_sessions;
    use crate::render_websocket::render_websocket;
    use crate::websocket_codegen_error::WebSocketCodegenError;
    use crate::websocket_handlers::websocket_handlers;

    const REQUEST_TRAIT: &str = "use margaret::framework::websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;\n";
    const NOTIFICATION_TRAIT: &str = "use margaret::framework::websocket::responds_to_web_socket_notification::RespondsToWebSocketNotification;\n";

    const FULL_SESSION: &str = r#"
use std::sync::Arc;
use margaret::framework::websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;
use margaret::framework::websocket::responds_to_web_socket_notification::RespondsToWebSocketNotification;

#[singleton]
struct SystemClock;

impl SystemClock {
    #[constructor]
    fn new() -> Self {}
}

#[singleton]
struct Config;

impl Config {
    #[constructor]
    fn new() -> Self {}
}

#[singleton]
struct LogPlugin;

impl LogPlugin {
    #[constructor]
    fn new() -> Self {}
}

#[websocket_session(path = "/chat/{room}", server = "public")]
struct ChatSession;

impl ChatSession {
    #[build_for_session]
    fn build_for_session(
        clock: Arc<SystemClock>,
        config: Arc<Config>,
        plugin: Arc<LogPlugin>,
        #[route_parameter(from = "room")] room: String,
    ) -> Self {}
}

#[websocket_message(request, method = "say", response = stream)]
struct Say;

#[websocket_message(request, method = "ping", response = single)]
struct Ping;

#[websocket_message(notification, method = "typing")]
struct Typing;

#[websocket_message(response, method = "chunk")]
struct Chunk;

#[singleton]
struct Speaker;

impl RespondsToWebSocketMessage for Speaker {
    type Session = ChatSession;
    type Message = Say;
}

#[singleton]
struct Ponger;

impl RespondsToWebSocketMessage for Ponger {
    type Session = ChatSession;
    type Message = Ping;
}

#[singleton]
struct Typist;

impl RespondsToWebSocketNotification for Typist {
    type Session = ChatSession;
    type Message = Typing;
}
"#;

    fn index_for(source: &str) -> AttributeIndex {
        let directory = tempdir().expect("a temporary crate directory is created");
        let source_directory = directory.path().join("src");

        fs::create_dir(&source_directory).expect("the src directory is created");
        fs::write(source_directory.join("lib.rs"), source).expect("lib.rs is written");

        AttributeIndexBuilder::new()
            .index_crate(&CrateRoot::new("crate", source_directory))
            .expect("the crate is indexed")
            .build()
    }

    fn bindings(index: &AttributeIndex) -> ContainerBindings {
        let registry = margaret_console_argument_codegen::scan::scan(index)
            .expect("the console arguments are scanned");

        render_container(index, &registry, &[])
            .expect("the container renders")
            .bindings
    }

    fn registries_for(index: &AttributeIndex) -> BindingRegistries {
        BindingRegistries::collect(index, ViewsAvailability::Available)
            .expect("the binding registries are collected")
    }

    fn generated(source: &str) -> String {
        let index = index_for(source);
        let registries = registries_for(&index);
        let plans =
            middleware_plans(&index, &registries).expect("the middleware plans are collected");

        render_websocket(&index, &bindings(&index), &plans, &registries)
            .expect("the websocket module is generated")
            .modules
            .into_iter()
            .map(|module| {
                module
                    .format()
                    .expect("the module formats")
                    .source()
                    .split_whitespace()
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("")
    }

    fn error(source: &str) -> WebSocketCodegenError {
        let index = index_for(source);
        let registries = match BindingRegistries::collect(&index, ViewsAvailability::Available) {
            Ok(registries) => registries,
            Err(rejection) => return WebSocketCodegenError::from(rejection),
        };
        let plans =
            middleware_plans(&index, &registries).expect("the middleware plans are collected");

        render_websocket(&index, &bindings(&index), &plans, &registries)
            .expect_err("the websocket module is rejected")
    }

    const CONSOLE_ARGUMENT_HANDLER: &str = r#"
use margaret::framework::websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;

#[websocket_session(path = "/room", server = "public")]
struct Room;

impl Room {
    #[build_for_session]
    fn build() -> Self {}
}

#[websocket_message(request, method = "chat", response = single)]
struct Chat;

#[singleton]
struct Chatter;

impl Chatter {
    #[constructor]
    fn create(#[console_argument(from = "greeting")] greeting: String) -> Self {}
}

impl RespondsToWebSocketMessage for Chatter {
    type Session = Room;
    type Message = Chat;
}
"#;

    #[test]
    fn weaves_a_console_argument_through_the_websocket_dispatch_chain() {
        let source = generated(CONSOLE_ARGUMENT_HANDLER);

        assert!(source.contains("console_argument_0:&str"));
        assert!(source.contains("container.chatter(console_argument_0.to_owned()).await"));
        assert!(source.contains("dispatch_table(container,console_argument_0).await"));
        assert!(source.contains(
            "public_routes(container:&super::container::Container,console_argument_0:&str,"
        ));
        assert!(source.contains("upgrade_entry(container,console_argument_0"));
    }

    const NON_STRUCT_HANDLER: &str = r#"
use margaret::framework::websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;

#[websocket_message(request, method = "chat", response = single)]
struct Chat;

#[singleton]
enum Chatter {}

impl RespondsToWebSocketMessage for Chatter {
    type Session = Chat;
    type Message = Chat;
}
"#;

    #[test]
    fn rejects_a_handler_that_is_not_a_struct() {
        let error = websocket_handlers(&index_for(NON_STRUCT_HANDLER))
            .err()
            .expect("a non-struct websocket handler is rejected");

        assert!(
            error
                .to_string()
                .contains("handler trait but is not a struct")
        );
    }

    const COLLIDING_HANDLER_FIELD: &str = r#"
use margaret::framework::websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;

#[websocket_message(request, method = "chat", response = single)]
struct Chat;

mod a {
    pub mod b {
        #[singleton]
        pub struct Handler;

        impl Handler {
            #[constructor]
            fn new() -> Self {}
        }
    }
}

mod a_b {
    use margaret::framework::websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;

    #[singleton]
    pub struct Handler;

    impl RespondsToWebSocketMessage for Handler {
        type Session = crate::Chat;
        type Message = crate::Chat;
    }
}
"#;

    #[test]
    fn resolves_a_handler_to_its_disambiguated_container_field() {
        let handlers = websocket_handlers(&index_for(COLLIDING_HANDLER_FIELD))
            .expect("the handlers are discovered");

        assert_eq!(handlers.len(), 1);
        assert_eq!(handlers[0].handler_field, "a_b_handler_2");
    }

    const COLLIDING_DISPATCH_METHODS: &str = r#"
use margaret::framework::websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;

#[websocket_session(path = "/room", server = "public")]
struct Room;

impl Room {
    #[build_for_session]
    fn build() -> Self {}
}

#[websocket_message(request, method = "say_hi", response = single)]
struct SayHi;

#[websocket_message(request, method = "say__hi", response = single)]
struct SayHiToo;

#[singleton]
struct FirstHandler;

impl RespondsToWebSocketMessage for FirstHandler {
    type Session = Room;
    type Message = SayHi;
}

#[singleton]
struct SecondHandler;

impl RespondsToWebSocketMessage for SecondHandler {
    type Session = Room;
    type Message = SayHiToo;
}
"#;

    #[test]
    fn disambiguates_dispatch_structs_whose_methods_share_an_upper_camel_form() {
        let source = generated(COLLIDING_DISPATCH_METHODS);

        assert!(source.contains("structSayHiDispatch"));
        assert!(source.contains("structSayHi2Dispatch"));
    }

    const SESSION_WITH_MIDDLEWARE: &str = r#"
use margaret::framework::http::next::Next;
use margaret::framework::http::request::Request;

#[websocket_session(path = "/room", server = "public")]
#[middleware(guard)]
struct Room;

impl Room {
    #[build_for_session]
    fn build() -> Self {}
}

#[handles_middleware_attribute(attribute = guard)]
struct Guard;

impl Guard {
    #[process]
    fn process(&self, request: &Request, next: Next) -> ResponseContinuation {}
}
"#;

    const SESSION_WITH_ROUTES_MIDDLEWARE: &str = r#"
use crate::margaret::routes::Routes;
use margaret::framework::http::next::Next;
use margaret::framework::http::request::Request;

#[websocket_session(path = "/room", server = "public")]
#[middleware(traced)]
struct Room;

impl Room {
    #[build_for_session]
    fn build() -> Self {}
}

#[handles_middleware_attribute(attribute = traced)]
struct Tracer;

impl Tracer {
    #[process]
    fn process(&self, request: &Request, next: Next, routes: &Routes) -> ResponseContinuation {}
}
"#;

    const INJECTED_ONLY_SESSION: &str = r#"
use std::sync::Arc;

#[singleton]
struct SystemClock;

impl SystemClock {
    #[constructor]
    fn new() -> Self {}
}

#[websocket_session(path = "/room", server = "public")]
struct Room;

impl Room {
    #[build_for_session]
    fn build_for_session(clock: Arc<SystemClock>) -> Self {}
}
"#;

    #[test]
    fn names_the_factory_handshake_unused_when_no_binding_reads_the_request() {
        let source = generated(INJECTED_ONLY_SESSION);

        assert!(source.contains("&self,_handshake:&margaret::framework::http::request::Request"));
    }

    #[test]
    fn names_the_factory_handshake_when_a_binding_reads_the_request() {
        let source = generated(FULL_SESSION);

        assert!(source.contains("&self,handshake:&margaret::framework::http::request::Request"));
    }

    #[test]
    fn names_the_dispatch_container_unused_when_a_session_has_no_handlers() {
        let source = generated(INJECTED_ONLY_SESSION);

        assert!(source.contains("dispatch_table(_container:&super::super::container::Container"));
    }

    #[test]
    fn names_the_dispatch_container_when_a_session_has_handlers() {
        let source = generated(FULL_SESSION);

        assert!(source.contains("dispatch_table(container:&super::super::container::Container"));
    }

    #[test]
    fn carries_its_middleware_on_the_route_entry() {
        let source = generated(SESSION_WITH_MIDDLEWARE);

        assert!(source.contains("margaret::framework::http::route_entry::RouteEntry::web_socket("));
        assert!(source.contains(
            "middleware.push(std::sync::Arc::new(super::middleware::Guard{inner:container.guard().await"
        ));
        assert!(!source.contains("GatedWebSocketUpgrade"));
    }

    const SESSION_WITH_CONSOLE_ARGUMENT_MIDDLEWARE: &str = r#"
use margaret::framework::http::next::Next;
use margaret::framework::http::request::Request;

#[websocket_session(path = "/room", server = "public")]
#[middleware(guard)]
struct Room;

impl Room {
    #[build_for_session]
    fn build() -> Self {}
}

#[singleton]
#[handles_middleware_attribute(attribute = guard)]
struct Guard;

impl Guard {
    #[constructor]
    fn create(#[console_argument(from = "token")] token: String) -> Self {}

    #[process]
    fn process(&self, request: &Request, next: Next) -> ResponseContinuation {}
}
"#;

    #[test]
    fn weaves_a_console_argument_into_a_session_middleware() {
        let source = generated(SESSION_WITH_CONSOLE_ARGUMENT_MIDDLEWARE);

        assert!(source.contains(
            "public_routes(container:&super::container::Container,console_argument_0:&str,"
        ));
        assert!(source.contains("container.guard(console_argument_0.to_owned()).await"));
    }

    #[test]
    fn rejects_a_session_with_an_unknown_middleware_tag() {
        assert!(
            error(
                "#[websocket_session(path = \"/room\", server = \"public\")]\n#[middleware(missing)]\nstruct Room;\n\nimpl Room {\n    #[build_for_session]\n    fn build() -> Self {}\n}\n"
            )
            .to_string()
            .contains("no #[handles_middleware_attribute] handles it")
        );
    }

    #[test]
    fn weaves_routes_when_only_a_middleware_injects_them() {
        let source = generated(SESSION_WITH_ROUTES_MIDDLEWARE);

        assert!(source.contains("routes:&::std::sync::Arc<super::routes::Routes>"));
        assert!(source.contains(
            "super::middleware::Tracer{inner:container.tracer().await?,routes:routes.clone()"
        ));
        assert!(source.contains("upgrade_entry(container)"));
    }

    #[test]
    fn generates_the_session_module_declarations_and_message_envelopes() {
        let source = generated(FULL_SESSION);

        assert!(source.contains("pubmodchat_session"));
        assert!(source.contains("WebSocketRequestMessageforcrate::Say"));
        assert!(source.contains("streaming_request_envelope::StreamingRequestEnvelope::new"));
        assert!(source.contains("WebSocketRequestMessageforcrate::Ping"));
        assert!(source.contains("request_envelope::RequestEnvelope::new"));
        assert!(!source.contains("WebSocketRequestMessageforcrate::Typing"));
        assert!(!source.contains("WebSocketRequestMessageforcrate::Chunk"));
        assert!(source.contains("WebSocketResponseMessageforcrate::Chunk"));
        assert!(source.contains("constMETHOD:&'staticstr=\"chunk\""));
        assert!(!source.contains("WebSocketResponseMessageforcrate::Say"));
        assert!(!source.contains("WebSocketResponseMessageforcrate::Typing"));
    }

    #[test]
    fn omits_the_request_method_from_the_request_dispatch() {
        let source = generated(FULL_SESSION);

        assert!(source.contains("RequestId,params:serde_json::Value"));
        assert!(!source.contains("method:::std::string::String"));
    }

    #[test]
    fn generates_a_factory_with_concrete_dependencies() {
        let source = generated(FULL_SESSION);

        assert!(source.contains("clock:::std::sync::Arc<crate::SystemClock>"));
        assert!(source.contains("config:::std::sync::Arc<crate::Config>"));
        assert!(source.contains("plugin:::std::sync::Arc<crate::LogPlugin>"));
        assert!(source.contains("container.system_clock().await"));
        assert!(source.contains("container.config().await"));
        assert!(source.contains("container.log_plugin().await"));
    }

    #[test]
    fn generates_a_factory_that_extracts_route_parameters() {
        let source = generated(FULL_SESSION);

        assert!(source.contains("require_route_parameter::require_route_parameter"));
        assert!(source.contains("\"room\""));
        assert!(source.contains("crate::ChatSession::build_for_session"));
        assert!(source.contains("self.clock.clone()"));
    }

    #[test]
    fn generates_request_and_notification_dispatch_tables() {
        let source = generated(FULL_SESSION);

        assert!(source.contains("SayDispatch"));
        assert!(source.contains("TypingDispatch"));
        assert!(source.contains("dispatch_request::dispatch_request"));
        assert!(source.contains("dispatch_notification::dispatch_notification"));
        assert!(source.contains("requests.insert(\"say\""));
        assert!(source.contains("notifications.insert(\"typing\""));
        assert!(source.contains("container.speaker().await"));
    }

    #[test]
    fn generates_a_public_upgrade_entry() {
        let source = generated(FULL_SESSION);

        assert!(source.contains("pubasyncfnupgrade_entry"));
        assert!(source.contains("web_socket_upgrade_entry::WebSocketUpgradeEntry::new"));
        assert!(source.contains("dispatch_table(container).await"));
    }

    #[test]
    fn generates_a_route_registration_function_per_server() {
        let source = generated(FULL_SESSION);

        assert!(source.contains("pubasyncfnpublic_routes"));
        assert!(source.contains("route_entry::RouteEntry::web_socket(\"/chat/{room}\""));
        assert!(source.contains("chat_session::upgrade_entry(container).await"));
    }

    #[test]
    fn rejects_a_message_that_is_not_a_struct() {
        assert!(
            error(r#"#[websocket_message(response)] enum Bad {}"#)
                .to_string()
                .contains("carries #[websocket_message]")
        );
    }

    #[test]
    fn rejects_a_message_without_a_kind() {
        assert!(
            error(r#"#[websocket_message(method = "x")] struct Bad;"#)
                .to_string()
                .contains("must declare a message kind")
        );
    }

    #[test]
    fn rejects_a_message_with_multiple_kinds() {
        assert!(error(r#"#[websocket_message(request, response, method = "x", response = single)] struct Bad;"#).to_string().contains("more than one message kind"));
    }

    #[test]
    fn rejects_a_request_without_a_method() {
        assert!(
            error(r#"#[websocket_message(request, response = single)] struct Bad;"#)
                .to_string()
                .contains("missing the required 'method'")
        );
    }

    #[test]
    fn rejects_a_request_without_a_cardinality() {
        assert!(
            error(r#"#[websocket_message(request, method = "x")] struct Bad;"#)
                .to_string()
                .contains("must declare 'response = single'")
        );
    }

    #[test]
    fn rejects_a_request_with_an_invalid_cardinality() {
        assert!(
            error(r#"#[websocket_message(request, method = "x", response = burst)] struct Bad;"#)
                .to_string()
                .contains("invalid cardinality")
        );
    }

    #[test]
    fn rejects_a_non_snake_case_method() {
        assert!(error(r#"#[websocket_message(request, method = "NotSnake", response = single)] struct Bad;"#).to_string().contains("has method"));
    }

    #[test]
    fn rejects_a_response_without_a_method() {
        assert!(
            error(r#"#[websocket_message(response)] struct Bad;"#)
                .to_string()
                .contains("missing the required 'method'")
        );
    }

    #[test]
    fn rejects_a_non_snake_case_response_method() {
        assert!(
            error(r#"#[websocket_message(response, method = "NotSnake")] struct Bad;"#)
                .to_string()
                .contains("has method")
        );
    }

    #[test]
    fn rejects_two_responses_that_share_a_method() {
        assert!(
            error(
                "#[websocket_message(response, method = \"dup\")] struct First;\n#[websocket_message(response, method = \"dup\")] struct Second;\n"
            )
            .to_string()
            .contains("declared more than once")
        );
    }

    #[test]
    fn rejects_a_notification_that_declares_a_cardinality() {
        assert!(
            error(
                r#"#[websocket_message(notification, method = "x", response = single)] struct Bad;"#
            )
            .to_string()
            .contains("declares a response cardinality but is not a request")
        );
    }

    #[test]
    fn rejects_a_notification_whose_response_argument_is_not_a_path() {
        assert!(
            error(
                r#"#[websocket_message(notification, method = "x", response = "single")] struct Bad;"#
            )
            .to_string()
            .contains("is not a path")
        );
    }

    #[test]
    fn rejects_a_session_that_is_not_a_struct() {
        assert!(
            error(r#"#[websocket_session(path = "/x", server = "public")] enum Bad {}"#)
                .to_string()
                .contains("carries #[websocket_session]")
        );
    }

    #[test]
    fn rejects_a_session_without_a_path() {
        assert!(
            error(r#"#[websocket_session(server = "public")] struct Bad;"#)
                .to_string()
                .contains("missing the required 'path'")
        );
    }

    #[test]
    fn rejects_a_session_without_a_server() {
        assert!(
            error(r#"#[websocket_session(path = "/x")] struct Bad;"#)
                .to_string()
                .contains("missing the required 'server'")
        );
    }

    #[test]
    fn rejects_a_session_with_a_non_snake_case_server() {
        assert!(
            error(r#"#[websocket_session(path = "/x", server = "Public")] struct Bad;"#)
                .to_string()
                .contains("has server")
        );
    }

    #[test]
    fn rejects_a_session_without_a_build_for_session_method() {
        assert!(
            error(r#"#[websocket_session(path = "/x", server = "public")] struct Bad;"#)
                .to_string()
                .contains("has no #[build_for_session] method")
        );
    }

    #[test]
    fn rejects_a_session_with_two_build_for_session_methods() {
        assert!(
            error(
                r#"
#[websocket_session(path = "/x", server = "public")]
struct Bad;

impl Bad {
    #[build_for_session]
    fn first() -> Self {}

    #[build_for_session]
    fn second() -> Self {}
}
"#
            )
            .to_string()
            .contains("more than one #[build_for_session] method")
        );
    }

    #[test]
    fn rejects_a_build_for_session_that_does_not_return_self() {
        assert!(
            error(
                r#"
#[websocket_session(path = "/x", server = "public")]
struct Bad;

impl Bad {
    #[build_for_session]
    fn build() {}
}
"#
            )
            .to_string()
            .contains("must return Self")
        );
    }

    #[test]
    fn rejects_a_route_parameter_without_a_from() {
        assert!(
            error(
                r#"
#[websocket_session(path = "/x/{id}", server = "public")]
struct Bad;

impl Bad {
    #[build_for_session]
    fn build(#[route_parameter] id: String) -> Self {}
}
"#
            )
            .to_string()
            .contains("is missing `from")
        );
    }

    #[test]
    fn rejects_a_route_parameter_missing_from_the_path() {
        assert!(
            error(
                r#"
#[websocket_session(path = "/x", server = "public")]
struct Bad;

impl Bad {
    #[build_for_session]
    fn build(#[route_parameter(from = "id")] id: String) -> Self {}
}
"#
            )
            .to_string()
            .contains("does not appear in the route path")
        );
    }

    #[test]
    fn rejects_an_unsupported_session_parameter_shape() {
        assert!(
            error(
                r#"
#[websocket_session(path = "/x", server = "public")]
struct Bad;

impl Bad {
    #[build_for_session]
    fn build(value: String) -> Self {}
}
"#
            )
            .to_string()
            .contains("is not an injectable dependency")
        );
    }

    #[test]
    fn rejects_a_session_parameter_without_a_provider() {
        assert!(
            error(
                r#"
use std::sync::Arc;

#[websocket_session(path = "/x", server = "public")]
struct Bad;

impl Bad {
    #[build_for_session]
    fn build(missing: Arc<Unknown>) -> Self {}
}
"#
            )
            .to_string()
            .contains("no #[singleton] provides it")
        );
    }

    const AUTHENTICATED_HANDSHAKE: &str = r#"
use margaret::framework::http::request::Request;
use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret::framework::websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;

struct User;

struct SessionCookie;

#[singleton]
#[infers_authenticated_user(user_model = User)]
struct SessionUserProvider;

impl SessionUserProvider {
    #[infer_from_request]
    fn infer(&self, request: &Request, #[form_request(from = Cookie)] cookie: SessionCookie) -> AuthenticatedUserOutcome<User> {}
}

#[websocket_session(path = "/room", server = "public")]
struct Room;

impl Room {
    #[build_for_session]
    fn assemble(#[authenticated_user] viewer: Option<User>) -> Self {}
}

#[websocket_message(request, method = "chat", response = single)]
struct Chat;

#[singleton]
struct Chatter;

impl RespondsToWebSocketMessage for Chatter {
    type Session = Room;
    type Message = Chat;
}
"#;

    #[test]
    fn holds_the_authenticated_user_provider_on_the_session_factory() {
        let source = generated(AUTHENTICATED_HANDSHAKE);

        assert!(source.contains(
            "structFactory{session_user_provider:::std::sync::Arc<super::super::authenticated_users::SessionUserProvider,>,}"
        ));
        assert!(source.contains(
            "Factory{session_user_provider:::std::sync::Arc::new(super::super::authenticated_users::SessionUserProvider{inner:container.session_user_provider().await?,}),}"
        ));
    }

    #[test]
    fn infers_the_authenticated_user_during_the_handshake() {
        let source = generated(AUTHENTICATED_HANDSHAKE);

        assert!(source.contains(
            "margaret::framework::identity::optional_authenticated_user::optional_authenticated_user(margaret::framework::identity::infers_authenticated_user::InfersAuthenticatedUser::infer(self.session_user_provider.as_ref(),handshake,).await,)"
        ));
        assert!(source.contains("Err(response)=>return::std::result::Result::Err(response)"));
    }

    #[test]
    fn builds_the_session_through_the_method_the_session_declares() {
        assert!(generated(AUTHENTICATED_HANDSHAKE).contains("crate::Room::assemble(viewer)"));
    }

    #[test]
    fn interrupts_the_handshake_with_a_continuation() {
        let source = generated(AUTHENTICATED_HANDSHAKE);

        assert!(source.contains(
            "->::std::result::Result<::std::sync::Arc<Self::Session>,margaret::framework::http::response_continuation::ResponseContinuation,>"
        ));
    }

    #[test]
    fn hands_the_routes_to_a_handshake_authenticated_user_provider() {
        let source = generated(
            r#"
use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret::framework::websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;

struct User;

#[singleton]
#[infers_authenticated_user(user_model = User)]
struct SessionUserProvider;

impl SessionUserProvider {
    #[infer_from_request]
    fn infer(&self, routes: &crate::margaret::routes::Routes) -> AuthenticatedUserOutcome<User> {}
}

#[websocket_session(path = "/room", server = "public")]
struct Room;

impl Room {
    #[build_for_session]
    fn build(#[authenticated_user] viewer: User) -> Self {}
}

#[websocket_message(request, method = "chat", response = single)]
struct Chat;

#[singleton]
struct Chatter;

impl RespondsToWebSocketMessage for Chatter {
    type Session = Room;
    type Message = Chat;
}
"#,
        );

        assert!(source.contains("routes:routes.clone(),"));
    }

    #[test]
    fn keeps_a_captured_provider_clear_of_a_session_parameter_that_takes_its_name() {
        let source = generated(
            r#"
use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret::framework::websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;

#[singleton]
struct SystemClock;

impl SystemClock {
    #[constructor]
    fn create() -> Self {}
}

struct User;

#[singleton]
#[infers_authenticated_user(user_model = User)]
struct Session;

impl Session {
    #[infer_from_request]
    fn infer(&self) -> AuthenticatedUserOutcome<User> {}
}

#[websocket_session(path = "/room", server = "public")]
struct Room;

impl Room {
    #[build_for_session]
    fn build(session: std::sync::Arc<SystemClock>, #[authenticated_user] viewer: User) -> Self {}
}

#[websocket_message(request, method = "chat", response = single)]
struct Chat;

#[singleton]
struct Chatter;

impl RespondsToWebSocketMessage for Chatter {
    type Session = Room;
    type Message = Chat;
}
"#,
        );

        assert!(source.contains("session:::std::sync::Arc<crate::SystemClock>,"));
        assert!(
            source.contains(
                "session_2:::std::sync::Arc<super::super::authenticated_users::Session>,"
            )
        );
        assert!(source.contains("self.session_2.as_ref()"));
    }

    const CONSOLE_ARGUMENT_PROVIDER: &str = r#"
use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret::framework::websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;

struct User;

#[singleton]
#[infers_authenticated_user(user_model = User)]
struct SessionUserProvider;

impl SessionUserProvider {
    #[constructor]
    fn create(#[console_argument(from = "realm")] realm: String) -> Self {}

    #[infer_from_request]
    fn infer(&self) -> AuthenticatedUserOutcome<User> {}
}

#[websocket_session(path = "/room", server = "public")]
struct Room;

impl Room {
    #[build_for_session]
    fn build(#[authenticated_user] viewer: User) -> Self {}
}

#[websocket_message(request, method = "chat", response = single)]
struct Chat;

#[singleton]
struct Chatter;

impl RespondsToWebSocketMessage for Chatter {
    type Session = Room;
    type Message = Chat;
}
"#;

    #[test]
    fn threads_the_console_arguments_of_an_authenticated_user_provider_through_the_handshake() {
        let source = generated(CONSOLE_ARGUMENT_PROVIDER);

        assert!(source.contains("pubasyncfnupgrade_entry(container:&super::super::container::Container,console_argument_0:&str,)"));
        assert!(source.contains(
            "inner:container.session_user_provider(console_argument_0.to_owned()).await?,"
        ));
        assert!(source.contains(
            "pubasyncfnpublic_routes(container:&super::container::Container,console_argument_0:&str,_routes:"
        ));
        assert!(source.contains("upgrade_entry(container,console_argument_0).await"));
    }

    #[test]
    fn rejects_a_middleware_that_renders_the_views_on_a_session() {
        assert!(
            error(
                r#"
use margaret::framework::http::next::Next;

#[singleton]
#[handles_middleware_attribute(attribute = decorated)]
struct Decorator;

impl Decorator {
    #[process]
    fn process(&self, views: &crate::margaret::views::Views, next: Next) -> ResponseContinuation {}
}

#[websocket_session(path = "/room", server = "public")]
#[middleware(decorated)]
struct Room;

impl Room {
    #[build_for_session]
    fn build() -> Self {}
}
"#
            )
            .to_string()
            .contains("which renders the views, but a WebSocket upgrade handshake has no views")
        );
    }

    #[test]
    fn rejects_an_authenticated_user_whose_provider_renders_the_views() {
        assert!(
            error(
                r#"
use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;

struct User;

#[singleton]
#[infers_authenticated_user(user_model = User)]
struct SessionUserProvider;

impl SessionUserProvider {
    #[infer_from_request]
    fn infer(&self, views: &crate::margaret::views::Views) -> AuthenticatedUserOutcome<User> {}
}

#[websocket_session(path = "/room", server = "public")]
struct Room;

impl Room {
    #[build_for_session]
    fn build(#[authenticated_user] viewer: User) -> Self {}
}
"#
            )
            .to_string()
            .contains("but a WebSocket upgrade handshake has no views")
        );
    }

    #[test]
    fn rejects_the_views_in_a_session_builder() {
        assert!(
            error(
                r#"
#[websocket_session(path = "/x", server = "public")]
struct Bad;

impl Bad {
    #[build_for_session]
    fn build(views: &crate::margaret::views::Views) -> Self {}
}
"#
            )
            .to_string()
            .contains("render them from an HTTP responder instead")
        );
    }

    #[test]
    fn rejects_an_authenticated_user_whose_provider_reads_the_request_body() {
        assert!(
            error(
                r#"
use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;

struct User;

struct Credentials;

#[singleton]
#[infers_authenticated_user(user_model = User)]
struct SessionUserProvider;

impl SessionUserProvider {
    #[infer_from_request]
    fn infer(&self, #[form_request(from = Json)] credentials: Credentials) -> AuthenticatedUserOutcome<User> {}
}

#[websocket_session(path = "/room", server = "public")]
struct Room;

impl Room {
    #[build_for_session]
    fn build(#[authenticated_user] viewer: User) -> Self {}
}
"#
            )
            .to_string()
            .contains("but a WebSocket upgrade handshake has no body")
        );
    }

    const PARITY_SESSION: &str = r#"
use std::sync::Arc;
use margaret::framework::websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;

#[singleton]
struct Greeter;

impl Greeter {
    #[constructor]
    fn new() -> Self {}
}

struct Article;

#[singleton]
#[provides_route_parameter]
struct ArticleStore;

impl ArticleStore {
    #[constructor]
    fn new() -> Self {}
}

impl margaret::framework::http::http_route_parameter_binder::HttpRouteParameterBinder for ArticleStore {
    type Model = Article;
}

struct Filters;

struct Preferences;

#[websocket_session(path = "/board/{topic}/{article}", server = "public")]
struct BoardSession;

impl BoardSession {
    #[build_for_session]
    fn build_for_session(
        greeter: Arc<Greeter>,
        #[route_parameter(from = "topic")] topic: String,
        #[route_parameter(from = "article")] article: Article,
        #[form_request(from = Query)] filters: Filters,
        #[form_request(from = Cookie)] preferences: Preferences,
        request: &margaret::framework::http::request::Request,
        peer: &spiffe::spiffe_id::SpiffeId,
        routes: &crate::margaret::routes::Routes,
        assets: margaret::framework::asset_bag::asset_bag::AssetBag,
    ) -> Self {}
}

#[websocket_message(request, method = "post", response = single)]
struct Post;

#[singleton]
struct Poster;

impl Poster {
    #[constructor]
    fn new() -> Self {}
}

impl RespondsToWebSocketMessage for Poster {
    type Session = BoardSession;
    type Message = Post;
}
"#;

    #[test]
    fn generates_a_session_factory_for_every_supported_binding() {
        let source = generated(PARITY_SESSION);

        assert!(source.contains("require_bound_route_parameter::require_bound_route_parameter"));
        assert!(source.contains("RequestInput::Query"));
        assert!(source.contains("RequestInput::Cookie"));
        assert!(source.contains("require_peer_spiffe_id::require_peer_spiffe_id"));
        assert!(source.contains("::margaret::framework::asset_bag::asset_bag::AssetBag::new()"));
        assert!(source.contains("self.routes.as_ref()"));
        assert!(source.contains("routes:&::std::sync::Arc<super::super::routes::Routes>"));
        assert!(source.contains("upgrade_entry(container,routes)"));
        assert!(source.contains("routes:&::std::sync::Arc<super::routes::Routes>"));
        assert!(!source.contains("views"));
    }

    #[test]
    fn rejects_a_form_body_request_in_a_session() {
        assert!(
            error(
                r#"
#[websocket_session(path = "/x", server = "public")]
struct Bad;

struct Form;

impl Bad {
    #[build_for_session]
    fn build(#[form_request(from = Form)] form: Form) -> Self {}
}
"#
            )
            .to_string()
            .contains("handshake has no body")
        );
    }

    #[test]
    fn rejects_a_json_body_request_in_a_session() {
        assert!(
            error(
                r#"
#[websocket_session(path = "/x", server = "public")]
struct Bad;

struct Payload;

impl Bad {
    #[build_for_session]
    fn build(#[form_request(from = Json)] payload: Payload) -> Self {}
}
"#
            )
            .to_string()
            .contains("handshake has no body")
        );
    }

    #[test]
    fn rejects_a_forwarder_in_a_session() {
        assert!(
            error(
                r#"
#[websocket_session(path = "/x", server = "public")]
struct Bad;

impl Bad {
    #[build_for_session]
    fn build(forward: crate::margaret::forwarders::public::Forwarder) -> Self {}
}
"#
            )
            .to_string()
            .contains("cannot forward a request")
        );
    }

    #[test]
    fn rejects_a_bound_route_parameter_without_a_binder() {
        assert!(
            error(
                r#"
#[websocket_session(path = "/x/{item}", server = "public")]
struct Bad;

struct Widget;

impl Bad {
    #[build_for_session]
    fn build(#[route_parameter(from = "item")] item: Widget) -> Self {}
}
"#
            )
            .to_string()
            .contains("no #[provides_route_parameter]")
        );
    }

    #[test]
    fn propagates_a_route_parameter_binder_error() {
        assert!(
            error(
                r#"
#[provides_route_parameter]
enum Bad {}

#[websocket_session(path = "/x", server = "public")]
struct Room;

impl Room {
    #[build_for_session]
    fn build() -> Self {}
}
"#
            )
            .to_string()
            .contains("is only supported on structs")
        );
    }

    #[test]
    fn rejects_a_handler_that_is_not_a_singleton() {
        let source = format!(
            "{REQUEST_TRAIT}\n#[websocket_message(request, method = \"m\", response = single)]\nstruct M;\n\nstruct Handler;\n\nimpl RespondsToWebSocketMessage for Handler {{\n    type Session = Missing;\n    type Message = M;\n}}\n"
        );

        assert!(error(&source).to_string().contains("is not a #[singleton]"));
    }

    #[test]
    fn rejects_a_handler_missing_an_associated_type() {
        let source = format!(
            "{REQUEST_TRAIT}\n#[websocket_message(request, method = \"m\", response = single)]\nstruct M;\n\n#[singleton]\nstruct Handler;\n\nimpl RespondsToWebSocketMessage for Handler {{\n    type Message = M;\n}}\n"
        );

        assert!(error(&source).to_string().contains("associated type"));
    }

    #[test]
    fn rejects_a_handler_whose_session_is_not_a_session() {
        let source = format!(
            "{REQUEST_TRAIT}\n#[websocket_message(request, method = \"m\", response = single)]\nstruct M;\n\nstruct NotASession;\n\n#[singleton]\nstruct Handler;\n\nimpl RespondsToWebSocketMessage for Handler {{\n    type Session = NotASession;\n    type Message = M;\n}}\n"
        );

        assert!(
            error(&source)
                .to_string()
                .contains("does not resolve to a #[websocket_session]")
        );
    }

    #[test]
    fn rejects_a_handler_whose_message_is_not_a_message() {
        let source = format!(
            "{REQUEST_TRAIT}\n#[websocket_session(path = \"/x\", server = \"public\")]\nstruct S;\n\nimpl S {{\n    #[build_for_session]\n    fn build() -> Self {{}}\n}}\n\nstruct NotAMessage;\n\n#[singleton]\nstruct Handler;\n\nimpl RespondsToWebSocketMessage for Handler {{\n    type Session = S;\n    type Message = NotAMessage;\n}}\n"
        );

        assert!(
            error(&source)
                .to_string()
                .contains("does not resolve to a #[websocket_message]")
        );
    }

    #[test]
    fn rejects_a_handler_whose_message_is_a_response() {
        let source = format!(
            "{REQUEST_TRAIT}\n#[websocket_session(path = \"/x\", server = \"public\")]\nstruct S;\n\nimpl S {{\n    #[build_for_session]\n    fn build() -> Self {{}}\n}}\n\n#[websocket_message(response, method = \"r\")]\nstruct R;\n\n#[singleton]\nstruct Handler;\n\nimpl RespondsToWebSocketMessage for Handler {{\n    type Session = S;\n    type Message = R;\n}}\n"
        );

        assert!(
            error(&source)
                .to_string()
                .contains("handles a response message")
        );
    }

    #[test]
    fn rejects_a_request_handler_bound_to_a_notification() {
        let source = format!(
            "{REQUEST_TRAIT}\n#[websocket_session(path = \"/x\", server = \"public\")]\nstruct S;\n\nimpl S {{\n    #[build_for_session]\n    fn build() -> Self {{}}\n}}\n\n#[websocket_message(notification, method = \"n\")]\nstruct N;\n\n#[singleton]\nstruct Handler;\n\nimpl RespondsToWebSocketMessage for Handler {{\n    type Session = S;\n    type Message = N;\n}}\n"
        );

        assert!(
            error(&source)
                .to_string()
                .contains("is not a request message")
        );
    }

    #[test]
    fn rejects_a_notification_handler_bound_to_a_request() {
        let source = format!(
            "{NOTIFICATION_TRAIT}\n#[websocket_session(path = \"/x\", server = \"public\")]\nstruct S;\n\nimpl S {{\n    #[build_for_session]\n    fn build() -> Self {{}}\n}}\n\n#[websocket_message(request, method = \"r\", response = single)]\nstruct R;\n\n#[singleton]\nstruct Handler;\n\nimpl RespondsToWebSocketNotification for Handler {{\n    type Session = S;\n    type Message = R;\n}}\n"
        );

        assert!(
            error(&source)
                .to_string()
                .contains("is not a notification message")
        );
    }

    const REUSABLE_MESSAGES: &str = r#"
#[websocket_session(path = "/first", server = "public")]
struct FirstSession;

impl FirstSession {
    #[build_for_session]
    fn build() -> Self {}
}

#[websocket_session(path = "/second", server = "public")]
struct SecondSession;

impl SecondSession {
    #[build_for_session]
    fn build() -> Self {}
}

#[websocket_message(request, method = "shared_request", response = single)]
struct SharedRequest;

#[websocket_message(notification, method = "shared_notification")]
struct SharedNotification;

mod first_request {
    #[singleton]
    pub struct Handler;

    impl margaret::framework::websocket::responds_to_web_socket_message::RespondsToWebSocketMessage for Handler {
        type Session = crate::FirstSession;
        type Message = crate::SharedRequest;
    }
}

mod second_request {
    #[singleton]
    pub struct Handler;

    impl margaret::framework::websocket::responds_to_web_socket_message::RespondsToWebSocketMessage for Handler {
        type Session = crate::SecondSession;
        type Message = crate::SharedRequest;
    }
}

mod first_notification {
    #[singleton]
    pub struct Handler;

    impl margaret::framework::websocket::responds_to_web_socket_notification::RespondsToWebSocketNotification for Handler {
        type Session = crate::FirstSession;
        type Message = crate::SharedNotification;
    }
}

mod second_notification {
    #[singleton]
    pub struct Handler;

    impl margaret::framework::websocket::responds_to_web_socket_notification::RespondsToWebSocketNotification for Handler {
        type Session = crate::SecondSession;
        type Message = crate::SharedNotification;
    }
}
"#;

    #[test]
    fn reuses_request_and_notification_messages_across_sessions() {
        let source = generated(REUSABLE_MESSAGES);

        assert!(source.contains("crate::first_request::Handler"));
        assert!(source.contains("crate::second_request::Handler"));
        assert!(source.contains("crate::first_notification::Handler"));
        assert!(source.contains("crate::second_notification::Handler"));
        assert_eq!(source.matches("\"shared_request\".to_string()").count(), 2);
        assert_eq!(
            source
                .matches("\"shared_notification\".to_string()")
                .count(),
            2
        );
        assert_eq!(
            source
                .matches("WebSocketRequestMessageforcrate::SharedRequest")
                .count(),
            1
        );
    }

    #[test]
    fn rejects_two_handlers_for_the_same_message() {
        let source = format!(
            "{REQUEST_TRAIT}\n#[websocket_session(path = \"/x\", server = \"public\")]\nstruct S;\n\nimpl S {{\n    #[build_for_session]\n    fn build() -> Self {{}}\n}}\n\n#[websocket_message(request, method = \"m\", response = single)]\nstruct M;\n\n#[singleton]\nstruct First;\n\nimpl RespondsToWebSocketMessage for First {{\n    type Session = S;\n    type Message = M;\n}}\n\n#[singleton]\nstruct Second;\n\nimpl RespondsToWebSocketMessage for Second {{\n    type Session = S;\n    type Message = M;\n}}\n"
        );

        assert!(
            error(&source)
                .to_string()
                .contains("handled by more than one handler")
        );
    }

    #[test]
    fn rejects_a_message_without_a_handler() {
        let source =
            "#[websocket_message(request, method = \"m\", response = single)]\nstruct M;\n";

        assert!(error(source).to_string().contains("has no handler"));
    }

    #[test]
    fn detects_the_presence_of_websocket_sessions() {
        let index = index_for(r#"#[websocket_session(path = "/x", server = "public")] struct S;"#);

        assert!(has_websocket_sessions(&index));
    }

    #[test]
    fn detects_the_absence_of_websocket_sessions() {
        assert!(!has_websocket_sessions(&index_for("struct Plain;")));
    }

    #[test]
    fn generates_a_module_declaration_for_each_session() {
        let source = generated(
            r#"
#[websocket_session(path = "/a", server = "public")]
struct AlphaSession;

impl AlphaSession {
    #[build_for_session]
    fn build() -> Self {}
}

#[websocket_session(path = "/b", server = "public")]
struct BetaSession;

impl BetaSession {
    #[build_for_session]
    fn build() -> Self {}
}
"#,
        );

        assert!(source.contains("pubmodalpha_session"));
        assert!(source.contains("pubmodbeta_session"));
    }

    #[test]
    fn rejects_a_build_for_session_that_returns_a_non_self_type() {
        assert!(
            error(
                r#"
#[websocket_session(path = "/x", server = "public")]
struct Bad;

impl Bad {
    #[build_for_session]
    fn build() -> u8 {}
}
"#
            )
            .to_string()
            .contains("must return Self")
        );
    }

    #[test]
    fn propagates_a_non_path_cardinality() {
        assert!(
            error(r#"#[websocket_message(request, method = "m", response = 5)] struct Bad;"#)
                .to_string()
                .contains("failed to read")
        );
    }

    #[test]
    fn propagates_a_non_string_request_method() {
        assert!(
            error(r#"#[websocket_message(request, method = 5, response = single)] struct Bad;"#)
                .to_string()
                .contains("failed to read")
        );
    }

    #[test]
    fn rejects_a_notification_without_a_method() {
        assert!(
            error(r#"#[websocket_message(notification)] struct Bad;"#)
                .to_string()
                .contains("missing the required 'method'")
        );
    }

    #[test]
    fn rejects_a_response_that_declares_a_cardinality() {
        assert!(
            error(r#"#[websocket_message(response, response = single)] struct Bad;"#)
                .to_string()
                .contains("declares a response cardinality but is not a request")
        );
    }

    #[test]
    fn propagates_a_non_string_response_method() {
        assert!(
            error(r#"#[websocket_message(response, method = 5)] struct Bad;"#)
                .to_string()
                .contains("failed to read")
        );
    }

    #[test]
    fn propagates_malformed_message_arguments() {
        assert!(
            error(r#"#[websocket_message(= 5)] struct Bad;"#)
                .to_string()
                .contains("failed to read")
        );
    }

    #[test]
    fn propagates_malformed_session_arguments() {
        assert!(
            error(r#"#[websocket_session(= 5)] struct Bad;"#)
                .to_string()
                .contains("failed to read")
        );
    }

    #[test]
    fn propagates_a_non_string_session_path() {
        assert!(
            error(r#"#[websocket_session(path = 5, server = "public")] struct Bad;"#)
                .to_string()
                .contains("failed to read")
        );
    }

    #[test]
    fn propagates_a_non_string_session_server() {
        assert!(
            error(r#"#[websocket_session(path = "/x", server = 5)] struct Bad;"#)
                .to_string()
                .contains("failed to read")
        );
    }

    #[test]
    fn propagates_malformed_route_parameter_arguments() {
        assert!(
            error(
                r#"
#[websocket_session(path = "/x/{id}", server = "public")]
struct Bad;

impl Bad {
    #[build_for_session]
    fn build(#[route_parameter(= 5)] id: String) -> Self {}
}
"#
            )
            .to_string()
            .contains("failed to read")
        );
    }

    #[test]
    fn propagates_a_non_string_route_parameter_from() {
        assert!(
            error(
                r#"
#[websocket_session(path = "/x/{id}", server = "public")]
struct Bad;

impl Bad {
    #[build_for_session]
    fn build(#[route_parameter(from = 5)] id: String) -> Self {}
}
"#
            )
            .to_string()
            .contains("failed to read")
        );
    }

    #[test]
    fn rejects_a_handler_missing_the_message_associated_type() {
        let source = format!(
            "{REQUEST_TRAIT}\n#[websocket_session(path = \"/x\", server = \"public\")]\nstruct S;\n\nimpl S {{\n    #[build_for_session]\n    fn build() -> Self {{}}\n}}\n\n#[singleton]\nstruct Handler;\n\nimpl RespondsToWebSocketMessage for Handler {{\n    type Session = S;\n}}\n"
        );

        assert!(error(&source).to_string().contains("associated type"));
    }

    #[test]
    fn ignores_a_trait_impl_whose_trait_does_not_resolve() {
        assert!(generated("struct Widget;\nimpl Unresolved for Widget {}\n").is_empty());
    }

    #[test]
    fn rejects_two_methods_that_collide_in_a_session() {
        let source = format!(
            "{REQUEST_TRAIT}\n#[websocket_session(path = \"/x\", server = \"public\")]\nstruct S;\n\nimpl S {{\n    #[build_for_session]\n    fn build() -> Self {{}}\n}}\n\n#[websocket_message(request, method = \"m\", response = single)]\nstruct First;\n\n#[websocket_message(request, method = \"m\", response = single)]\nstruct Second;\n\n#[singleton]\nstruct HandlerOne;\n\nimpl RespondsToWebSocketMessage for HandlerOne {{\n    type Session = S;\n    type Message = First;\n}}\n\n#[singleton]\nstruct HandlerTwo;\n\nimpl RespondsToWebSocketMessage for HandlerTwo {{\n    type Session = S;\n    type Message = Second;\n}}\n"
        );

        assert!(error(&source).to_string().contains("more than once"));
    }
}
