use reqwest::StatusCode;

use margaret_cluster_fixture::routes::public::subject_answer::SubjectAnswer;

use crate::cluster::Cluster;

/// # Panics
///
/// Panics when the route does not admit the bearer token.
pub async fn bearer_subject(cluster: &Cluster, url: String, token: &str) -> String {
    let response = cluster
        .client
        .get(url)
        .bearer_auth(token)
        .send()
        .await
        .expect("the bearer request is answered");

    assert_eq!(response.status(), StatusCode::OK);

    response
        .json::<SubjectAnswer>()
        .await
        .expect("the route answers with the subject")
        .subject
}
