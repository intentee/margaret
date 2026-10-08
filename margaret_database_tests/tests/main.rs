#[cfg(feature = "tests_that_use_postgres")]
mod aborts_one_of_two_conflicting_serializable_transactions;
#[cfg(feature = "tests_that_use_postgres")]
mod commits_the_writes_of_a_transaction;
#[cfg(feature = "tests_that_use_postgres")]
mod create_probes;
#[cfg(feature = "tests_that_use_postgres")]
mod discards_the_writes_of_a_dropped_transaction;
#[cfg(feature = "tests_that_use_postgres")]
mod executes_statements_through_the_pool;
#[cfg(feature = "tests_that_use_postgres")]
mod insert_probe;
#[cfg(feature = "tests_that_use_postgres")]
mod isolates_a_repeatable_read_transaction_from_later_commits;
#[cfg(feature = "tests_that_use_postgres")]
mod probe_amounts;
#[cfg(feature = "tests_that_use_postgres")]
mod probes;
#[cfg(feature = "tests_that_use_postgres")]
mod reads_an_optional_row;
#[cfg(feature = "tests_that_use_postgres")]
mod reads_an_optional_row_inside_a_transaction;
#[cfg(feature = "tests_that_use_postgres")]
mod reads_exactly_one_row;
#[cfg(feature = "tests_that_use_postgres")]
mod reads_exactly_one_row_inside_a_transaction;
#[cfg(feature = "tests_that_use_postgres")]
mod releases_lock_waiters_together;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_conflict_with_a_row_hidden_by_row_level_security;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_single_row_statement_that_returns_no_row;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_statement_that_cannot_be_executed;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_statement_that_cannot_be_prepared;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_transaction_that_cannot_begin;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_transaction_that_cannot_commit;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_transaction_that_cannot_roll_back;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_an_unreachable_database;
#[cfg(feature = "tests_that_use_postgres")]
mod revokes_a_privilege_while_a_statement_waits_on_a_lock;
#[cfg(feature = "tests_that_use_postgres")]
mod rolls_back_the_writes_of_a_transaction;
#[cfg(feature = "tests_that_use_postgres")]
mod select_probe_amounts;
#[cfg(feature = "tests_that_use_postgres")]
mod update_probe_amount;
