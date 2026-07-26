use crate::request_error::RequestError;
use crate::response::Response;

#[must_use]
pub fn respond_with_user_error(error: anyhow::Error) -> Response {
    eprintln!("{}", RequestError::UserError(error));

    Response::internal_server_error()
}

#[cfg(test)]
mod tests {
    use http_body_util::BodyExt;

    use super::respond_with_user_error;

    #[tokio::test]
    async fn renders_a_generic_internal_server_error() {
        let response = respond_with_user_error(anyhow::anyhow!("the datastore is unavailable"));

        assert_eq!(response.status(), 500);

        let body = response
            .into_http()
            .into_body()
            .collect()
            .await
            .expect("the body collects")
            .to_bytes();

        assert_eq!(body.as_ref(), b"Internal Server Error");
    }
}
