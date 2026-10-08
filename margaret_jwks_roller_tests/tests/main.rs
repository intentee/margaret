#[cfg(feature = "tests_that_use_postgres")]
mod concurrent_due_rolls_replace_the_keys_once;
#[cfg(feature = "tests_that_use_postgres")]
mod concurrent_roll_rounds_never_diverge;
#[cfg(feature = "tests_that_use_postgres")]
mod concurrent_starts_on_an_empty_store_create_one_secret;
mod roller_error_key_generation_reports_its_source;
mod signing_key_retention_outlasts_every_signed_token_lifetime;
#[cfg(feature = "tests_that_use_postgres")]
mod synchronizer_adopts_a_newer_generation_without_writing;
#[cfg(feature = "tests_that_use_postgres")]
mod synchronizer_adopts_the_keys_another_instance_created;
#[cfg(feature = "tests_that_use_postgres")]
mod synchronizer_creates_fresh_keys_in_an_empty_store;
#[cfg(feature = "tests_that_use_postgres")]
mod synchronizer_rejects_a_document_whose_keys_cannot_be_restored;
#[cfg(feature = "tests_that_use_postgres")]
mod synchronizer_rejects_a_malformed_document;
#[cfg(feature = "tests_that_use_postgres")]
mod synchronizer_rejects_signing_keys_stored_by_an_earlier_release;
#[cfg(feature = "tests_that_use_postgres")]
mod synchronizer_rejects_stored_keys_of_another_curve;
#[cfg(feature = "tests_that_use_postgres")]
mod synchronizer_reports_a_failed_creation;
#[cfg(feature = "tests_that_use_postgres")]
mod synchronizer_reports_a_failed_replacement;
#[cfg(feature = "tests_that_use_postgres")]
mod synchronizer_reports_a_key_generation_error_when_no_rsa_key_can_be_made;
#[cfg(feature = "tests_that_use_postgres")]
mod synchronizer_reports_a_regressed_store;
#[cfg(feature = "tests_that_use_postgres")]
mod synchronizer_reports_a_roll_error_when_no_rsa_key_can_be_made;
#[cfg(feature = "tests_that_use_postgres")]
mod synchronizer_reports_a_superseding_generation_it_cannot_observe;
#[cfg(feature = "tests_that_use_postgres")]
mod synchronizer_reports_an_outage_after_a_conflicting_creation;
#[cfg(feature = "tests_that_use_postgres")]
mod synchronizer_reports_an_outage_after_a_superseded_roll;
#[cfg(feature = "tests_that_use_postgres")]
mod synchronizer_reports_an_unreachable_store;
#[cfg(feature = "tests_that_use_postgres")]
mod synchronizer_reports_created_keys_it_cannot_observe;
#[cfg(feature = "tests_that_use_postgres")]
mod synchronizer_reports_keys_forked_at_the_held_generation;
#[cfg(feature = "tests_that_use_postgres")]
mod synchronizer_reports_keys_vanished_during_a_roll;
#[cfg(feature = "tests_that_use_postgres")]
mod synchronizer_reports_keys_vanished_from_the_store;
#[cfg(feature = "tests_that_use_postgres")]
mod synchronizer_rolls_overdue_keys_once;
#[cfg(feature = "tests_that_use_postgres")]
mod synchronizer_writes_nothing_before_the_roll_is_due;
