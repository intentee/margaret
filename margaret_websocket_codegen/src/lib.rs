pub mod render_websocket;
pub mod web_socket_artifacts;
pub mod web_socket_codegen_error;
pub mod web_socket_plan;

mod build_for_session_method;
mod build_websocket_plan;
mod built_websocket_plan;
mod discovered_handler;
mod handler_binding;
mod handler_kind;
mod handler_serve_inputs;
mod message_cardinality;
mod message_kind;
mod render_messages;
mod render_server_routes;
mod render_sessions;
mod server_serve_inputs;
mod session_arguments;
mod session_handler_plan;
mod session_plan;
mod session_serve_inputs;
mod web_socket_message;
mod web_socket_session;
mod websocket_handlers;
mod websocket_messages;
mod websocket_sessions;

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use quote::format_ident;

    use margaret_attributes::attribute_index::AttributeIndex;
    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_attributes::tag::Tag;
    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_container::container_bindings::ContainerBindings;
    use margaret_container::framework_construction::FrameworkConstruction;
    use margaret_container::framework_enablement::FrameworkEnablement;
    use margaret_container::framework_injection_role::FrameworkInjectionRole;
    use margaret_container::framework_provider::FrameworkProvider;
    use margaret_container::render_container::render_container;
    use margaret_http_codegen::server_transport_policy::ServerTransportPolicy;
    use margaret_middleware_codegen::layer_application::LayerApplication;
    use margaret_middleware_codegen::middleware_plans::MiddlewarePlans;
    use margaret_request_binding_codegen::binding_registries::BindingRegistries;
    use margaret_request_binding_codegen::bound_parameter::BoundParameter;
    use margaret_request_binding_codegen::request_binding::RequestBinding;
    use margaret_request_binding_codegen::request_binding_error::RequestBindingError;
    use margaret_request_binding_codegen::views_availability::ViewsAvailability;
    use margaret_serve_input_codegen::scan::scan;
    use margaret_tag_codegen::tag_pool::TagPool;

    use crate::handler_binding::HandlerBinding;
    use crate::render_websocket;
    use crate::server_serve_inputs::server_serve_inputs;
    use crate::session_plan::SessionPlan;
    use crate::session_serve_inputs::session_serve_inputs;
    use crate::web_socket_artifacts::WebSocketArtifacts;
    use crate::web_socket_codegen_error::WebSocketCodegenError;
    use crate::web_socket_plan::WebSocketPlan;
    use crate::web_socket_session::WebSocketSession;
    use crate::websocket_handlers::websocket_handlers;

    fn render_websocket(
        index: &AttributeIndex,
        bindings: &ContainerBindings,
        middleware_plans: &MiddlewarePlans,
        registries: &BindingRegistries,
    ) -> Result<WebSocketArtifacts, WebSocketCodegenError> {
        WebSocketPlan::build(index, bindings, middleware_plans, registries)
            .map(|plan| render_websocket::render_websocket(plan, bindings))
    }

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
    fn new() -> anyhow::Result<Self> {}
}

#[singleton]
struct Config;

impl Config {
    #[constructor]
    fn new() -> anyhow::Result<Self> {}
}

#[singleton]
struct LogPlugin;

impl LogPlugin {
    #[constructor]
    fn new() -> anyhow::Result<Self> {}
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
    ) -> anyhow::Result<Self> {}
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

    fn bindings(index: &AttributeIndex) -> ContainerBindings {
        let registry = scan(index).expect("the console arguments are scanned");

        render_container(index, &registry, &[])
            .expect("the container renders")
            .bindings
    }

    fn collect_registries(
        index: &AttributeIndex,
    ) -> Result<BindingRegistries, RequestBindingError> {
        BindingRegistries::collect(
            index,
            ViewsAvailability::Available,
            &TagPool::collect(index).expect("the tags are collected"),
            &bindings(&IndexedSource::new("").index),
        )
    }

    fn registries_for(index: &AttributeIndex) -> BindingRegistries {
        collect_registries(index).expect("the binding registries are collected")
    }

    fn missing_path() -> CanonicalPath {
        CanonicalPath::new(vec!["crate".to_string(), "Missing".to_string()])
    }

    fn empty_session_plan() -> SessionPlan {
        SessionPlan {
            notification_handlers: Vec::new(),
            request_handlers: Vec::new(),
            session: WebSocketSession {
                layers: Vec::new(),
                method_name: format_ident!("build"),
                module_name: "room".to_string(),
                parameters: Vec::new(),
                path: "/room".to_string(),
                server: "public".to_string(),
                session_path: CanonicalPath::new(vec!["crate".to_string(), "Room".to_string()]),
            },
        }
    }

    fn missing_handler() -> HandlerBinding {
        HandlerBinding {
            dispatch_ident: "MissingDispatch".to_string(),
            handler_field: "missing".to_string(),
            handler_path: missing_path(),
            method: "missing".to_string(),
        }
    }

    fn generated(source: &str) -> String {
        let index = IndexedSource::new(source).index;
        let registries = registries_for(&index);
        let tags = TagPool::collect(&index).expect("the tags are collected");
        let plans = MiddlewarePlans::collect(&index, &registries, &tags)
            .expect("the middleware plans are collected");

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
            .collect::<String>()
    }

    fn error(source: &str) -> WebSocketCodegenError {
        let index = IndexedSource::new(source).index;
        let registries = match collect_registries(&index) {
            Ok(registries) => registries,
            Err(rejection) => return WebSocketCodegenError::from(rejection),
        };
        let tags = TagPool::collect(&index).expect("the tags are collected");
        let plans = MiddlewarePlans::collect(&index, &registries, &tags)
            .expect("the middleware plans are collected");

        render_websocket(&index, &bindings(&index), &plans, &registries)
            .expect_err("the websocket module is rejected")
    }

    const CONSOLE_ARGUMENT_HANDLER: &str = r#"
