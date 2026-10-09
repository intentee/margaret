use margaret::framework::macros::postgres_database;

#[postgres_database(
    url_from = "FIXTURE_DATABASE_URL",
    max_connections_from = "FIXTURE_DATABASE_MAX_CONNECTIONS"
)]
pub struct FixtureDatabase;
