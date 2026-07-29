use margaret_macros::build_for_session;
use margaret_macros::console_command;
use margaret_macros::constructor;
use margaret_macros::handles_middleware_attribute;
use margaret_macros::middleware;
use margaret_macros::model;
use margaret_macros::process;
use margaret_macros::provides_jwks_endpoint;
use margaret_macros::provides_route_parameter;
use margaret_macros::renders_view;
use margaret_macros::responds_to_http;
use margaret_macros::route_parameter_value;
use margaret_macros::scheduled_with_tick_timer;
use margaret_macros::service;
use margaret_macros::singleton;
use margaret_macros::websocket_message;
use margaret_macros::websocket_session;

#[singleton]
#[responds_to_http(method = Get, path = "/subject", server = "public")]
#[renders_view(name = "subject")]
#[console_command]
#[handles_middleware_attribute(attribute = traced)]
#[middleware(traced)]
struct Subject {
    name: String,
}

impl Subject {
    #[constructor]
    fn create(#[console_argument(positional)] name: String) -> Self {
        Self { name }
    }

    #[process]
    fn respond(&self, #[route_parameter] id: &str) -> String {
        format!("{}/{id}", self.name)
    }

    #[process]
    fn run(&self) -> String {
        self.name.clone()
    }
}

#[provides_route_parameter]
struct Binder {
    prefix: String,
}

#[provides_jwks_endpoint]
struct JwksEndpoint;

#[route_parameter_value]
struct SubjectId(String);

impl Binder {
    fn bind(&self, value: &str) -> String {
        format!("{}{value}", self.prefix)
    }
}

#[service]
#[scheduled_with_tick_timer(interval = SomeInterval)]
struct Worker;

#[model(table = "records")]
struct Record {
    #[column(primary_key, name = "id")]
    id: String,
    #[column]
    label: String,
}

#[websocket_session(path = "/session/{topic}", server = "public")]
struct Session {
    topic: String,
}

impl Session {
    #[build_for_session]
    fn build(#[route_parameter(from = "topic")] topic: String) -> Self {
        Self { topic }
    }
}

#[websocket_message(request, method = "message", response = single)]
struct Message {
    field: String,
}

#[test]
fn attribute_macros_leave_runtime_behavior_untouched() {
    let subject = Subject::create("typed-name".to_string());

    assert_eq!(subject.respond("path-id"), "typed-name/path-id");
    assert_eq!(subject.run(), "typed-name");
    assert_eq!(
        Binder {
            prefix: "bind-".to_string(),
        }
        .bind("bound"),
        "bind-bound"
    );

    assert_eq!(SubjectId("7".to_string()).0, "7");
    assert_eq!(size_of::<JwksEndpoint>(), 0);
    assert_eq!(size_of::<Worker>(), 0);

    let record = Record {
        id: "the-id".to_string(),
        label: "the-label".to_string(),
    };

    assert_eq!(record.id, "the-id");
    assert_eq!(record.label, "the-label");

    let session = Session::build("weather".to_string());

    assert_eq!(session.topic, "weather");

    let message = Message {
        field: "value".to_string(),
    };

    assert_eq!(message.field, "value");
}
