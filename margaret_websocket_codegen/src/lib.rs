pub mod has_websocket_sessions;
pub mod render_websocket;
pub mod websocket_artifacts;
pub mod websocket_codegen_error;

mod build_for_session_method;
mod discovered_handler;
mod handler_binding;
mod handler_kind;
mod message_cardinality;
mod message_kind;
mod render_messages;
mod render_server_routes;
mod render_sessions;
mod session_arguments;
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

    use crate::has_websocket_sessions::has_websocket_sessions;
    use crate::render_websocket::render_websocket;
    use crate::websocket_codegen_error::WebSocketCodegenError;

    const REQUEST_TRAIT: &str =
        "use margaret_websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;\n";
    const NOTIFICATION_TRAIT: &str = "use margaret_websocket::responds_to_web_socket_notification::RespondsToWebSocketNotification;\n";

    const FULL_SESSION: &str = r#"
use std::sync::Arc;
use margaret_websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;
use margaret_websocket::responds_to_web_socket_notification::RespondsToWebSocketNotification;

trait Clock {}

#[singleton(provides = Clock)]
struct SystemClock;

impl SystemClock {
    #[constructor]
    fn new() -> Self {}
}

impl Clock for SystemClock {}

#[singleton]
struct Config;

impl Config {
    #[constructor]
    fn new() -> Self {}
}

trait Plugin {}

#[singleton(collection = Plugin)]
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
        clock: Arc<dyn Clock>,
        config: Arc<Config>,
        plugins: Vec<Arc<dyn Plugin>>,
        #[route_parameter(from = "room")] room: String,
    ) -> Self {}
}

#[websocket_message(request, method = "say", response = stream)]
struct Say;

#[websocket_message(request, method = "ping", response = single)]
struct Ping;

#[websocket_message(notification, method = "typing")]
struct Typing;

#[websocket_message(response)]
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
        render_container(index)
            .expect("the container renders")
            .bindings
    }

    fn generated(source: &str) -> String {
        generated_with_views(source, false)
    }

    fn generated_with_views(source: &str, has_views: bool) -> String {
        let index = index_for(source);
        let plans = middleware_plans(&index).expect("the middleware plans are collected");

        render_websocket(&index, &bindings(&index), has_views, &plans)
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
        let plans = middleware_plans(&index).expect("the middleware plans are collected");

        render_websocket(&index, &bindings(&index), false, &plans)
            .expect_err("the websocket module is rejected")
    }

    const SESSION_WITH_MIDDLEWARE: &str = r#"
use margaret_http::next::Next;
use margaret_http::request::Request;

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
use margaret_http::next::Next;
use margaret_http::request::Request;

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

    #[test]
    fn wraps_the_upgrade_entry_with_its_middleware() {
        let source = generated(SESSION_WITH_MIDDLEWARE);

        assert!(source.contains(
            "margaret_http::gated_web_socket_upgrade::GatedWebSocketUpgrade::new"
        ));
        assert!(source.contains(
            "middleware.push(std::sync::Arc::new(super::super::middleware::Guard{inner:container.guard().await"
        ));
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
    fn threads_routes_when_only_a_middleware_injects_them() {
        let source = generated(SESSION_WITH_ROUTES_MIDDLEWARE);

        assert!(source.contains("routes:&::std::sync::Arc<super::super::routes::Routes>"));
        assert!(source.contains("super::super::middleware::Tracer{inner:container.tracer().await,routes:routes.clone()"));
        assert!(source.contains("upgrade_entry(container,routes)"));
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
    }

    #[test]
    fn generates_a_factory_with_every_dependency_shape() {
        let source = generated(FULL_SESSION);

        assert!(source.contains("clock:::std::sync::Arc<dyncrate::Clock>"));
        assert!(source.contains("config:::std::sync::Arc<crate::Config>"));
        assert!(source.contains("plugins:::std::vec::Vec<::std::sync::Arc<dyncrate::Plugin>>"));
        assert!(source.contains("container.system_clock().await"));
        assert!(source.contains("container.config().await"));
        assert!(source.contains("::std::vec::Vec::from([container.log_plugin().await])"));
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
    fn rejects_a_response_that_declares_a_method() {
        assert!(
            error(r#"#[websocket_message(response, method = "x")] struct Bad;"#)
                .to_string()
                .contains("must not declare a 'method'")
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

    const PARITY_SESSION: &str = r#"
use std::sync::Arc;
use margaret_websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;

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

impl HttpRouteParameterBinder for ArticleStore {
    type Model = Article;
}

struct Filters;

#[websocket_session(path = "/board/{topic}/{article}", server = "public")]
struct BoardSession;

impl BoardSession {
    #[build_for_session]
    fn build_for_session(
        greeter: Arc<Greeter>,
        #[route_parameter(from = "topic")] topic: String,
        #[route_parameter(from = "article")] article: Article,
        #[form_request(from = Query)] filters: Filters,
        request: &margaret_http::request::Request,
        peer: &spiffe::spiffe_id::SpiffeId,
        routes: &crate::margaret::routes::Routes,
        views: &crate::margaret::views::Views,
        assets: margaret_asset_bag::asset_bag::AssetBag,
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
        let source = generated_with_views(PARITY_SESSION, true);

        assert!(source.contains("require_bound_route_parameter::require_bound_route_parameter"));
        assert!(source.contains("RequestInput::Query"));
        assert!(source.contains("require_peer_spiffe_id::require_peer_spiffe_id"));
        assert!(source.contains("::margaret_asset_bag::asset_bag::AssetBag::new()"));
        assert!(source.contains("self.routes.as_ref()"));
        assert!(source.contains("self.views.as_ref()"));
        assert!(source.contains("routes:&::std::sync::Arc<super::super::routes::Routes>"));
        assert!(source.contains("views:&::std::sync::Arc<super::super::views::Views>"));
        assert!(source.contains("upgrade_entry(container,routes,views)"));
        assert!(source.contains("routes:&::std::sync::Arc<super::routes::Routes>"));
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
            "{REQUEST_TRAIT}\n#[websocket_session(path = \"/x\", server = \"public\")]\nstruct S;\n\nimpl S {{\n    #[build_for_session]\n    fn build() -> Self {{}}\n}}\n\n#[websocket_message(response)]\nstruct R;\n\n#[singleton]\nstruct Handler;\n\nimpl RespondsToWebSocketMessage for Handler {{\n    type Session = S;\n    type Message = R;\n}}\n"
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
