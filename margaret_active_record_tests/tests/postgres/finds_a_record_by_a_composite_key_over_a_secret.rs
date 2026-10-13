use zeroize::Zeroizing;

use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret::framework::active_record::secret_text::SecretText;
use margaret_active_record_tests::models::credential_scope::CredentialScope;
use margaret_active_record_tests::models::scoped_credential::ScopedCredential;

use crate::postgres::started_with_models::started_with_models;

#[tokio::test]
async fn finds_a_record_by_a_composite_key_over_a_secret() {
    let started = started_with_models().await;
    let database = started.database.as_ref();

    ScopedCredential {
        scope: CredentialScope::Deploy,
        secret: SecretText::new(Zeroizing::new("deploy key material".to_string())),
        holder: "release".to_string(),
    }
    .insert()
    .run(database)
    .await
    .expect("the scoped credential is inserted");

    assert!(matches!(
        ScopedCredential::query()
            .scope
            .eq(CredentialScope::Deploy)
            .secret
            .eq(SecretText::new(Zeroizing::new("deploy key material".to_string())))
            .find(database)
            .await
            .expect("the scoped credential is read"),
        Lookup::Found(ScopedCredential { holder, .. }) if holder == "release"
    ));
}
