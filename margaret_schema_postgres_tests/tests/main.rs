#[cfg(feature = "tests_that_use_postgres")]
mod applies_and_round_trips;
#[cfg(feature = "tests_that_use_postgres")]
mod check_constraints_reject_invalid_values;
#[cfg(feature = "tests_that_use_postgres")]
mod composite_foreign_key_rejects_an_orphan_row;
#[cfg(feature = "tests_that_use_postgres")]
mod foreign_key_cascade_deletes_dependent_rows;
#[cfg(feature = "tests_that_use_postgres")]
mod numeric_and_float_columns_round_trip;
#[cfg(feature = "tests_that_use_postgres")]
mod structure_matches_the_declared_schema;
#[cfg(feature = "tests_that_use_postgres")]
mod uuidv7_default_populates_the_primary_key;
