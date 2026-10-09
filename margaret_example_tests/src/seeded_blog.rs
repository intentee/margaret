use std::sync::Arc;

use margaret_database_tests::apply_schema::apply_schema;
use margaret_database_tests::started_database::StartedDatabase;
use margaret_example::commands::seed::Seed;
use margaret_example::margaret::schema::schema;
use margaret_example::system_clock::SystemClock;

/// # Panics
///
/// Panics when the blog database cannot be started, migrated, or seeded.
pub async fn seeded_blog() -> StartedDatabase {
    let started = StartedDatabase::start().await;

    apply_schema(&started.database, &schema()).await;
    Seed::create(Arc::new(SystemClock), Arc::clone(&started.database))
        .expect("the seed command is constructed")
        .run()
        .await
        .expect("the blog is seeded");

    started
}
