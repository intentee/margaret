use std::fmt::Display;

use crate::response::Response;

#[must_use]
pub fn reported_unavailability(unavailability: &impl Display) -> Response {
    eprintln!("margaret_http: a dependency of the request is unavailable: {unavailability}");

    Response::text(503, "Service Unavailable")
}

#[cfg(test)]
mod tests {
    use http_body_util::BodyExt;

    use super::reported_unavailability;

    #[tokio::test]
    async fn answers_with_a_generic_body_so_the_failure_never_reaches_the_client() {
        let response = reported_unavailability(
            &"error sending request for url (https://issuer.internal/refresh)",
        )
        .into_http();

        assert_eq!(response.status().as_u16(), 503);
        assert_eq!(
            response
                .into_body()
                .collect()
                .await
                .expect("the buffered body is collected")
                .to_bytes()
                .as_ref(),
            b"Service Unavailable"
        );
    }
}
