use margaret_jwks_roller_server::jwks_roller_server_error::JwksRollerServerError;

#[tokio::test]
async fn jwks_roller_server_error_document_serialization_reports_its_source() {
    let source = serde_json::from_str::<serde_json::Value>("not json")
        .expect_err("the fixture is not valid json");
    let source_message = source.to_string();

    let error = JwksRollerServerError::DocumentSerialization(source);

    assert_eq!(
        error.to_string(),
        format!("the published jwks document could not be serialized: {source_message}")
    );
}
