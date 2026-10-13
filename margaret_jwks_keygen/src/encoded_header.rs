use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;
use serde_json::Map;
use serde_json::Value;

use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jws_verification::key_id::KeyId;

pub(crate) fn encoded_header(algorithm: JwsAlgorithm, kid: &KeyId, jwt_type: JwtType) -> String {
    let mut header = Map::new();

    header.insert(
        "alg".to_string(),
        Value::String(algorithm.wire_name().to_string()),
    );
    header.insert("kid".to_string(), Value::String(kid.as_str().to_string()));
    header.insert(
        "typ".to_string(),
        Value::String(jwt_type.wire_name().to_string()),
    );

    Base64UrlUnpadded::encode_string(Value::Object(header).to_string().as_bytes())
}
