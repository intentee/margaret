use std::collections::HashMap;
use std::ops::ControlFlow;

use oauth2::AuthorizationCode;

use margaret_authorization_server_client::server_unavailability::ServerUnavailability;
use margaret_identity_session::sign_in_transaction_claims::SignInTransactionClaims;
use margaret_issuer_metadata::issuer_metadata::IssuerMetadata;
use margaret_issuer_metadata::metadata_holding::MetadataHolding;
use margaret_oidc_discovery::authorization_response_issuer::AuthorizationResponseIssuer;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

use crate::equal_in_constant_time::equal_in_constant_time;
use crate::sign_in_completion::SignInCompletion;
use crate::sign_in_refusal::SignInRefusal;

fn refused<TIdClaims, TContinuation>(
    refusal: SignInRefusal,
) -> ControlFlow<SignInCompletion<TIdClaims>, TContinuation> {
    ControlFlow::Break(SignInCompletion::Refused(refusal))
}

fn named_issuer<TIdClaims>(
    query: &HashMap<String, String>,
    issuer: &IssuerIdentifier,
    metadata: &IssuerMetadata,
) -> ControlFlow<SignInCompletion<TIdClaims>> {
    let advertised = match metadata.holding() {
        MetadataHolding::Awaiting => {
            return ControlFlow::Break(SignInCompletion::Unavailable(
                ServerUnavailability::MetadataAwaited,
            ));
        }
        MetadataHolding::Held(metadata) => metadata.authorization_response_issuer,
    };

    match query.get("iss") {
        Some(found) if found != issuer.as_str() => refused(SignInRefusal::IssuerMismatch {
            found: found.clone(),
        }),
        Some(_) => ControlFlow::Continue(()),
        None => match advertised {
            AuthorizationResponseIssuer::Advertised => refused(SignInRefusal::IssuerMissing),
            AuthorizationResponseIssuer::Unadvertised => ControlFlow::Continue(()),
        },
    }
}

pub(crate) fn callback_code<TIdClaims>(
    query: &HashMap<String, String>,
    transaction: &SignInTransactionClaims,
    issuer: &IssuerIdentifier,
    metadata: &IssuerMetadata,
) -> ControlFlow<SignInCompletion<TIdClaims>, AuthorizationCode> {
    if !query
        .get("state")
        .is_some_and(|state| equal_in_constant_time(state, &transaction.state))
    {
        return refused(SignInRefusal::StateMismatch);
    }

    named_issuer(query, issuer, metadata)?;

    if let Some(error) = query.get("error") {
        return refused(SignInRefusal::AuthorizationDenied {
            description: query.get("error_description").cloned(),
            error: error.clone(),
        });
    }

    match query.get("code") {
        Some(code) => ControlFlow::Continue(AuthorizationCode::new(code.clone())),
        None => refused(SignInRefusal::CodeMissing),
    }
}
