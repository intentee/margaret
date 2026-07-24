#[cfg(feature = "tests_that_use_postgres")]
mod applies_and_round_trips;
#[cfg(feature = "tests_that_use_postgres")]
mod composite_and_non_pk_foreign_keys;
#[cfg(feature = "tests_that_use_postgres")]
mod foreign_key_cascade_deletes_dependent_rows;
#[cfg(feature = "tests_that_use_postgres")]
mod foreign_key_set_null_on_delete;
#[cfg(feature = "tests_that_use_postgres")]
mod structure_matches_the_declared_schema;
#[cfg(feature = "tests_that_use_postgres")]
mod uuidv7_default_populates_the_primary_key;
