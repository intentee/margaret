use margaret_authorization_server_client::server_unavailability::ServerUnavailability;

use crate::sign_in_refusal::SignInRefusal;
use crate::signed_in::SignedIn;

pub enum SignInCompletion<TIdClaims> {
    Refused(SignInRefusal),
    SignedIn(SignedIn<TIdClaims>),
    SigningKeysAwaited,
    Unavailable(ServerUnavailability),
}
