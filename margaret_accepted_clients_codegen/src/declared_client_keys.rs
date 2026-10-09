use margaret_https_url::https_url::HttpsUrl;
use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;

#[derive(Debug)]
pub enum DeclaredClientKeys {
    Own,
    Published {
        jwks_uri: HttpsUrl,
        signing: JwsAlgorithm,
    },
}
