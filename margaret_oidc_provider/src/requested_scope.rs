use std::borrow::Borrow;
use std::collections::BTreeSet;
use std::ops::ControlFlow;

use oauth2::basic::BasicErrorResponseType;

use margaret_http::response::Response;
use margaret_oauth_vocabulary::scope::Scope;
use margaret_oauth_vocabulary::scope_list::ScopeList;

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
        match scope.map(str::parse::<ScopeList>) {
            Some(Ok(ScopeList { scopes })) => ControlFlow::Continue(Self::Narrowed(scopes)),
            Some(Err(_)) => ControlFlow::Break(invalid_scope()),
            None => ControlFlow::Continue(Self::Granted),
        }
    }

    pub(crate) fn resolved<TGranted: Borrow<Scope> + Ord>(
        scope: Option<&str>,
        granted: &BTreeSet<TGranted>,
    ) -> ControlFlow<Response, BTreeSet<Scope>> {
        Self::of(scope)?.within(granted)
    }

    pub(crate) fn within<TGranted: Borrow<Scope> + Ord>(
        self,
        granted: &BTreeSet<TGranted>,
    ) -> ControlFlow<Response, BTreeSet<Scope>> {
        match self {
            Self::Granted => ControlFlow::Continue(
                granted
                    .iter()
                    .map(|scope| Scope::clone(scope.borrow()))
                    .collect(),
            ),
            Self::Narrowed(scopes) if scopes.iter().all(|scope| granted.contains(scope)) => {
                ControlFlow::Continue(scopes)
            }
            Self::Narrowed(_) => ControlFlow::Break(invalid_scope()),
        }
    }
}
