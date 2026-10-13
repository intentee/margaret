use reqwest::RequestBuilder;
use reqwest::StatusCode;

#[derive(Debug, Eq, PartialEq)]
pub struct Answer {
    pub body: String,
    pub status: StatusCode,
}

impl Answer {
    /// # Panics
    ///
    /// Panics when the library does not answer the request in full.
    pub async fn of(request: RequestBuilder) -> Self {
        let response = request.send().await.expect("the library answers");
        let status = response.status();

        Self {
            body: response.text().await.expect("the answer is read in full"),
            status,
        }
    }
}
