#[cfg(feature = "tests_that_use_postgres")]
mod at_epoch_seconds;
#[cfg(feature = "tests_that_use_postgres")]
mod attaches_nothing_to_no_records;
#[cfg(feature = "tests_that_use_postgres")]
mod attaches_relations_to_records;
#[cfg(feature = "tests_that_use_postgres")]
mod binds_a_record_by_its_primary_key;
#[cfg(feature = "tests_that_use_postgres")]
mod binds_a_shape_by_the_primary_key_of_its_model;
#[cfg(feature = "tests_that_use_postgres")]
mod binds_no_record_for_a_malformed_primary_key;
#[cfg(feature = "tests_that_use_postgres")]
mod binds_no_record_for_an_unknown_primary_key;
#[cfg(feature = "tests_that_use_postgres")]
mod created_article;
#[cfg(feature = "tests_that_use_postgres")]
mod created_author;
#[cfg(feature = "tests_that_use_postgres")]
mod created_node;
#[cfg(feature = "tests_that_use_postgres")]
mod created_note;
#[cfg(feature = "tests_that_use_postgres")]
mod creates_a_record_whose_guard_holds;
#[cfg(feature = "tests_that_use_postgres")]
mod creates_a_record_with_its_database_defaults;
#[cfg(feature = "tests_that_use_postgres")]
mod deletes_a_record_by_its_primary_key;
#[cfg(feature = "tests_that_use_postgres")]
mod deletes_a_unique_row;
#[cfg(feature = "tests_that_use_postgres")]
mod deletes_the_rows_of_a_branching_prefix;
#[cfg(feature = "tests_that_use_postgres")]
mod deletes_the_rows_of_a_range;
#[cfg(feature = "tests_that_use_postgres")]
mod deletes_the_rows_of_an_index_prefix;
#[cfg(feature = "tests_that_use_postgres")]
mod describes_a_key_by_its_primary_key;
#[cfg(feature = "tests_that_use_postgres")]
mod finds_a_record_by_a_composite_primary_key;
#[cfg(feature = "tests_that_use_postgres")]
mod finds_a_record_by_a_unique_field;
#[cfg(feature = "tests_that_use_postgres")]
mod finds_a_record_by_its_primary_key;
#[cfg(feature = "tests_that_use_postgres")]
mod ignores_an_insert_of_a_present_record;
#[cfg(feature = "tests_that_use_postgres")]
mod inserted_counter;
#[cfg(feature = "tests_that_use_postgres")]
mod inserts_a_record_that_is_not_present;
#[cfg(feature = "tests_that_use_postgres")]
mod inserts_a_record_whose_guard_holds;
#[cfg(feature = "tests_that_use_postgres")]
mod keeps_a_row_whose_upsert_condition_fails;
#[cfg(feature = "tests_that_use_postgres")]
mod leaves_a_unique_row_whose_condition_fails_unchanged;
#[cfg(feature = "tests_that_use_postgres")]
mod loads_a_belongs_to_relation;
#[cfg(feature = "tests_that_use_postgres")]
mod loads_a_child_of_a_joined_parent;
#[cfg(feature = "tests_that_use_postgres")]
mod loads_a_page_of_shapes;
#[cfg(feature = "tests_that_use_postgres")]
mod loads_a_present_optional_parent;
#[cfg(feature = "tests_that_use_postgres")]
mod loads_a_present_single_child;
#[cfg(feature = "tests_that_use_postgres")]
mod loads_an_absent_optional_parent;
#[cfg(feature = "tests_that_use_postgres")]
mod loads_an_absent_single_child;
#[cfg(feature = "tests_that_use_postgres")]
mod loads_an_optional_parent_of_an_optional_parent;
#[cfg(feature = "tests_that_use_postgres")]
mod loads_nested_children;
#[cfg(feature = "tests_that_use_postgres")]
mod loads_the_family_of_a_present_parent;
#[cfg(feature = "tests_that_use_postgres")]
mod matches_a_row_above_a_bound;
#[cfg(feature = "tests_that_use_postgres")]
mod matches_a_row_at_an_inclusive_lower_bound;
#[cfg(feature = "tests_that_use_postgres")]
mod matches_a_row_at_an_inclusive_upper_bound;
#[cfg(feature = "tests_that_use_postgres")]
mod matches_a_row_below_a_bound;
#[cfg(feature = "tests_that_use_postgres")]
mod matches_a_row_satisfying_both_conditions;
#[cfg(feature = "tests_that_use_postgres")]
mod matches_a_row_whose_nullable_field_is_null;
#[cfg(feature = "tests_that_use_postgres")]
mod narrows_a_branching_index_prefix;
#[cfg(feature = "tests_that_use_postgres")]
mod note_bodies;
#[cfg(feature = "tests_that_use_postgres")]
mod note_lookup;
#[cfg(feature = "tests_that_use_postgres")]
mod orders_a_range_in_descending_order;
#[cfg(feature = "tests_that_use_postgres")]
mod orders_the_rows_of_an_index_prefix;
#[cfg(feature = "tests_that_use_postgres")]
mod pages_records_along_an_ordering_index;
#[cfg(feature = "tests_that_use_postgres")]
mod pages_records_in_descending_order;
#[cfg(feature = "tests_that_use_postgres")]
mod posted_message;
#[cfg(feature = "tests_that_use_postgres")]
mod profiled;
#[cfg(feature = "tests_that_use_postgres")]
mod reads_the_payload_of_a_json_column;
#[cfg(feature = "tests_that_use_postgres")]
mod reads_the_primary_key_a_key_references;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_a_creation_whose_guard_fails;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_an_insert_whose_guard_fails;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_binding_the_database_refuses;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_condition_that_cannot_be_encoded;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_deadlock;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_draft_that_cannot_be_encoded;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_duplicate_insert_as_a_unique_violation;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_malformed_column;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_missing_parent_as_a_foreign_key_violation;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_missing_record;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_missing_shape;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_price_beyond_its_precision_as_out_of_range;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_record_that_cannot_be_encoded;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_record_that_vanished_before_attaching;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_scan_that_cannot_be_encoded;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_serialization_failure;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_statement_the_database_refuses;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_value_below_its_minimum_as_a_check_violation;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_an_unavailable_database;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_an_unknown_enum_variant;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_children_that_cannot_be_read;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_deleting_a_missing_row;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_deleting_a_record_that_is_gone;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_malformed_json;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_saving_a_missing_record;
#[cfg(feature = "tests_that_use_postgres")]
mod resumes_a_page_from_its_cursor;
#[cfg(feature = "tests_that_use_postgres")]
mod saves_a_present_record;
#[cfg(feature = "tests_that_use_postgres")]
mod skips_a_row_at_an_exclusive_lower_bound;
#[cfg(feature = "tests_that_use_postgres")]
mod skips_a_row_matching_a_negated_condition;
#[cfg(feature = "tests_that_use_postgres")]
mod skips_a_row_whose_nullable_field_is_set;
#[cfg(feature = "tests_that_use_postgres")]
mod skips_the_family_of_an_absent_parent;
#[cfg(feature = "tests_that_use_postgres")]
mod started_with_models;
#[cfg(feature = "tests_that_use_postgres")]
mod stores_and_reads_back_a_secret_text;
#[cfg(feature = "tests_that_use_postgres")]
mod stores_every_scalar_column_type;
#[cfg(feature = "tests_that_use_postgres")]
mod stores_optional_values;
#[cfg(feature = "tests_that_use_postgres")]
mod streams_the_records_of_a_range_in_batches;
#[cfg(feature = "tests_that_use_postgres")]
mod takes_the_payload_of_a_json_column;
#[cfg(feature = "tests_that_use_postgres")]
mod takes_the_primary_key_a_key_references;
#[cfg(feature = "tests_that_use_postgres")]
mod translated;
#[cfg(feature = "tests_that_use_postgres")]
mod truncates_children_beyond_their_limit;
#[cfg(feature = "tests_that_use_postgres")]
mod unserializable_document;
#[cfg(feature = "tests_that_use_postgres")]
mod updates_a_unique_row;
#[cfg(feature = "tests_that_use_postgres")]
mod updates_the_rows_of_a_branching_prefix;
#[cfg(feature = "tests_that_use_postgres")]
mod updates_the_rows_of_a_range;
#[cfg(feature = "tests_that_use_postgres")]
mod updates_the_rows_of_an_index_prefix;
#[cfg(feature = "tests_that_use_postgres")]
mod upserts_the_excluded_value;
#[cfg(feature = "tests_that_use_postgres")]
mod upserts_the_greatest_value;
