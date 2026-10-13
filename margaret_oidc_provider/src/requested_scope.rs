use std::collections::BTreeSet;
use std::ops::ControlFlow;

use oauth2::basic::BasicErrorResponseType;

use margaret_http::response::Response;
use margaret_oauth_vocabulary::scope::Scope;
use margaret_oauth_vocabulary::scope_list::ScopeList;
use margaret_oauth_vocabulary::scope_list_parsing::ScopeListParsing;

use crate::oauth_error::oauth_error;

fn invalid_scope() -> Response {
    oauth_error(
        400,
        BasicErrorResponseType::InvalidScope,
        "the requested scope exceeds the granted scope",
    )
}

pub(crate) enum RequestedScope {
    Granted,
    Narrowed(BTreeSet<Scope>),
}

impl RequestedScope {
    pub(crate) fn of(scope: Option<&str>) -> ControlFlow<Response, Self> {
        match scope.map(ScopeList::parse) {
            Some(ScopeListParsing::Accepted(ScopeList { scopes })) => {
                ControlFlow::Continue(Self::Narrowed(scopes))
            }
            Some(ScopeListParsing::Rejected(_)) => ControlFlow::Break(invalid_scope()),
            None => ControlFlow::Continue(Self::Granted),
        }
    }

    pub(crate) fn resolved(
        scope: Option<&str>,
        granted: &BTreeSet<&str>,
    ) -> ControlFlow<Response, BTreeSet<String>> {
        Self::of(scope)?.within(granted)
    }

    pub(crate) fn within(
        self,
        granted: &BTreeSet<&str>,
    ) -> ControlFlow<Response, BTreeSet<String>> {
        match self {
            Self::Granted => {
                ControlFlow::Continue(granted.iter().map(ToString::to_string).collect())
            }
            Self::Narrowed(scopes)
                if scopes.iter().all(|scope| granted.contains(scope.as_str())) =>
            {
                ControlFlow::Continue(
                    scopes
                        .iter()
                        .map(|scope| scope.as_str().to_string())
                        .collect(),
                )
            }
            Self::Narrowed(_) => ControlFlow::Break(invalid_scope()),
        }
    }
}
