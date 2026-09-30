use serde::de::DeserializeOwned;
use validator::Validate;

use margaret_http::body_limit::BodyLimit;
use margaret_http::body_reading::BodyReading;
use margaret_http::read_form_fields::read_form_fields;
use margaret_http::request::Request;
use margaret_http::request_body::RequestBody;
use margaret_validation::validate::validate;
use margaret_validation::validation_result::ValidationResult;

const FORM_LIMIT: BodyLimit = BodyLimit::new(16_384);

pub async fn form_of<TForm: DeserializeOwned + Validate>(
    request: &Request,
    body: RequestBody,
) -> ValidationResult<TForm> {
    match read_form_fields(request, body, FORM_LIMIT).await {
        BodyReading::Read(fields) => validate(&fields),
        BodyReading::Rejected(rejection) => panic!("the test sends a readable form: {rejection}"),
    }
}
