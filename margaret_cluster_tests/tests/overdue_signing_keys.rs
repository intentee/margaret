use chrono::Utc;

use margaret::framework::jwks_roller::jwks_roll_interval::JWKS_ROLL_INTERVAL;
use margaret::framework::model::qualified_framework_table::qualified_framework_table;
use margaret::framework::registered_claims::numeric_date::NumericDate;
use margaret_cluster_tests::cluster::Cluster;

pub async fn overdue_signing_keys(cluster: &Cluster) {
    let overdue = NumericDate::from(Utc::now() - JWKS_ROLL_INTERVAL).seconds_since_epoch() - 1;

    cluster
        .database
        .database
        .client()
        .await
        .expect("a connection is checked out")
        .execute(
            &format!(
                "UPDATE {} SET document = jsonb_set(document::jsonb, '{{rolled_at}}', to_jsonb($1::bigint))::text",
                qualified_framework_table("signing_key_sets")
            ),
            &[&overdue],
        )
        .await
        .expect("the signing keys become overdue");
}
