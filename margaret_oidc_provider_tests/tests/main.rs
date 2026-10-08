#[cfg(feature = "tests_that_use_postgres")]
mod accepts_the_revocation_of_an_unknown_token_hinted_as_an_access_token;
#[cfg(feature = "tests_that_use_postgres")]
mod accepts_the_rfc_7009_example_revocation_of_an_invalid_token;
#[cfg(feature = "tests_that_use_postgres")]
mod answers_the_rfc_7662_example_introspection_of_an_inactive_token;
#[cfg(feature = "tests_that_use_postgres")]
mod answers_userinfo_with_the_subject_of_the_access_token;
#[cfg(feature = "tests_that_use_postgres")]
mod asks_an_anonymous_end_user_to_authenticate;
#[cfg(feature = "tests_that_use_postgres")]
mod asks_consent_of_a_prompting_client;
#[cfg(feature = "tests_that_use_postgres")]
mod asks_consent_when_an_implicitly_consenting_client_prompts_for_it;
#[cfg(feature = "tests_that_use_postgres")]
mod asks_to_authenticate_again_after_the_maximum_authentication_age;
#[cfg(feature = "tests_that_use_postgres")]
mod asks_to_authenticate_again_and_keeps_the_consent_prompt;
#[cfg(feature = "tests_that_use_postgres")]
mod asks_to_authenticate_again_when_login_is_prompted;
#[cfg(feature = "tests_that_use_postgres")]
mod authenticates_the_client_before_refusing_an_unsupported_grant;
#[cfg(feature = "tests_that_use_postgres")]
mod awaits_the_signing_keys_of_a_client;
#[cfg(feature = "tests_that_use_postgres")]
mod awaits_the_signing_keys_of_the_subject_token_issuer;
#[cfg(feature = "tests_that_use_postgres")]
mod exchanges_a_code_of_a_public_client_for_a_named_resource;
#[cfg(feature = "tests_that_use_postgres")]
mod exchanges_a_code_without_openid_for_an_access_token_alone;
#[cfg(feature = "tests_that_use_postgres")]
mod exchanges_a_subject_token_for_a_margaret_client;
#[cfg(feature = "tests_that_use_postgres")]
mod exchanges_a_trusted_subject_token;
#[cfg(feature = "tests_that_use_postgres")]
mod exchanges_an_authorization_code_for_tokens;
#[cfg(feature = "tests_that_use_postgres")]
mod forgets_a_consent_decided_by_another_end_user;
#[cfg(feature = "tests_that_use_postgres")]
mod forgets_an_expired_pending_consent;
#[cfg(feature = "tests_that_use_postgres")]
mod introspects_an_access_token_addressed_to_the_callers_resources;
#[cfg(feature = "tests_that_use_postgres")]
mod introspects_an_access_token_for_a_margaret_resource_server;
#[cfg(feature = "tests_that_use_postgres")]
mod introspects_an_access_token_of_a_single_audience;
#[cfg(feature = "tests_that_use_postgres")]
mod issues_a_code_after_an_approved_consent;
#[cfg(feature = "tests_that_use_postgres")]
mod issues_a_code_to_an_implicitly_consenting_client;
#[cfg(feature = "tests_that_use_postgres")]
mod issues_a_code_without_a_requested_scope_or_state;
#[cfg(feature = "tests_that_use_postgres")]
mod issues_client_credentials_to_a_margaret_client;
#[cfg(feature = "tests_that_use_postgres")]
mod issues_client_credentials_tokens;
#[cfg(feature = "tests_that_use_postgres")]
mod narrows_the_scope_of_a_refreshed_token;
#[cfg(feature = "tests_that_use_postgres")]
mod publishes_the_provider_metadata;
#[cfg(feature = "tests_that_use_postgres")]
mod redeems_a_code_with_the_rfc_7636_example_verifier;
#[cfg(feature = "tests_that_use_postgres")]
mod redeems_an_authorization_code_once;
#[cfg(feature = "tests_that_use_postgres")]
mod redirects_a_code_challenge_that_is_not_a_sha256_digest;
#[cfg(feature = "tests_that_use_postgres")]
mod redirects_a_code_challenge_that_is_not_base64url;
#[cfg(feature = "tests_that_use_postgres")]
mod redirects_a_denied_consent;
#[cfg(feature = "tests_that_use_postgres")]
mod redirects_a_malformed_maximum_authentication_age;
#[cfg(feature = "tests_that_use_postgres")]
mod redirects_a_malformed_scope;
#[cfg(feature = "tests_that_use_postgres")]
mod redirects_a_plain_code_challenge;
#[cfg(feature = "tests_that_use_postgres")]
mod redirects_a_prompt_combining_none_with_login;
#[cfg(feature = "tests_that_use_postgres")]
mod redirects_a_prompt_listing_an_unsupported_value;
#[cfg(feature = "tests_that_use_postgres")]
mod redirects_a_prompting_client_asked_for_no_interaction;
#[cfg(feature = "tests_that_use_postgres")]
mod redirects_a_request_object;
#[cfg(feature = "tests_that_use_postgres")]
mod redirects_a_request_object_reference;
#[cfg(feature = "tests_that_use_postgres")]
mod redirects_a_request_without_a_code_challenge;
#[cfg(feature = "tests_that_use_postgres")]
mod redirects_a_request_without_a_code_challenge_method;
#[cfg(feature = "tests_that_use_postgres")]
mod redirects_a_scope_the_client_may_not_request;
#[cfg(feature = "tests_that_use_postgres")]
mod redirects_an_anonymous_end_user_asked_for_no_interaction;
#[cfg(feature = "tests_that_use_postgres")]
mod redirects_an_unsupported_response_type;
#[cfg(feature = "tests_that_use_postgres")]
mod refreshes_with_the_granted_scopes_when_the_scope_is_empty;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_a_client_without_credentials_without_a_challenge;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_a_code_exchange_for_a_resource_outside_the_client;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_a_code_exchange_that_names_no_resource_of_several;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_a_code_exchanged_by_a_client_without_the_grant;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_a_code_granted_to_another_client;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_a_code_presented_with_a_malformed_callback;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_a_code_presented_with_a_verifier_longer_than_128_characters;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_a_code_presented_with_a_verifier_of_reserved_characters;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_a_code_presented_with_a_verifier_shorter_than_43_characters;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_a_code_presented_with_another_callback;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_a_code_presented_with_another_verifier;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_a_code_replayed_before_its_refresh_family_opens;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_a_malformed_introspection_request;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_a_malformed_revocation_request;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_a_malformed_token_request;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_a_refresh_beyond_the_granted_scope_and_keeps_the_token;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_a_refresh_by_a_client_without_the_grant;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_a_refresh_for_a_resource_outside_the_client;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_a_refresh_token_of_another_client;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_a_refresh_token_revoked_while_it_rotates;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_a_refresh_token_rotated_concurrently;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_a_refresh_with_a_malformed_scope;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_a_replayed_client_assertion;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_a_revocation_by_an_unauthenticated_client;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_a_subject_token_addressed_to_no_exchanger;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_a_subject_token_addressed_to_two_exchangers;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_a_subject_token_of_a_type_its_exchanger_does_not_admit;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_a_subject_token_of_an_untrusted_issuer;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_a_subject_token_that_is_not_a_jws;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_a_subject_token_the_exchanger_refuses;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_a_token_request_that_is_not_a_form;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_a_token_request_with_an_empty_code;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_a_token_request_with_an_empty_grant_type;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_an_exchange_beyond_the_exchanged_scope;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_an_exchange_for_a_resource_outside_the_client;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_an_exchange_for_another_token_type;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_an_exchange_naming_both_an_audience_and_a_resource;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_an_exchange_on_behalf_of_an_actor;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_an_expired_authorization_code;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_an_expired_refresh_token;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_an_introspection_by_a_client_not_permitted_to_introspect;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_an_introspection_by_a_public_client;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_an_introspection_by_an_unauthenticated_client;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_an_unknown_authorization_code;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_an_unsupported_grant_type;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_an_unsupported_subject_token_type;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_client_authentication_in_the_authorization_header_with_a_basic_challenge;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_client_credentials_beyond_the_client_scope;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_client_credentials_for_a_resource_outside_the_client;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_client_credentials_to_a_client_without_the_grant;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_client_credentials_with_a_malformed_scope;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_to_revoke_a_refresh_token_of_another_client;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_to_revoke_an_access_token;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_token_exchange_to_a_client_without_the_grant;
mod refuses_userinfo_claims_that_are_not_an_object;
mod refuses_userinfo_claims_that_name_their_own_subject;
mod refuses_userinfo_for_a_token_of_a_client_itself;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_userinfo_for_a_token_without_openid;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_userinfo_with_a_rejected_token;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_userinfo_with_malformed_credentials;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_userinfo_without_credentials;
#[cfg(feature = "tests_that_use_postgres")]
mod rejects_a_client_that_may_not_request_codes;
#[cfg(feature = "tests_that_use_postgres")]
mod rejects_a_malformed_authorization_request;
#[cfg(feature = "tests_that_use_postgres")]
mod rejects_an_authorization_request_of_an_unknown_client;
#[cfg(feature = "tests_that_use_postgres")]
mod rejects_an_unregistered_redirect_uri;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_client_assertion_that_cannot_be_remembered;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_code_exchange_whose_code_cannot_be_redeemed;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_code_exchange_whose_refresh_family_cannot_be_opened;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_code_that_cannot_be_issued;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_failing_exchanger_as_a_server_error;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_pending_consent_that_cannot_be_held;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_pending_consent_that_cannot_be_taken;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_refresh_whose_token_cannot_be_found;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_refresh_whose_token_cannot_be_rotated;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_replayed_code_whose_family_cannot_be_revoked;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_revocation_whose_client_assertion_cannot_be_remembered;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_revocation_whose_refresh_token_cannot_be_found;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_a_token_that_is_not_an_access_token_as_inactive;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_an_exchanger_granting_an_undeclared_scope_as_a_server_error;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_an_introspection_whose_client_assertion_cannot_be_remembered;
mod reports_userinfo_claims_that_cannot_be_serialized;
#[cfg(feature = "tests_that_use_postgres")]
mod revokes_a_refresh_token_presented_with_an_access_token_hint;
#[cfg(feature = "tests_that_use_postgres")]
mod revokes_the_family_of_a_refresh_token;
#[cfg(feature = "tests_that_use_postgres")]
mod revokes_the_family_of_a_replayed_refresh_token;
#[cfg(feature = "tests_that_use_postgres")]
mod rotates_a_refresh_token;
#[cfg(feature = "tests_that_use_postgres")]
mod serves_an_access_token_a_margaret_resource_server_admits;
#[cfg(feature = "tests_that_use_postgres")]
mod signs_in_a_margaret_client_through_the_provider;
#[cfg(feature = "tests_that_use_postgres")]
mod treats_an_empty_max_age_as_omitted;
