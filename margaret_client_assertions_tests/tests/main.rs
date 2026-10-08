#[cfg(feature = "tests_that_use_postgres")]
mod client_assertions_are_first_again_once_expired;
#[cfg(feature = "tests_that_use_postgres")]
mod client_assertions_are_remembered_per_client;
#[cfg(feature = "tests_that_use_postgres")]
mod concurrent_client_assertions_are_first_once;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_remembrance_in_a_database_without_its_table;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_remembrance_the_database_refuses;
