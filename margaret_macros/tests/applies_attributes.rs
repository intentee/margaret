use margaret_macros::build_for_session;
use margaret_macros::console_command;
use margaret_macros::constructor;
use margaret_macros::handles_middleware_attribute;
use margaret_macros::middleware;
use margaret_macros::model;
use margaret_macros::process;
use margaret_macros::provides_endpoint;
use margaret_macros::provides_route_parameter;
use margaret_macros::renders_view;
use margaret_macros::responds_to_http;
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
struct Subject;

impl Subject {
    #[constructor]
    fn create() -> Self {
        Self
    }

    #[process]
    fn respond(&self, #[route_parameter] id: String) -> String {
        id
    }

    #[process]
    fn run(&self, #[console_argument] name: String) -> String {
        name
    }
}

#[provides_route_parameter]
struct Binder;

impl Binder {
    fn bind(&self, value: String) -> String {
        value
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

#[provides_endpoint(jwks)]
struct Endpoint {
    origin: String,
}

impl Endpoint {
    #[constructor]
    fn create() -> Self {
        Self {
            origin: "origin".to_string(),
        }
    }
}

struct Consumer {
    endpoint: Endpoint,
}

impl Consumer {
    #[constructor]
    fn create(#[endpoint_provider(jwks)] endpoint: Endpoint) -> Self {
        Self { endpoint }
    }
}

#[websocket_message(request, method = "message", response = single)]
struct Message {
    field: String,
}

#[test]
fn attribute_macros_leave_runtime_behavior_untouched() {
    let subject = Subject::create();

    assert_eq!(subject.respond("path-id".to_string()), "path-id");
    assert_eq!(subject.run("typed-name".to_string()), "typed-name");
    assert_eq!(Binder.bind("bound".to_string()), "bound");

    let _worker = Worker;

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

    let consumer = Consumer::create(Endpoint::create());

    assert_eq!(consumer.endpoint.origin, "origin");
}