use margaret::framework::websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;

#[websocket_session(path = "/room", server = "public")]
struct Room;

impl Room {
    #[build_for_session]
    fn build() -> anyhow::Result<Self> {}
}

#[websocket_message(request, method = "chat", response = single)]
struct Chat;

#[singleton]
struct Chatter;

impl Chatter {
    #[constructor]
    fn create(#[console_argument(from = "greeting")] greeting: String) -> anyhow::Result<Self> {}
}

impl RespondsToWebSocketMessage for Chatter {
    type Session = Room;
    type Message = Chat;
}
"#;

    #[test]
    fn reads_a_preconstructed_websocket_handler_through_the_dispatch_chain() {
        let source = generated(CONSOLE_ARGUMENT_HANDLER);

        assert!(source.contains("container.chatter()"));
        assert!(source.contains("dispatch_table(container)"));
        assert!(source.contains("public_routes(container:&super::super::container::Container,"));
        assert!(source.contains("upgrade_entry(container"));
        assert!(!source.contains("serve_input_"));
    }

    const CONSOLE_ARGUMENT_NOTIFICATION_HANDLER: &str = r#"
use margaret::framework::websocket::responds_to_web_socket_notification::RespondsToWebSocketNotification;

#[websocket_session(path = "/room", server = "public")]
struct Room;

impl Room {
    #[build_for_session]
    fn build() -> anyhow::Result<Self> {}
}

#[websocket_message(notification, method = "typing")]
struct Typing;

#[singleton]
struct Typist;

impl Typist {
    #[constructor]
    fn create(#[console_argument(from = "channel")] channel: String) -> anyhow::Result<Self> {}
}

impl RespondsToWebSocketNotification for Typist {
    type Session = Room;
    type Message = Typing;
}
"#;

    #[test]
    fn reads_a_preconstructed_notification_handler_through_the_dispatch_chain() {
        let source = generated(CONSOLE_ARGUMENT_NOTIFICATION_HANDLER);

        assert!(source.contains("container.typist()"));
        assert!(source.contains("dispatch_table(container)"));
        assert!(!source.contains("serve_input_"));
    }

    #[test]
    fn reports_a_notification_handler_absent_from_the_container_plan() {
        let index = IndexedSource::new("#[singleton]\nstruct Known;\n").index;
        let mut plan = empty_session_plan();
        plan.notification_handlers.push(missing_handler());

        let error = session_serve_inputs(&plan, &bindings(&index))
            .expect_err("an unplanned handler has no console argument closure");

        assert!(error.to_string().contains("crate::Missing"));
    }

    #[test]
    fn reports_a_request_handler_absent_from_the_container_plan() {
        let index = IndexedSource::new("#[singleton]\nstruct Known;\n").index;
        let mut plan = empty_session_plan();
        plan.request_handlers.push(missing_handler());

        let error = session_serve_inputs(&plan, &bindings(&index))
            .expect_err("an unplanned handler has no console arguments");

        assert!(error.to_string().contains("crate::Missing"));

        let error = server_serve_inputs(&[&plan], &bindings(&index))
            .expect_err("a server cannot include an unplanned handler");

        assert!(error.to_string().contains("crate::Missing"));
    }

