use reqwest::Response;
use reqwest::header::COOKIE;
use url::Position;
use url::Url;

use crate::begun_sign_in::BegunSignIn;
use crate::cluster::Cluster;

/// # Panics
///
/// Panics when the callback cannot be sent to the instance.
pub async fn completed_sign_in(
    cluster: &Cluster,
    begun: &BegunSignIn,
    callback: &Url,
    public: &Url,
) -> Response {
    cluster
        .client
        .get(
            public
                .join(&callback[Position::BeforePath..])
                .expect("the callback joins the instance URL"),
        )
        .header(COOKIE, &begun.transaction_cookies)
        .send()
        .await
        .expect("the sign-in callback is answered")
}
