use reqwest::RequestBuilder;
use reqwest::StatusCode;

pub async fn instance_answers(request: RequestBuilder) -> bool {
    request
        .send()
        .await
        .is_ok_and(|response| response.status() == StatusCode::OK)
}
