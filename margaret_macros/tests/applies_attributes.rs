use margaret_macros::console_command;
use margaret_macros::constructor;
use margaret_macros::handles_middleware_attribute;
use margaret_macros::intercepts;
use margaret_macros::provides_route_parameter;
use margaret_macros::responder;
use margaret_macros::responds_to_http;
use margaret_macros::runner;
use margaret_macros::scheduled_with_tick_timer;
use margaret_macros::service;
use margaret_macros::singleton;

#[singleton]
#[responds_to_http(method = Get, path = "/subject", server = "public")]
#[console_command]
#[handles_middleware_attribute(attribute = traced)]
#[intercepts(SomeMarker)]
struct Subject;

impl Subject {
    #[constructor]
    fn create() -> Self {
        Self
    }

    #[responder]
    fn respond(&self, #[route_parameter] id: String) -> String {
        id
    }

    #[runner]
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

#[test]
fn attribute_macros_leave_runtime_behavior_untouched() {
    let subject = Subject::create();

    assert_eq!(subject.respond("path-id".to_string()), "path-id");
    assert_eq!(subject.run("typed-name".to_string()), "typed-name");
    assert_eq!(Binder.bind("bound".to_string()), "bound");

    let _worker = Worker;
}
