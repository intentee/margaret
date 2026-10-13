use thiserror::Error;

use margaret::framework::active_record::active_record_error::ActiveRecordError;
use margaret_https_url::https_url_rejection::HttpsUrlRejection;

#[derive(Debug, Error)]
pub enum SessionsError {
    #[error("the session cookie domain '{domain}' is not a domain name")]
    CookieDomainNotADomain { domain: String },

    #[error("the session cookie domain '{domain}' is malformed: {source}")]
    MalformedCookieDomain {
        domain: String,
        #[source]
        source: url::ParseError,
    },

    #[error("the session refresh url is not usable: {0}")]
    MalformedRefreshUrl(HttpsUrlRejection),

    #[error("the sessions that expired cannot be swept: {0}")]
    SweepSessions(#[source] ActiveRecordError),

    #[error("the session cannot be stored: {0}")]
    OpenSession(#[source] ActiveRecordError),

    #[error("the session of a presented secret cannot be looked up: {0}")]
    FindSession(#[source] ActiveRecordError),

    #[error("the session of a presented secret cannot be forgotten: {0}")]
    ForgetSession(#[source] ActiveRecordError),
}
