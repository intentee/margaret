#[cfg(feature = "tests_that_use_postgres")]
mod jwks_roller_adopts_the_roll_of_a_peer_once_its_keys_are_due;
#[cfg(feature = "tests_that_use_postgres")]
mod jwks_roller_publishes_the_document_of_its_secret_when_created;
#[cfg(feature = "tests_that_use_postgres")]
mod jwks_roller_reports_an_unreachable_storage_when_created;
#[cfg(feature = "tests_that_use_postgres")]
mod jwks_roller_restores_the_stored_keys_across_a_restart;
mod jwks_roller_server_error_document_serialization_reports_its_source;
#[cfg(feature = "tests_that_use_postgres")]
mod jwks_roller_stops_when_its_store_fails;
