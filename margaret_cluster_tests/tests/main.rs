#[cfg(feature = "tests_that_use_cluster")]
mod a_chat_session_closes_on_stop_and_reconnects_to_a_peer;
#[cfg(feature = "tests_that_use_cluster")]
mod a_console_command_write_is_visible_on_every_instance;
#[cfg(feature = "tests_that_use_cluster")]
mod a_crashed_instance_restarted_admits_earlier_tokens;
#[cfg(feature = "tests_that_use_cluster")]
mod a_decided_consent_is_unknown_on_another_instance;
#[cfg(feature = "tests_that_use_cluster")]
mod a_forward_renders_a_note_written_on_another_instance;
#[cfg(feature = "tests_that_use_cluster")]
mod a_refresh_token_signed_after_a_roll_mints_on_an_instance_started_before_it;
#[cfg(feature = "tests_that_use_cluster")]
mod a_replayed_client_assertion_is_refused_on_another_instance;
#[cfg(feature = "tests_that_use_cluster")]
mod a_replayed_code_revokes_its_family_on_every_instance;
#[cfg(feature = "tests_that_use_cluster")]
mod a_replayed_sign_in_callback_is_refused_on_another_instance;
#[cfg(feature = "tests_that_use_cluster")]
mod a_resource_token_is_admitted_on_every_instance;
#[cfg(feature = "tests_that_use_cluster")]
mod a_restarted_instance_remembers_client_assertions;
#[cfg(feature = "tests_that_use_cluster")]
mod a_revocation_on_one_instance_refuses_the_refresh_token_everywhere;
#[cfg(feature = "tests_that_use_cluster")]
mod a_rotation_on_one_instance_supersedes_the_token_on_another;
#[cfg(feature = "tests_that_use_cluster")]
mod a_service_acquires_a_token_through_a_peer;
#[cfg(feature = "tests_that_use_cluster")]
mod a_session_refresh_token_from_one_instance_mints_on_another;
#[cfg(feature = "tests_that_use_cluster")]
mod a_sign_in_begun_on_one_instance_completes_on_another;
#[cfg(feature = "tests_that_use_cluster")]
mod a_sign_in_transaction_crosses_a_key_roll;
#[cfg(feature = "tests_that_use_cluster")]
mod a_sign_out_on_one_instance_is_refused_everywhere;
#[cfg(feature = "tests_that_use_cluster")]
mod a_terminated_instance_exits_cleanly_while_peers_serve;
#[cfg(feature = "tests_that_use_cluster")]
mod a_view_shows_a_note_written_on_another_instance;
#[cfg(feature = "tests_that_use_cluster")]
mod an_expired_code_is_refused_everywhere;
#[cfg(feature = "tests_that_use_cluster")]
mod an_expired_pending_authorization_is_unknown_everywhere;
#[cfg(feature = "tests_that_use_cluster")]
mod an_instance_restarted_after_an_issuer_key_rotation_admits_the_new_key;
#[cfg(feature = "tests_that_use_cluster")]
mod an_upload_on_one_instance_downloads_identically_from_another;
#[cfg(feature = "tests_that_use_cluster")]
mod any_instance_sweeps_notes_another_wrote;
#[cfg(feature = "tests_that_use_cluster")]
mod assert_verified_by_instance_keys;
#[cfg(feature = "tests_that_use_cluster")]
mod assets_are_identical_on_every_instance;
#[cfg(feature = "tests_that_use_cluster")]
mod authorizes_consents_and_redeems_on_different_instances;
#[cfg(feature = "tests_that_use_cluster")]
mod chat_messages_posted_on_one_instance_are_listed_on_another;
#[cfg(feature = "tests_that_use_cluster")]
mod client_credentials_introspect_active_on_another_instance;
#[cfg(feature = "tests_that_use_cluster")]
mod cluster_binary;
#[cfg(feature = "tests_that_use_cluster")]
mod concurrent_code_redemptions_on_several_instances_admit_one;
#[cfg(feature = "tests_that_use_cluster")]
mod concurrent_presentations_of_one_client_assertion_admit_one;
#[cfg(feature = "tests_that_use_cluster")]
mod concurrent_rotations_on_several_instances_admit_one;
#[cfg(feature = "tests_that_use_cluster")]
mod discovery_is_identical_everywhere;
#[cfg(feature = "tests_that_use_cluster")]
mod every_instance_admits_the_external_issuer;
#[cfg(feature = "tests_that_use_cluster")]
mod exchanges_an_external_token_on_another_instance;
#[cfg(feature = "tests_that_use_cluster")]
mod expire_framework_rows;
#[cfg(feature = "tests_that_use_cluster")]
mod instances_started_on_overdue_keys_roll_once;
#[cfg(feature = "tests_that_use_cluster")]
mod instances_started_together_create_one_key_set;
#[cfg(feature = "tests_that_use_cluster")]
mod listed_messages;
#[cfg(feature = "tests_that_use_cluster")]
mod note_written;
#[cfg(feature = "tests_that_use_cluster")]
mod notes_are_written_read_updated_and_deleted_across_instances;
#[cfg(feature = "tests_that_use_cluster")]
mod overdue_signing_keys;
#[cfg(feature = "tests_that_use_cluster")]
mod poll_until;
#[cfg(feature = "tests_that_use_cluster")]
mod profile_status;
#[cfg(feature = "tests_that_use_cluster")]
mod signing_key_generations;
#[cfg(feature = "tests_that_use_cluster")]
mod stored_message;
#[cfg(feature = "tests_that_use_cluster")]
mod stored_note;
#[cfg(feature = "tests_that_use_cluster")]
mod userinfo_answers_on_another_instance;
