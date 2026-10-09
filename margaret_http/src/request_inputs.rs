use std::collections::HashMap;

use cookie::Cookie;
use http::header::COOKIE;

use crate::form_fields::form_fields;
use crate::named_value::NamedValue;
use crate::request_outcome::RequestOutcome;
use crate::request_rejection::RequestRejection;
use crate::server_params::ServerParams;
use crate::unique_index::UniqueIndex;

fn parse_cookies(server: &ServerParams) -> RequestOutcome<HashMap<String, String>> {
    let Some(raw) = server.header(&COOKIE) else {
        return RequestOutcome::Parsed(HashMap::new());
    };
    let mut cookies = Vec::new();

    for parsed in Cookie::split_parse(raw) {
        match parsed {
            Ok(cookie) => cookies.push(NamedValue {
                name: cookie.name().to_string(),
                value: cookie.value().to_string(),
            }),
            Err(source) => {
                return RequestOutcome::Rejected(RequestRejection::MalformedCookie { source });
            }
        }
    }

    match UniqueIndex::of(cookies) {
        UniqueIndex::Indexed(indexed) => RequestOutcome::Parsed(indexed),
        UniqueIndex::Duplicated { name } => {
            RequestOutcome::Rejected(RequestRejection::DuplicateCookie { name })
        }
    }
}

fn parse_query(server: &ServerParams) -> RequestOutcome<HashMap<String, String>> {
    match UniqueIndex::of(form_fields(server.query_string().as_bytes())) {
        UniqueIndex::Indexed(indexed) => RequestOutcome::Parsed(indexed),
        UniqueIndex::Duplicated { name } => {
            RequestOutcome::Rejected(RequestRejection::DuplicateQueryParameter { name })
        }
    }
}

pub struct RequestInputs {
    pub cookies: HashMap<String, String>,
    pub query: HashMap<String, String>,
    pub server: ServerParams,
}

impl RequestInputs {
    pub(crate) fn parse(server: ServerParams) -> RequestOutcome<Self> {
        let cookies = match parse_cookies(&server) {
            RequestOutcome::Parsed(cookies) => cookies,
            RequestOutcome::Rejected(rejection) => return RequestOutcome::Rejected(rejection),
        };
        let query = match parse_query(&server) {
            RequestOutcome::Parsed(query) => query,
            RequestOutcome::Rejected(rejection) => return RequestOutcome::Rejected(rejection),
        };

        RequestOutcome::Parsed(Self {
            cookies,
            query,
            server,
        })
    }
}