    #[test]
    fn reports_a_session_binding_absent_from_the_container_plan() {
        let index = IndexedSource::new("#[singleton]\nstruct Known;\n").index;
        let mut plan = empty_session_plan();
        plan.session.parameters.push(BoundParameter {
            binding: RequestBinding::BoundRouteParameter {
                binder_field: "missing".to_string(),
                binder_provider: missing_path(),
                path_key: "room".to_string(),
            },
            holder: format_ident!("room"),
        });

        let error = session_serve_inputs(&plan, &bindings(&index))
            .expect_err("an unplanned binding has no console arguments");

        assert!(error.to_string().contains("crate::Missing"));
    }

    #[test]
    fn reports_a_session_layer_absent_from_the_container_plan() {
        let index = IndexedSource::new("#[singleton]\nstruct Known;\n").index;
        let mut plan = empty_session_plan();
        plan.session.layers.push(LayerApplication {
            concrete: missing_path(),
            field: format_ident!("missing"),
            injects_peer_spiffe_id: false,
            injects_routes: false,
            injects_views: false,
            wrapper: format_ident!("Missing"),
        });

        let error = server_serve_inputs(&[&plan], &bindings(&index))
            .expect_err("an unplanned middleware layer has no console arguments");

        assert!(error.to_string().contains("crate::Missing"));
    }

    #[test]
    fn propagates_a_container_mismatch_from_websocket_rendering() {
        let index = IndexedSource::new(
            r#"
use margaret::framework::websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;

#[websocket_session(path = "/room", server = "public")]
struct Room;

impl Room {
    #[build_for_session]
    fn build() -> anyhow::Result<Self> {}
}

#[websocket_message(request, method = "ping", response = single)]
struct Ping;

#[singleton]
struct Ponger;

impl RespondsToWebSocketMessage for Ponger {
    type Session = Room;
    type Message = Ping;
}
"#,
        )
        .index;
        let registries = registries_for(&index);
        let tags = TagPool::collect(&index).expect("the tags are collected");
        let plans = MiddlewarePlans::collect(&index, &registries, &tags)
            .expect("the middleware plans are collected");
        let empty_index = IndexedSource::new("").index;
        let error = render_websocket(&index, &bindings(&empty_index), &plans, &registries)
            .map(drop)
            .expect_err("websocket rendering requires the same container plan");

        let message = error.to_string();

        assert!(message.contains("crate::"));
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
        let error = websocket_handlers(&IndexedSource::new(NON_STRUCT_HANDLER).index)
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
            fn new() -> anyhow::Result<Self> {}
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
        let handlers = websocket_handlers(&IndexedSource::new(COLLIDING_HANDLER_FIELD).index)
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
    fn build() -> anyhow::Result<Self> {}
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
    fn build() -> anyhow::Result<Self> {}
}

#[handles_middleware_attribute(attribute = guard)]
struct Guard;

impl Guard {
    #[process]
    fn process(&self, request: &Request, next: Next) -> anyhow::Result<ResponseContinuation> {}
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
    fn build() -> anyhow::Result<Self> {}
}

#[handles_middleware_attribute(attribute = traced)]
struct Tracer;

impl Tracer {
    #[process]
    fn process(&self, request: &Request, next: Next, routes: &Routes) -> anyhow::Result<ResponseContinuation> {}
}
"#;

    const INJECTED_ONLY_SESSION: &str = r#"
use std::sync::Arc;

#[singleton]
struct SystemClock;

impl SystemClock {
    #[constructor]
    fn new() -> anyhow::Result<Self> {}
}

#[websocket_session(path = "/room", server = "public")]
struct Room;

impl Room {
    #[build_for_session]
    fn build_for_session(clock: Arc<SystemClock>) -> anyhow::Result<Self> {}
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

        assert!(
            source.contains("dispatch_table(_container:&super::super::super::container::Container")
        );
    }

    #[test]
    fn names_the_dispatch_container_when_a_session_has_handlers() {
        let source = generated(FULL_SESSION);

        assert!(
            source.contains("dispatch_table(container:&super::super::super::container::Container")
        );
    }

