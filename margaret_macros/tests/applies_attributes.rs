mod catalog {
    use std::time::Duration;

    pub const TICK_INTERVAL: Duration = Duration::from_secs(1);

    pub struct Account;

    pub enum TickBehavior {
        Delay,
    }

    pub enum RequestInput {
        Query,
    }
}

use margaret_macros::accepts_oauth_client;
use margaret_macros::build_for_session;
use margaret_macros::console_command;
use margaret_macros::constructor;
use margaret_macros::exchanges_subject_tokens;
use margaret_macros::handles_middleware_attribute;
use margaret_macros::infer_from_request;
use margaret_macros::infers_authenticated_user;
use margaret_macros::issues_tokens;
use margaret_macros::middleware;
use margaret_macros::model;
use margaret_macros::oauth_client;
use margaret_macros::process;
use margaret_macros::provides_jwks_endpoint;
use margaret_macros::provides_route_parameter;
use margaret_macros::renders_view;
use margaret_macros::responds_to_http;
use margaret_macros::route_parameter_value;
use margaret_macros::scheduled_with_tick_timer;
use margaret_macros::service;
use margaret_macros::singleton;
use margaret_macros::trusts_oidc_issuer;
use margaret_macros::websocket_message;
use margaret_macros::websocket_session;

use crate::catalog::Account;
use crate::catalog::RequestInput;
use crate::catalog::TICK_INTERVAL;
use crate::catalog::TickBehavior;

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

    #[process]
    fn search(&self, #[form_request(from = RequestInput::Query)] phrase: &str) -> String {
        format!("{}?{phrase}", self.name)
    }
}

#[provides_route_parameter]
struct Binder {
    prefix: String,
}

#[provides_jwks_endpoint]
struct JwksEndpoint;

#[issues_tokens]
struct TokenIssuer;

#[trusts_oidc_issuer(partner)]
struct PartnerIssuer;

#[oauth_client(partner_client, issuer = partner)]
struct PartnerClient;

#[accepts_oauth_client]
struct PortalClient;

#[exchanges_subject_tokens(issuer = partner)]
struct PartnerExchanger;

#[route_parameter_value]
struct SubjectId(String);

impl Binder {
    fn bind(&self, value: &str) -> String {
        format!("{}{value}", self.prefix)
    }
}

#[service]
#[scheduled_with_tick_timer(interval = TICK_INTERVAL, behavior = TickBehavior::Delay)]
struct Worker;

#[infers_authenticated_user(user_model = Account)]
struct AccountProvider;

impl AccountProvider {
    #[infer_from_request]
    fn infer(&self, #[bearer_token(issuer = partner)] token: &str) -> String {
        format!("verified {token}")
    }
}

#[model(table = "records")]
#[primary_key(columns = [id, label])]
#[unique(columns = [label, id])]
#[index(name = "records_label_id", columns = [label, id])]
#[foreign_key(columns = [id], references = Account)]
struct Record {
    #[column(name = "id")]
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
    assert_eq!(subject.search("term"), "typed-name?term");
    assert_eq!(
        Binder {
            prefix: "bind-".to_string(),
        }
        .bind("bound"),
        "bind-bound"
    );

    assert_eq!(SubjectId("7".to_string()).0, "7");
    assert_eq!(size_of::<JwksEndpoint>(), 0);
    assert_eq!(size_of::<TokenIssuer>(), 0);
    assert_eq!(size_of::<PartnerIssuer>(), 0);
    assert_eq!(size_of::<PartnerClient>(), 0);
    assert_eq!(size_of::<PortalClient>(), 0);
    assert_eq!(size_of::<PartnerExchanger>(), 0);
    assert_eq!(size_of::<Worker>(), 0);
    assert_eq!(size_of_val(&AccountProvider), 0);
    assert_eq!(AccountProvider.infer("bearer"), "verified bearer");
    assert_eq!(size_of_val(&Account), 0);

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
