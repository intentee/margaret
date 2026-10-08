use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret_active_record_tests::models::measurement::Measurement;

use crate::started_with_models::started_with_models;

#[tokio::test]
async fn stores_every_scalar_column_type() {
    let started = started_with_models().await;
    let database = started.database.as_ref();
    let measurement = Measurement {
        id: 7,
        small: -3,
        ratio: 0.5,
        precise: 2.25,
        flag: true,
    };

    measurement
        .insert()
        .run(database)
        .await
        .expect("the measurement is inserted");

    assert_eq!(
        Measurement::query()
            .id
            .eq(7)
            .find(database)
            .await
            .expect("the measurement is read"),
        Lookup::Found(measurement)
    );
}
