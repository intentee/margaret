use margaret::framework::active_record::model::Model;
use margaret::framework::database::database::Database;
use margaret_active_record_tests::models::measurement::Measurement;

async fn attempted(database: &Database) {
    let _ = Measurement::query()
        .id
        .eq(1)
        .when(|measurement| measurement.flag.above(false))
        .find(database)
        .await;
}

fn main() {
    drop(attempted);
}
