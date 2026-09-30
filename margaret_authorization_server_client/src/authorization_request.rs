use oauth2::CsrfToken;
use oauth2::PkceCodeChallenge;
use oauth2::RedirectUrl;
use oauth2::Scope;

pub struct AuthorizationRequest {
    pub nonce: String,
    pub pkce_challenge: PkceCodeChallenge,
    pub redirect_uri: RedirectUrl,
    pub scopes: Vec<Scope>,
    pub state: CsrfToken,
}
