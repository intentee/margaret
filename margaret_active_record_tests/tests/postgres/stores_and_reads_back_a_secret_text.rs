use zeroize::Zeroizing;

use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret::framework::active_record::secret_text::SecretText;
use margaret_active_record_tests::models::credential::Credential;

use crate::postgres::started_with_models::started_with_models;

#[tokio::test]
async fn stores_and_reads_back_a_secret_text() {
    let started = started_with_models().await;
    let database = started.database.as_ref();

    Credential {
        name: "signing".to_string(),
        secret: SecretText::new(Zeroizing::new("private key material".to_string())),
    }
    .insert()
    .run(database)
    .await
    .expect("the credential is inserted");

    assert!(matches!(
        Credential::query()
            .name
            .eq("signing".to_string())
            .find(database)
            .await
            .expect("the credential is read"),
        Lookup::Found(Credential { secret, .. }) if secret.expose() == "private key material"
    ));
}
