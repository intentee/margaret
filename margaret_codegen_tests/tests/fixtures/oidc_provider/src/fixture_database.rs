use margaret::framework::macros::postgres_database;

#[postgres_database(url_from = "FIXTURE_DATABASE_URL")]
pub struct FixtureDatabase;
