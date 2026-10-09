use reqwest::StatusCode;
use url::Url;

use crate::cluster::Cluster;
use crate::code_grant::code_grant;
use crate::issued_tokens::IssuedTokens;
use crate::partner_token::partner_token;

/// # Panics
///
/// Panics when the code is not exchanged for tokens.
pub async fn partner_tokens(cluster: &Cluster, token_at: &Url, code: &str) -> IssuedTokens {
    let response = partner_token(cluster, token_at, &code_grant(code)).await;

    assert_eq!(response.status(), StatusCode::OK);

    response
        .json()
        .await
        .expect("the token response carries the tokens")
}
