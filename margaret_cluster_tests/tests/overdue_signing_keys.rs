use chrono::Utc;
use serde_json::Value;
use zeroize::Zeroizing;

use margaret::framework::active_record::model::Model;
use margaret::framework::active_record::saving::Saving;
use margaret::framework::active_record::secret_text::SecretText;
use margaret::framework::jwks_roller::jwks_roll_interval::JWKS_ROLL_INTERVAL;
use margaret::framework::registered_claims::numeric_date::NumericDate;
use margaret::framework::signing_keys::signing_key_set::SigningKeySet;
use margaret_cluster_tests::cluster::Cluster;

use crate::signing_key_sets::signing_key_sets;

pub async fn overdue_signing_keys(cluster: &Cluster) {
    let overdue = NumericDate::from(Utc::now() - JWKS_ROLL_INTERVAL).seconds_since_epoch() - 1;

    for key_set in signing_key_sets(cluster).await {
        let mut document: Value =
            serde_json::from_str(key_set.document.expose()).expect("the key set document is JSON");

        document["rolled_at"] = Value::from(overdue);

        assert_eq!(
            SigningKeySet {
                document: SecretText::new(Zeroizing::new(document.to_string())),
                ..key_set
            }
            .save(cluster.database.database.as_ref())
            .await
            .expect("the signing keys become overdue"),
            Saving::Saved
        );
    }
}
