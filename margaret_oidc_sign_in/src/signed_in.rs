use cookie::Cookie;
use oauth2::AccessToken;

pub struct SignedIn<TIdClaims> {
    pub access_token: AccessToken,
    pub claims: TIdClaims,
    pub subject: String,
    pub transaction_removal: Cookie<'static>,
}