    #[test]
    fn carries_its_middleware_on_the_route_entry() {
        let source = generated(SESSION_WITH_MIDDLEWARE);

        assert!(source.contains("margaret::framework::http::route_entry::RouteEntry::web_socket("));
        assert!(source.contains(
            "=[std::sync::Arc::new(super::super::middleware::Guard{inner:container.guard()"
        ));
        assert!(source.contains("::std::vec::Vec::from(middleware)"));
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
    fn build() -> anyhow::Result<Self> {}
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
    fn reads_a_preconstructed_session_middleware() {
        let source = generated(SESSION_WITH_CONSOLE_ARGUMENT_MIDDLEWARE);

        assert!(source.contains("public_routes(container:&super::super::container::Container,"));
        assert!(source.contains("container.guard()"));
        assert!(!source.contains("serve_input_"));
    }

    #[test]
    fn rejects_a_session_with_an_unknown_middleware_tag() {
        assert!(
            error(
                "#[websocket_session(path = \"/room\", server = \"public\")]\n#[middleware(missing)]\nstruct Room;\n\nimpl Room {\n    #[build_for_session]\n    fn build() -> anyhow::Result<Self> {}\n}\n"
            )
            .to_string()
            .contains("which no middleware handler declares")
        );
    }

    #[test]
    fn weaves_routes_when_only_a_middleware_injects_them() {
        let source = generated(SESSION_WITH_ROUTES_MIDDLEWARE);

        assert!(source.contains("routes:&::std::sync::Arc<super::super::routes::Routes>"));
        assert!(source.contains(
            "super::super::middleware::Tracer{inner:container.tracer(),routes:routes.clone()"
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
        assert!(source.contains("container.system_clock()"));
        assert!(source.contains("container.config()"));
        assert!(source.contains("container.log_plugin()"));
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
        assert!(source.contains("container.speaker()"));
    }

    #[test]
    fn generates_a_public_upgrade_entry() {
        let source = generated(FULL_SESSION);

        assert!(source.contains("pubfnupgrade_entry"));
        assert!(source.contains("web_socket_upgrade_entry::WebSocketUpgradeEntry::new"));
        assert!(source.contains("dispatch_table(container)"));
    }

    #[test]
    fn generates_a_route_registration_function_per_server() {
        let source = generated(FULL_SESSION);

        assert!(source.contains("pubfnpublic_routes"));
        assert!(source.contains("route_entry::RouteEntry::web_socket(\"/chat/{room}\""));
        assert!(source.contains("chat_session::upgrade_entry(container)"));
    }

    #[test]
    fn rejects_a_message_that_is_not_a_struct() {
        assert!(
            error(r"#[websocket_message(response)] enum Bad {}")
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
            error(r"#[websocket_message(request, response = single)] struct Bad;")
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
            error(r"#[websocket_message(response)] struct Bad;")
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
    fn first() -> anyhow::Result<Self> {}

    #[build_for_session]
    fn second() -> anyhow::Result<Self> {}
}
"#
            )
            .to_string()
            .contains("more than one #[build_for_session] method")
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
    fn build(#[route_parameter] id: String) -> anyhow::Result<Self> {}
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
    fn build(#[route_parameter(from = "id")] id: String) -> anyhow::Result<Self> {}
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
    fn build(value: String) -> anyhow::Result<Self> {}
}
"#
            )
            .to_string()
            .contains("is not an injectable dependency")
        );
    }

    #[test]
    fn rejects_a_session_parameter_that_injects_a_token_issuer_client_by_path() {
        let index = IndexedSource::new(
            r#"
use std::sync::Arc;

use crate::TrustedIssuer;

#[websocket_session(path = "/x", server = "public")]
struct Bad;

impl Bad {
    #[build_for_session]
    fn build(trusted_issuer: Arc<TrustedIssuer>) -> anyhow::Result<Self> {}
}
"#,
        )
        .index;
        let registries = registries_for(&index);
        let tags = TagPool::collect(&index).expect("the tags are collected");
        let plans = MiddlewarePlans::collect(&index, &registries, &tags)
            .expect("the middleware plans are collected");
        let registry = scan(&index).expect("the console arguments are scanned");
        let bindings = render_container(
            &index,
            &registry,
            &[FrameworkProvider {
                construction: FrameworkConstruction::Unit,
                enablement: FrameworkEnablement::Always,
                injection: FrameworkInjectionRole::TrustedIssuer(
                    Tag::from_path(&syn::parse_str("auth").expect("the tag path parses"))
                        .expect("the tag is a plain name"),
                ),
                provided: CanonicalPath::new(vec![
                    "crate".to_string(),
                    "TrustedIssuer".to_string(),
                ]),
            }],
        )
        .expect("the container renders")
        .bindings;

        assert!(
            render_websocket(&index, &bindings, &plans, &registries)
                .expect_err("a trusted issuer is not injectable by path")
                .to_string()
                .contains("which only the framework may inject")
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
    fn build(missing: Arc<Unknown>) -> anyhow::Result<Self> {}
}
"#
            )
            .to_string()
            .contains("no #[singleton] provides it")
        );
    }

