use oauth2::CsrfToken;
use oauth2::RedirectUrl;
use oauth2::Scope;

use margaret_oauth_vocabulary::code_challenge::CodeChallenge;

pub struct AuthorizationRequest {
    pub code_challenge: CodeChallenge,
    pub nonce: String,
    pub redirect_uri: RedirectUrl,
    pub scopes: Vec<Scope>,
    pub state: CsrfToken,
}
