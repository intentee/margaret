#[cfg(feature = "tests_that_use_postgres")]
mod a_rotation_waiting_on_its_token_honors_a_revocation_committed_meanwhile;
#[cfg(feature = "tests_that_use_postgres")]
mod code_redemption_finds_no_unknown_code;
#[cfg(feature = "tests_that_use_postgres")]
mod concurrent_code_redemptions_admit_one;
#[cfg(feature = "tests_that_use_postgres")]
mod concurrent_pending_authorization_takes_admit_one;
#[cfg(feature = "tests_that_use_postgres")]
mod concurrent_refresh_rotations_admit_one;
#[cfg(feature = "tests_that_use_postgres")]
mod holding_a_pending_authorization_sweeps_expired_ones;
#[cfg(feature = "tests_that_use_postgres")]
mod issuing_a_code_sweeps_expired_codes;
#[cfg(feature = "tests_that_use_postgres")]
mod issuing_a_code_sweeps_expired_redeemed_codes;
#[cfg(feature = "tests_that_use_postgres")]
mod opened_refresh_family_resolves_its_first_token;
#[cfg(feature = "tests_that_use_postgres")]
mod opening_a_family_whose_first_token_is_refused_leaves_no_family;
#[cfg(feature = "tests_that_use_postgres")]
mod opening_a_refresh_family_sweeps_expired_families;
#[cfg(feature = "tests_that_use_postgres")]
mod opening_a_refresh_family_sweeps_expired_revocations;
#[cfg(feature = "tests_that_use_postgres")]
mod pending_authorization_take_finds_no_unknown_authorization;
#[cfg(feature = "tests_that_use_postgres")]
mod refresh_family_revoked_before_opening_never_opens;
#[cfg(feature = "tests_that_use_postgres")]
mod refresh_rotation_finds_no_unknown_token;
#[cfg(feature = "tests_that_use_postgres")]
mod refresh_token_lookup_finds_no_unknown_token;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_code_issued_in_a_database_without_its_tables;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_code_redeemed_in_a_database_without_its_tables;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_code_the_database_refuses_to_issue;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_lookup_whose_family_revocation_cannot_be_read;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_malformed_stored_grant;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_pending_authorization_held_in_a_database_without_its_tables;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_pending_authorization_taken_from_a_database_without_its_tables;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_pending_authorization_the_database_refuses_to_hold;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_refresh_family_opened_in_a_database_without_its_tables;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_refresh_family_revoked_in_a_database_without_its_tables;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_refresh_family_the_database_refuses_to_open;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_refresh_token_lookup_in_a_database_without_its_tables;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_refresh_token_rotated_in_a_database_without_its_tables;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_rotation_the_database_refuses;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_expired_revocations_the_database_refuses_to_sweep;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_malformed_stored_scopes;
#[cfg(feature = "tests_that_use_postgres")]
mod revoked_refresh_family_neither_resolves_nor_rotates;
#[cfg(feature = "tests_that_use_postgres")]
mod revoking_an_unopened_family_bars_it_for_the_refresh_family_lifetime;
#[cfg(feature = "tests_that_use_postgres")]
mod rotating_a_refresh_token_sweeps_expired_families;
#[cfg(feature = "tests_that_use_postgres")]
mod rotating_a_superseded_token_of_a_revoked_family_finds_it_unknown;
#[cfg(feature = "tests_that_use_postgres")]
mod rotating_a_token_whose_successor_is_refused_keeps_it_current;
