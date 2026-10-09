use std::sync::Arc;

use margaret_database_tests::started_database::StartedDatabase;
use margaret_example::commands::seed::Seed;
use margaret_example::margaret::schema::SCHEMA;
use margaret_example::system_clock::SystemClock;

/// # Panics
///
/// Panics when the blog database cannot be started, migrated, or seeded.
pub async fn seeded_blog() -> StartedDatabase {
    let started = StartedDatabase::with_schema(&SCHEMA).await;

    Seed::create(Arc::new(SystemClock), Arc::clone(&started.database))
        .expect("the seed command is constructed")
        .run()
        .await
        .expect("the blog is seeded");

    started
}
