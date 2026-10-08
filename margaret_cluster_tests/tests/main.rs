#[cfg(feature = "tests_that_use_cluster")]
mod a_decided_consent_is_unknown_on_another_instance;
#[cfg(feature = "tests_that_use_cluster")]
mod a_replayed_code_revokes_its_family_on_every_instance;
#[cfg(feature = "tests_that_use_cluster")]
mod a_revocation_on_one_instance_refuses_the_refresh_token_everywhere;
#[cfg(feature = "tests_that_use_cluster")]
mod a_rotation_on_one_instance_supersedes_the_token_on_another;
#[cfg(feature = "tests_that_use_cluster")]
mod an_expired_code_is_refused_everywhere;
#[cfg(feature = "tests_that_use_cluster")]
mod an_expired_pending_authorization_is_unknown_everywhere;
#[cfg(feature = "tests_that_use_cluster")]
mod assert_verified_by_instance_keys;
#[cfg(feature = "tests_that_use_cluster")]
mod authorizes_consents_and_redeems_on_different_instances;
#[cfg(feature = "tests_that_use_cluster")]
mod client_credentials_introspect_active_on_another_instance;
#[cfg(feature = "tests_that_use_cluster")]
mod cluster_binary;
#[cfg(feature = "tests_that_use_cluster")]
mod concurrent_code_redemptions_on_several_instances_admit_one;
#[cfg(feature = "tests_that_use_cluster")]
mod concurrent_rotations_on_several_instances_admit_one;
#[cfg(feature = "tests_that_use_cluster")]
mod discovery_is_identical_everywhere;
#[cfg(feature = "tests_that_use_cluster")]
mod exchanges_an_external_token_on_another_instance;
#[cfg(feature = "tests_that_use_cluster")]
mod expire_framework_rows;
#[cfg(feature = "tests_that_use_cluster")]
mod instances_started_together_create_one_key_set;
#[cfg(feature = "tests_that_use_cluster")]
mod userinfo_answers_on_another_instance;
