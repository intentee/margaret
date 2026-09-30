use std::collections::BTreeSet;
use std::ops::ControlFlow;

use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;
use chrono::TimeDelta;

use margaret_accepted_clients::accepted_client::AcceptedClient;
use margaret_accepted_clients::grant_type::GrantType;
use margaret_oauth_vocabulary::scope::Scope;
use margaret_oauth_vocabulary::scope_list::ScopeList;

use crate::authentication_age::AuthenticationAge;
use crate::authorization_error::AuthorizationError;
use crate::authorization_request::AuthorizationRequest;
use crate::prompt::Prompt;

const CODE_CHALLENGE_OCTETS: usize = 32;
const CODE_RESPONSE_TYPE: &str = "code";
const S256: &str = "S256";

fn prompt(request: &AuthorizationRequest) -> ControlFlow<AuthorizationError, Prompt> {
    let Some(prompt) = &request.prompt else {
        return ControlFlow::Continue(Prompt::Interactive);
    };
    let values = prompt.split(' ').collect::<BTreeSet<&str>>();
    let consent = values.contains("consent");
    let login = values.contains("login");

    if values.contains("none") {
        if consent || login {
            ControlFlow::Break(AuthorizationError::InvalidRequest)
        } else {
            ControlFlow::Continue(Prompt::NoInteraction)
        }
    } else if login {
        ControlFlow::Continue(if consent {
            Prompt::LoginAndConsent
        } else {
            Prompt::Login
        })
    } else if consent {
        ControlFlow::Continue(Prompt::Consent)
    } else {
        ControlFlow::Continue(Prompt::Interactive)
    }
}

fn authentication_age(
    request: &AuthorizationRequest,
) -> ControlFlow<AuthorizationError, AuthenticationAge> {
    match &request.max_age {
        Some(max_age) => match max_age.parse::<u32>() {
            Ok(seconds) => ControlFlow::Continue(AuthenticationAge::AtMost(TimeDelta::seconds(
                i64::from(seconds),
            ))),
            Err(_) => ControlFlow::Break(AuthorizationError::InvalidRequest),
        },
        None => ControlFlow::Continue(AuthenticationAge::Unbounded),
    }
}

fn code_challenge(request: &AuthorizationRequest) -> ControlFlow<AuthorizationError, String> {
    let s256 = request
        .code_challenge_method
        .as_ref()
        .is_some_and(|method| method == S256);

    match &request.code_challenge {
        Some(challenge)
            if s256
                && Base64UrlUnpadded::decode_vec(challenge)
                    .is_ok_and(|digest| digest.len() == CODE_CHALLENGE_OCTETS) =>
        {
            ControlFlow::Continue(challenge.clone())
        }
        Some(_) | None => ControlFlow::Break(AuthorizationError::InvalidRequest),
    }
}

fn granted_scopes(
    client: &AcceptedClient,
    request: &AuthorizationRequest,
) -> ControlFlow<AuthorizationError, BTreeSet<Scope>> {
    match request.scope.as_deref().map(str::parse::<ScopeList>) {
        Some(Ok(ScopeList { scopes })) if scopes.is_subset(&client.scopes) => {
            ControlFlow::Continue(scopes)
        }
        Some(Ok(_) | Err(_)) => ControlFlow::Break(AuthorizationError::InvalidScope),
        None => ControlFlow::Continue(BTreeSet::new()),
    }
}

pub(crate) struct AuthorizationParameters {
    pub(crate) authentication_age: AuthenticationAge,
    pub(crate) code_challenge: String,
    pub(crate) nonce: Option<String>,
    pub(crate) prompt: Prompt,
    pub(crate) scopes: BTreeSet<Scope>,
}

impl AuthorizationParameters {
    pub(crate) fn of(
        client: &AcceptedClient,
        request: &AuthorizationRequest,
    ) -> ControlFlow<AuthorizationError, Self> {
        if request.response_type.as_deref() != Some(CODE_RESPONSE_TYPE) {
            return ControlFlow::Break(AuthorizationError::UnsupportedResponseType);
        }

        if !client.grants.contains(&GrantType::AuthorizationCode) {
            return ControlFlow::Break(AuthorizationError::UnauthorizedClient);
        }

        ControlFlow::Continue(Self {
            authentication_age: authentication_age(request)?,
            code_challenge: code_challenge(request)?,
            nonce: request.nonce.clone(),
            prompt: prompt(request)?,
            scopes: granted_scopes(client, request)?,
        })
    }
}
