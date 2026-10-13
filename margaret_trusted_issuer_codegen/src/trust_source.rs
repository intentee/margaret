use margaret_https_url::https_url::HttpsUrl;

pub(crate) enum TrustSource {
    Discovery,
    JwksEndpoint { jwks_uri: HttpsUrl },
}
