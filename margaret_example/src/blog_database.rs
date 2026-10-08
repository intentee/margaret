use margaret::framework::macros::postgres_database;

#[postgres_database(url_from = "MARGARET_EXAMPLE_DATABASE_URL")]
pub struct BlogDatabase;
