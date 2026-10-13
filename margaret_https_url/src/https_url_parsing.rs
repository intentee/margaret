use crate::https_url::HttpsUrl;
use crate::https_url_rejection::HttpsUrlRejection;

#[derive(Debug, Eq, PartialEq)]
pub enum HttpsUrlParsing {
    Accepted(HttpsUrl),
    Rejected(HttpsUrlRejection),
}
