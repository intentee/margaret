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
mod a_replayed_client_assertion_is_refused_on_another_instance;
#[cfg(feature = "tests_that_use_cluster")]
mod a_replayed_code_revokes_its_family_on_every_instance;
#[cfg(feature = "tests_that_use_cluster")]
mod a_replayed_sign_in_callback_is_refused_on_another_instance;
#[cfg(feature = "tests_that_use_cluster")]
mod a_resource_token_is_admitted_on_every_instance;
#[cfg(feature = "tests_that_use_cluster")]
mod a_restarted_instance_remembers_client_assertions_so_a_replay_still_fails;
#[cfg(feature = "tests_that_use_cluster")]
mod a_revocation_on_one_instance_refuses_the_refresh_token_everywhere;
#[cfg(feature = "tests_that_use_cluster")]
mod a_rotation_on_one_instance_supersedes_the_token_on_another;
#[cfg(feature = "tests_that_use_cluster")]
mod a_service_acquires_a_token_through_a_peer;
#[cfg(feature = "tests_that_use_cluster")]
mod a_session_signed_after_a_roll_is_admitted_on_an_instance_started_before_it;
#[cfg(feature = "tests_that_use_cluster")]
mod a_session_started_on_one_instance_refreshes_on_another;
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
mod an_expired_code_is_refused_on_a_peer_so_expiry_cannot_be_dodged_by_switching;
#[cfg(feature = "tests_that_use_cluster")]
mod an_expired_pending_authorization_is_unknown_everywhere;
#[cfg(feature = "tests_that_use_cluster")]
mod an_instance_restarted_after_an_issuer_key_rotation_admits_the_new_key;
#[cfg(feature = "tests_that_use_cluster")]
mod an_upload_on_one_instance_downloads_identically_from_another;
#[cfg(feature = "tests_that_use_cluster")]
mod answers_a_missing_asset_as_not_found;
#[cfg(feature = "tests_that_use_cluster")]
mod any_instance_sweeps_notes_another_wrote_so_a_crashed_writer_leaves_no_garbage;
#[cfg(feature = "tests_that_use_cluster")]
mod assets_are_identical_on_every_instance_so_any_peer_can_serve_a_hashed_url;
#[cfg(feature = "tests_that_use_cluster")]
mod authorizes_consents_and_redeems_on_different_instances;
#[cfg(feature = "tests_that_use_cluster")]
mod challenges_a_notes_caller_without_a_bearer_token;
#[cfg(feature = "tests_that_use_cluster")]
mod chat_messages_posted_on_one_instance_are_listed_on_another;
#[cfg(feature = "tests_that_use_cluster")]
mod client_credentials_introspect_active_on_another_instance;
mod cluster_binary;
mod concludes_a_forward_to_every_named_public_route_with_its_props;
#[cfg(feature = "tests_that_use_cluster")]
mod concurrent_code_redemptions_on_several_instances_admit_one;
#[cfg(feature = "tests_that_use_cluster")]
mod concurrent_presentations_of_one_client_assertion_admit_one;
#[cfg(feature = "tests_that_use_cluster")]
mod concurrent_rotations_on_several_instances_admit_one;
#[cfg(feature = "tests_that_use_cluster")]
mod deletes_a_message_by_its_record;
#[cfg(feature = "tests_that_use_cluster")]
mod deletes_an_upload_by_its_record;
#[cfg(feature = "tests_that_use_cluster")]
mod deletes_the_messages_posted_at_a_moment;
#[cfg(feature = "tests_that_use_cluster")]
mod deletes_the_token_acquisitions_of_an_outcome;
#[cfg(feature = "tests_that_use_cluster")]
mod each_sign_out_route_lands_on_its_own_route;
#[cfg(feature = "tests_that_use_cluster")]
mod every_instance_admits_the_external_issuer_so_a_balancer_may_route_anywhere;
#[cfg(feature = "tests_that_use_cluster")]
mod every_instance_serves_byte_identical_discovery_so_clients_see_one_provider;
#[cfg(feature = "tests_that_use_cluster")]
mod exchanges_an_external_token_on_another_instance;
#[cfg(feature = "tests_that_use_cluster")]
mod fails_a_profile_whose_session_cannot_be_resolved;
#[cfg(feature = "tests_that_use_cluster")]
mod fails_deleting_a_note_the_database_refuses;
#[cfg(feature = "tests_that_use_cluster")]
mod fails_downloading_an_upload_the_database_refuses;
#[cfg(feature = "tests_that_use_cluster")]
mod fails_patching_a_note_the_database_refuses;
#[cfg(feature = "tests_that_use_cluster")]
mod fails_reading_a_note_the_database_refuses;
#[cfg(feature = "tests_that_use_cluster")]
mod fails_viewing_a_note_the_database_refuses;
#[cfg(feature = "tests_that_use_cluster")]
mod instances_started_on_overdue_keys_roll_once;
#[cfg(feature = "tests_that_use_cluster")]
mod instances_started_together_create_one_key_set;
#[cfg(feature = "tests_that_use_cluster")]
mod notes_are_written_read_updated_and_deleted_across_instances;
mod prints_help_without_a_command;
#[cfg(feature = "tests_that_use_cluster")]
mod refuses_a_note_patch_that_is_not_json;
#[cfg(feature = "tests_that_use_cluster")]
mod refuses_a_note_that_is_not_json;
#[cfg(feature = "tests_that_use_cluster")]
mod refuses_a_view_of_a_note_that_does_not_exist;
#[cfg(feature = "tests_that_use_cluster")]
mod refuses_an_upload_beyond_its_limit;
#[cfg(feature = "tests_that_use_cluster")]
mod refuses_deleting_a_note_that_does_not_exist;
#[cfg(feature = "tests_that_use_cluster")]
mod refuses_downloading_an_upload_that_does_not_exist;
#[cfg(feature = "tests_that_use_cluster")]
mod refuses_patching_a_note_that_does_not_exist;
mod refuses_to_seed_without_a_reachable_database;
#[cfg(feature = "tests_that_use_cluster")]
mod refuses_to_serve_when_its_signing_keys_cannot_be_read;
mod refuses_to_serve_without_a_reachable_database;
mod refuses_to_write_a_note_without_a_reachable_database;
mod rejects_a_stored_outcome_no_variant_names;
mod renders_the_schema_of_the_cluster;
#[cfg(feature = "tests_that_use_cluster")]
mod saves_a_token_acquisition_by_its_record;
#[cfg(feature = "tests_that_use_cluster")]
mod updates_the_messages_posted_at_a_moment;
#[cfg(feature = "tests_that_use_cluster")]
mod updates_the_token_acquisitions_of_an_outcome;
#[cfg(feature = "tests_that_use_cluster")]
mod userinfo_answers_for_a_token_issued_on_another_instance_without_sticky_routing;
