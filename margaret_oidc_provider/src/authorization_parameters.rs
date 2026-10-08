use std::collections::BTreeSet;
use std::ops::ControlFlow;

use chrono::TimeDelta;

use margaret_accepted_clients::code_grant_policy::CodeGrantPolicy;
use margaret_oauth_vocabulary::code_challenge::CodeChallenge;
use margaret_oauth_vocabulary::code_challenge_method::CodeChallengeMethod;
use margaret_oauth_vocabulary::code_challenge_method_parsing::CodeChallengeMethodParsing;
use margaret_oauth_vocabulary::code_challenge_parsing::CodeChallengeParsing;
use margaret_oauth_vocabulary::prompt_value::PromptValue;
use margaret_oauth_vocabulary::prompt_value_parsing::PromptValueParsing;
use margaret_oauth_vocabulary::scope::Scope;
use margaret_oauth_vocabulary::scope_list::ScopeList;
use margaret_oauth_vocabulary::scope_list_parsing::ScopeListParsing;
use margaret_oauth_vocabulary::space_delimited::space_delimited;

use crate::authentication_age::AuthenticationAge;
use crate::authorization_error::AuthorizationError;
use crate::authorization_request::AuthorizationRequest;
use crate::prompt::Prompt;

const CODE_RESPONSE_TYPE: &str = "code";

fn prompt(request: &AuthorizationRequest) -> ControlFlow<AuthorizationError, Prompt> {
    let Some(prompt) = &request.prompt else {
        return ControlFlow::Continue(Prompt::Interactive);
    };
    let mut values = BTreeSet::new();

    for value in space_delimited(prompt) {
        match PromptValue::parse(value) {
            PromptValueParsing::Accepted(value) => {
                values.insert(value);
            }
            PromptValueParsing::Unsupported => {
                return ControlFlow::Break(AuthorizationError::UnsupportedPromptValue);
            }
        }
    }

    match values.into_iter().collect::<Vec<PromptValue>>().as_slice() {
        [PromptValue::Consent] => ControlFlow::Continue(Prompt::Consent),
        [PromptValue::Consent, PromptValue::Login] => {
            ControlFlow::Continue(Prompt::LoginAndConsent)
        }
        [PromptValue::Login] => ControlFlow::Continue(Prompt::Login),
        [PromptValue::None] => ControlFlow::Continue(Prompt::NoInteraction),
        _ => ControlFlow::Break(AuthorizationError::ConflictingPromptValues),
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
            Err(_) => ControlFlow::Break(AuthorizationError::MalformedMaxAge),
        },
        None => ControlFlow::Continue(AuthenticationAge::Unbounded),
    }
}

fn code_challenge(
    request: &AuthorizationRequest,
) -> ControlFlow<AuthorizationError, CodeChallenge> {
    match request
        .code_challenge_method
        .as_deref()
        .map(CodeChallengeMethod::parse)
    {
        Some(CodeChallengeMethodParsing::Accepted(CodeChallengeMethod::S256)) => {}
        Some(CodeChallengeMethodParsing::Unsupported) => {
            return ControlFlow::Break(AuthorizationError::UnsupportedCodeChallengeMethod);
        }
        None => return ControlFlow::Break(AuthorizationError::MissingCodeChallengeMethod),
    }

    match request.code_challenge.as_deref().map(CodeChallenge::parse) {
        Some(CodeChallengeParsing::Accepted(challenge)) => ControlFlow::Continue(challenge),
        Some(CodeChallengeParsing::Rejected(rejection)) => {
            ControlFlow::Break(AuthorizationError::MalformedCodeChallenge(rejection))
        }
        None => ControlFlow::Break(AuthorizationError::MissingCodeChallenge),
    }
}

fn granted_scopes(
    policy: &CodeGrantPolicy,
    request: &AuthorizationRequest,
) -> ControlFlow<AuthorizationError, BTreeSet<Scope>> {
    match request.scope.as_deref().map(ScopeList::parse) {
        Some(ScopeListParsing::Accepted(ScopeList { scopes }))
            if scopes
                .iter()
                .all(|scope| policy.scopes.contains(&scope.as_str())) =>
        {
            ControlFlow::Continue(scopes)
        }
        Some(ScopeListParsing::Accepted(_)) => {
            ControlFlow::Break(AuthorizationError::ScopeNotGranted)
        }
        Some(ScopeListParsing::Rejected(_)) => {
            ControlFlow::Break(AuthorizationError::MalformedScope)
        }
        None => ControlFlow::Continue(BTreeSet::new()),
    }
}

pub(crate) struct AuthorizationParameters {
    pub(crate) authentication_age: AuthenticationAge,
    pub(crate) code_challenge: CodeChallenge,
    pub(crate) nonce: Option<String>,
    pub(crate) prompt: Prompt,
    pub(crate) scopes: BTreeSet<Scope>,
}

impl AuthorizationParameters {
    pub(crate) fn of(
        policy: &CodeGrantPolicy,
        request: &AuthorizationRequest,
    ) -> ControlFlow<AuthorizationError, Self> {
        if request.request.is_some() {
            return ControlFlow::Break(AuthorizationError::RequestNotSupported);
        }

        if request.request_uri.is_some() {
            return ControlFlow::Break(AuthorizationError::RequestUriNotSupported);
        }

        if request.response_type.as_deref() != Some(CODE_RESPONSE_TYPE) {
            return ControlFlow::Break(AuthorizationError::UnsupportedResponseType);
        }

        ControlFlow::Continue(Self {
            authentication_age: authentication_age(request)?,
            code_challenge: code_challenge(request)?,
            nonce: request.nonce.clone(),
            prompt: prompt(request)?,
            scopes: granted_scopes(policy, request)?,
        })
    }
}
