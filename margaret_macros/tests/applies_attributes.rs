use margaret_macros::can;
use margaret_macros::console_command;
use margaret_macros::constructor;
use margaret_macros::decides;
use margaret_macros::decides_crud_action;
use margaret_macros::decides_site_action;
use margaret_macros::handles_middleware_attribute;
use margaret_macros::intercepts;
use margaret_macros::provide;
use margaret_macros::provider;
use margaret_macros::provides_authenticated_actor;
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
#[can(SomeAction::View)]
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

#[provider]
struct Catalog;

impl Catalog {
    #[provide]
    fn provide(&self) -> u8 {
        7
    }
}

#[provides_authenticated_actor]
#[provides_route_parameter]
#[decides_crud_action]
#[decides_site_action(SomeAction::Manage)]
struct Authority;

impl Authority {
    #[decides]
    fn can(&self, granted: bool) -> bool {
        granted
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
    assert_eq!(Catalog.provide(), 7);
    assert!(Authority.can(true));

    let _worker = Worker;
}