    const AUTHENTICATED_HANDSHAKE: &str = r#"use margaret::framework::http_validation::request_input::RequestInput;


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
    fn infer(&self, request: &Request, #[form_request(from = RequestInput::Cookie)] cookie: SessionCookie) -> anyhow::Result<AuthenticatedUserOutcome<User>> {}
}

#[websocket_session(path = "/room", server = "public")]
struct Room;

impl Room {
    #[build_for_session]
    fn assemble(#[authenticated_user] viewer: Option<User>) -> anyhow::Result<Self> {}
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
            "structFactory{session_user_provider:::std::sync::Arc<super::super::super::authenticated_users::SessionUserProvider,>,}"
        ));
        assert!(source.contains(
            "Factory{session_user_provider:::std::sync::Arc::new(super::super::super::authenticated_users::SessionUserProvider{inner:container.session_user_provider(),}),}"
        ));
    }

    #[test]
    fn infers_the_authenticated_user_during_the_handshake() {
        let source = generated(AUTHENTICATED_HANDSHAKE);

        assert!(source.contains(
            "margaret::framework::identity::infers_authenticated_user::InfersAuthenticatedUser::infer(self.session_user_provider.as_ref(),handshake,).await"
        ));
        assert!(source.contains("::std::result::Result::Ok(outcome)"));
        assert!(source.contains(
            "margaret::framework::identity::optional_authenticated_user::optional_authenticated_user(outcome,)"
        ));
        assert!(source.contains("WebSocketSessionCreationOutcome::Interrupted(response,)"));
        assert!(source.contains("WebSocketSessionCreationError::consumer(error,)"));
    }

    #[test]
    fn builds_the_session_through_the_method_the_session_declares() {
        assert!(generated(AUTHENTICATED_HANDSHAKE).contains("crate::Room::assemble(viewer)"));
    }

    #[test]
    fn interrupts_the_handshake_with_a_continuation() {
        let source = generated(AUTHENTICATED_HANDSHAKE);

        assert!(source.contains("->::std::result::Result<"));
        assert!(source.contains(
            "margaret::framework::websocket_session::web_socket_session_creation_outcome::WebSocketSessionCreationOutcome<Self::Session,>"
        ));
        assert!(source.contains(
            "margaret::framework::websocket_session::web_socket_session_creation_error::WebSocketSessionCreationError"
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
    fn infer(&self, routes: &crate::margaret::routes::Routes) -> anyhow::Result<AuthenticatedUserOutcome<User>> {}
}

#[websocket_session(path = "/room", server = "public")]
struct Room;

impl Room {
    #[build_for_session]
    fn build(#[authenticated_user] viewer: User) -> anyhow::Result<Self> {}
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
    fn create() -> anyhow::Result<Self> {}
}

struct User;

#[singleton]
#[infers_authenticated_user(user_model = User)]
struct Session;

impl Session {
    #[infer_from_request]
    fn infer(&self) -> anyhow::Result<AuthenticatedUserOutcome<User>> {}
}

#[websocket_session(path = "/room", server = "public")]
struct Room;

impl Room {
    #[build_for_session]
    fn build(session: std::sync::Arc<SystemClock>, #[authenticated_user] viewer: User) -> anyhow::Result<Self> {}
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
        assert!(source.contains(
            "session_2:::std::sync::Arc<super::super::super::authenticated_users::Session>,"
        ));
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
    fn create(#[console_argument(from = "realm")] realm: String) -> anyhow::Result<Self> {}

    #[infer_from_request]
    fn infer(&self) -> anyhow::Result<AuthenticatedUserOutcome<User>> {}
}

#[websocket_session(path = "/room", server = "public")]
struct Room;

impl Room {
    #[build_for_session]
    fn build(#[authenticated_user] viewer: User) -> anyhow::Result<Self> {}
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
    fn threads_the_serve_inputs_of_an_authenticated_user_provider_through_the_handshake() {
        let source = generated(CONSOLE_ARGUMENT_PROVIDER);

        assert!(
            source.contains(
                "pubfnupgrade_entry(container:&super::super::super::container::Container,)"
            )
        );
        assert!(source.contains("inner:container.session_user_provider(),"));
        assert!(
            source.contains(
                "pubfnpublic_routes(container:&super::super::container::Container,_routes:"
            )
        );
        assert!(source.contains("upgrade_entry(container)"));
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
    fn process(&self, views: &crate::margaret::views::Views, next: Next) -> anyhow::Result<ResponseContinuation> {}
}

#[websocket_session(path = "/room", server = "public")]
#[middleware(decorated)]
struct Room;

impl Room {
    #[build_for_session]
    fn build() -> anyhow::Result<Self> {}
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
    fn infer(&self, views: &crate::margaret::views::Views) -> anyhow::Result<AuthenticatedUserOutcome<User>> {}
}

#[websocket_session(path = "/room", server = "public")]
struct Room;

impl Room {
    #[build_for_session]
    fn build(#[authenticated_user] viewer: User) -> anyhow::Result<Self> {}
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
    fn build(views: &crate::margaret::views::Views) -> anyhow::Result<Self> {}
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
                r#"use margaret::framework::http_validation::request_input::RequestInput;


use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;

struct User;

struct Credentials;

#[singleton]
#[infers_authenticated_user(user_model = User)]
struct SessionUserProvider;

impl SessionUserProvider {
    #[infer_from_request]
    fn infer(&self, #[form_request(from = RequestInput::Json)] credentials: Credentials) -> anyhow::Result<AuthenticatedUserOutcome<User>> {}
}

#[websocket_session(path = "/room", server = "public")]
struct Room;

impl Room {
    #[build_for_session]
    fn build(#[authenticated_user] viewer: User) -> anyhow::Result<Self> {}
}
"#
            )
            .to_string()
            .contains("which only an HTTP responder receives")
        );
    }

    const PARITY_SESSION: &str = r#"use margaret::framework::http_validation::request_input::RequestInput;


use std::sync::Arc;
use margaret::framework::websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;

#[singleton]
struct Greeter;

impl Greeter {
    #[constructor]
    fn new() -> anyhow::Result<Self> {}
}

struct Article;

#[singleton]
#[provides_route_parameter]
struct ArticleStore;

impl ArticleStore {
    #[constructor]
    fn new() -> anyhow::Result<Self> {}
}

impl margaret::framework::route_parameter_binding::http_route_parameter_binder::HttpRouteParameterBinder for ArticleStore {
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
        #[form_request(from = RequestInput::Query)] filters: Filters,
        #[form_request(from = RequestInput::Cookie)] preferences: Preferences,
        request: &margaret::framework::http::request::Request,
        peer: &spiffe::spiffe_id::SpiffeId,
        routes: &crate::margaret::routes::Routes,
        assets: margaret::framework::asset_bag::asset_bag::AssetBag,
    ) -> anyhow::Result<Self> {}
}

#[websocket_message(request, method = "post", response = single)]
struct Post;

#[singleton]
struct Poster;

impl Poster {
    #[constructor]
    fn new() -> anyhow::Result<Self> {}
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
        assert!(source.contains(".inputs.query"));
        assert!(source.contains(".inputs.cookies"));
        assert!(source.contains("require_peer_spiffe_id::require_peer_spiffe_id"));
        assert!(source.contains("::margaret::framework::asset_bag::asset_bag::AssetBag::new()"));
        assert!(source.contains("self.routes.as_ref()"));
        assert!(source.contains("routes:&::std::sync::Arc<super::super::routes::Routes>"));
        assert!(source.contains("upgrade_entry(container,routes)"));
        assert!(source.contains("routes:&::std::sync::Arc<super::super::super::routes::Routes>"));
        assert!(!source.contains("views"));
    }

    #[test]
    fn rejects_a_form_body_request_in_a_session() {
        assert!(
            error(
                r#"use margaret::framework::http_validation::request_input::RequestInput;


#[websocket_session(path = "/x", server = "public")]
struct Bad;

struct Form;

impl Bad {
    #[build_for_session]
    fn build(#[form_request(from = RequestInput::Form)] form: Form) -> anyhow::Result<Self> {}
}
"#
            )
            .to_string()
            .contains("which only an HTTP responder receives")
        );
    }

    #[test]
    fn rejects_a_json_body_request_in_a_session() {
        assert!(
            error(
                r#"use margaret::framework::http_validation::request_input::RequestInput;


#[websocket_session(path = "/x", server = "public")]
struct Bad;

struct Payload;

impl Bad {
    #[build_for_session]
    fn build(#[form_request(from = RequestInput::Json)] payload: Payload) -> anyhow::Result<Self> {}
}
"#
            )
            .to_string()
            .contains("which only an HTTP responder receives")
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
    fn build(forward: crate::margaret::forwarders::public::Forwarder) -> anyhow::Result<Self> {}
}
"#
            )
            .to_string()
            .contains("cannot forward a request")
        );
    }

    #[test]
    fn extracts_a_route_parameter_value_type_during_the_handshake() {
        let source = generated(
            r#"
#[route_parameter_value]
struct RoomSlug(String);

#[websocket_session(path = "/rooms/{room_slug}", server = "public")]
struct Room;

impl Room {
    #[build_for_session]
    fn build(#[route_parameter(from = "room_slug")] room_slug: RoomSlug) -> anyhow::Result<Self> {}
}
"#,
        );

        assert!(source.contains("require_route_parameter::require_route_parameter"));
        assert!(source.contains("\"room_slug\""));
        assert!(source.contains("crate::Room::build(room_slug)"));
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
    fn build(#[route_parameter(from = "item")] item: Widget) -> anyhow::Result<Self> {}
}
"#
            )
            .to_string()
            .contains("neither declared as a #[route_parameter_value] nor provided by a #[provides_route_parameter]")
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
    fn build() -> anyhow::Result<Self> {}
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
            "{REQUEST_TRAIT}\n#[websocket_session(path = \"/x\", server = \"public\")]\nstruct S;\n\nimpl S {{\n    #[build_for_session]\n    fn build() -> anyhow::Result<Self> {{}}\n}}\n\nstruct NotAMessage;\n\n#[singleton]\nstruct Handler;\n\nimpl RespondsToWebSocketMessage for Handler {{\n    type Session = S;\n    type Message = NotAMessage;\n}}\n"
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
            "{REQUEST_TRAIT}\n#[websocket_session(path = \"/x\", server = \"public\")]\nstruct S;\n\nimpl S {{\n    #[build_for_session]\n    fn build() -> anyhow::Result<Self> {{}}\n}}\n\n#[websocket_message(response, method = \"r\")]\nstruct R;\n\n#[singleton]\nstruct Handler;\n\nimpl RespondsToWebSocketMessage for Handler {{\n    type Session = S;\n    type Message = R;\n}}\n"
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
            "{REQUEST_TRAIT}\n#[websocket_session(path = \"/x\", server = \"public\")]\nstruct S;\n\nimpl S {{\n    #[build_for_session]\n    fn build() -> anyhow::Result<Self> {{}}\n}}\n\n#[websocket_message(notification, method = \"n\")]\nstruct N;\n\n#[singleton]\nstruct Handler;\n\nimpl RespondsToWebSocketMessage for Handler {{\n    type Session = S;\n    type Message = N;\n}}\n"
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
            "{NOTIFICATION_TRAIT}\n#[websocket_session(path = \"/x\", server = \"public\")]\nstruct S;\n\nimpl S {{\n    #[build_for_session]\n    fn build() -> anyhow::Result<Self> {{}}\n}}\n\n#[websocket_message(request, method = \"r\", response = single)]\nstruct R;\n\n#[singleton]\nstruct Handler;\n\nimpl RespondsToWebSocketNotification for Handler {{\n    type Session = S;\n    type Message = R;\n}}\n"
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
    fn build() -> anyhow::Result<Self> {}
}

#[websocket_session(path = "/second", server = "public")]
struct SecondSession;

impl SecondSession {
    #[build_for_session]
    fn build() -> anyhow::Result<Self> {}
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
            "{REQUEST_TRAIT}\n#[websocket_session(path = \"/x\", server = \"public\")]\nstruct S;\n\nimpl S {{\n    #[build_for_session]\n    fn build() -> anyhow::Result<Self> {{}}\n}}\n\n#[websocket_message(request, method = \"m\", response = single)]\nstruct M;\n\n#[singleton]\nstruct First;\n\nimpl RespondsToWebSocketMessage for First {{\n    type Session = S;\n    type Message = M;\n}}\n\n#[singleton]\nstruct Second;\n\nimpl RespondsToWebSocketMessage for Second {{\n    type Session = S;\n    type Message = M;\n}}\n"
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
    fn generates_a_module_declaration_for_each_session() {
        let source = generated(
            r#"
#[websocket_session(path = "/a", server = "public")]
struct AlphaSession;

impl AlphaSession {
    #[build_for_session]
    fn build() -> anyhow::Result<Self> {}
}

#[websocket_session(path = "/b", server = "public")]
struct BetaSession;

impl BetaSession {
    #[build_for_session]
    fn build() -> anyhow::Result<Self> {}
}
"#,
        );

        assert!(source.contains("pubmodalpha_session"));
        assert!(source.contains("pubmodbeta_session"));
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
            error(r"#[websocket_message(request, method = 5, response = single)] struct Bad;")
                .to_string()
                .contains("failed to read")
        );
    }

    #[test]
    fn rejects_a_notification_without_a_method() {
        assert!(
            error(r"#[websocket_message(notification)] struct Bad;")
                .to_string()
                .contains("missing the required 'method'")
        );
    }

    #[test]
    fn rejects_a_response_that_declares_a_cardinality() {
        assert!(
            error(r"#[websocket_message(response, response = single)] struct Bad;")
                .to_string()
                .contains("declares a response cardinality but is not a request")
        );
    }

    #[test]
    fn propagates_a_non_string_response_method() {
        assert!(
            error(r"#[websocket_message(response, method = 5)] struct Bad;")
                .to_string()
                .contains("failed to read")
        );
    }

    #[test]
    fn propagates_malformed_message_arguments() {
        assert!(
            error(r"#[websocket_message(= 5)] struct Bad;")
                .to_string()
                .contains("failed to read")
        );
    }

    #[test]
    fn propagates_malformed_session_arguments() {
        assert!(
            error(r"#[websocket_session(= 5)] struct Bad;")
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
    fn build(#[route_parameter(= 5)] id: String) -> anyhow::Result<Self> {}
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
    fn build(#[route_parameter(from = 5)] id: String) -> anyhow::Result<Self> {}
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
            "{REQUEST_TRAIT}\n#[websocket_session(path = \"/x\", server = \"public\")]\nstruct S;\n\nimpl S {{\n    #[build_for_session]\n    fn build() -> anyhow::Result<Self> {{}}\n}}\n\n#[singleton]\nstruct Handler;\n\nimpl RespondsToWebSocketMessage for Handler {{\n    type Session = S;\n}}\n"
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
            "{REQUEST_TRAIT}\n#[websocket_session(path = \"/x\", server = \"public\")]\nstruct S;\n\nimpl S {{\n    #[build_for_session]\n    fn build() -> anyhow::Result<Self> {{}}\n}}\n\n#[websocket_message(request, method = \"m\", response = single)]\nstruct First;\n\n#[websocket_message(request, method = \"m\", response = single)]\nstruct Second;\n\n#[singleton]\nstruct HandlerOne;\n\nimpl RespondsToWebSocketMessage for HandlerOne {{\n    type Session = S;\n    type Message = First;\n}}\n\n#[singleton]\nstruct HandlerTwo;\n\nimpl RespondsToWebSocketMessage for HandlerTwo {{\n    type Session = S;\n    type Message = Second;\n}}\n"
        );

        assert!(error(&source).to_string().contains("more than once"));
    }

    fn transport_policies(source: &str) -> BTreeMap<String, ServerTransportPolicy> {
        let index = IndexedSource::new(source).index;
        let registries = registries_for(&index);
        let tags = TagPool::collect(&index).expect("the tags are collected");
        let plans = MiddlewarePlans::collect(&index, &registries, &tags)
            .expect("the middleware plans are collected");

        render_websocket(&index, &bindings(&index), &plans, &registries)
            .expect("the websocket module is generated")
            .servers
            .into_iter()
            .map(|(server, requirements)| (server, requirements.transport_policy))
            .collect()
    }

    #[test]
    fn pins_only_the_server_whose_session_builder_reads_the_peer_spiffe_id() {
        let policies = transport_policies(
            r#"
use spiffe::spiffe_id::SpiffeId;

#[websocket_session(path = "/peer", server = "mesh")]
struct PeerSession;

impl PeerSession {
    #[build_for_session]
    fn build(peer: &SpiffeId) -> anyhow::Result<Self> {}
}

#[websocket_session(path = "/room", server = "public")]
struct Room;

impl Room {
    #[build_for_session]
    fn build() -> anyhow::Result<Self> {}
}
"#,
        );

        assert_eq!(
            policies,
            BTreeMap::from([
                ("mesh".to_string(), ServerTransportPolicy::PinnedSpiffeMtls),
                ("public".to_string(), ServerTransportPolicy::Negotiable),
            ])
        );
    }

    #[test]
    fn pins_the_server_whose_session_middleware_reads_the_peer_spiffe_id() {
        let policies = transport_policies(
            r#"
use margaret::framework::http::next::Next;
use spiffe::spiffe_id::SpiffeId;

#[websocket_session(path = "/peer", server = "mesh")]
#[middleware(verified)]
struct PeerSession;

impl PeerSession {
    #[build_for_session]
    fn build() -> anyhow::Result<Self> {}
}

#[handles_middleware_attribute(attribute = verified)]
struct Verifier;

impl Verifier {
    #[process]
    fn process(&self, peer: &SpiffeId, next: Next) -> anyhow::Result<ResponseContinuation> {}
}
"#,
        );

        assert_eq!(
            policies,
            BTreeMap::from([("mesh".to_string(), ServerTransportPolicy::PinnedSpiffeMtls)])
        );
    }

    #[test]
    fn pins_a_server_when_any_of_its_sessions_reads_the_peer_spiffe_id() {
        let policies = transport_policies(
            r#"
use spiffe::spiffe_id::SpiffeId;

#[websocket_session(path = "/open", server = "mesh")]
struct OpenSession;

impl OpenSession {
    #[build_for_session]
    fn build() -> anyhow::Result<Self> {}
}

#[websocket_session(path = "/peer", server = "mesh")]
struct PeerSession;

impl PeerSession {
    #[build_for_session]
    fn build(peer: &SpiffeId) -> anyhow::Result<Self> {}
}
"#,
        );

        assert_eq!(
            policies,
            BTreeMap::from([("mesh".to_string(), ServerTransportPolicy::PinnedSpiffeMtls)])
        );
    }
}
