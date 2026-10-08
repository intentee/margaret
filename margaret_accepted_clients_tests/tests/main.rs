#[cfg(feature = "tests_that_use_postgres")]
mod accepts_a_form_client_identifier_naming_the_asserting_client;
#[cfg(feature = "tests_that_use_postgres")]
mod authenticates_a_client_by_an_assertion_signed_with_own_keys;
#[cfg(feature = "tests_that_use_postgres")]
mod authenticates_a_confidential_client_by_its_assertion;
mod authenticates_a_public_client_by_its_identifier;
#[cfg(feature = "tests_that_use_postgres")]
mod awaits_the_keys_of_a_confidential_client;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_a_replayed_client_assertion;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_a_request_without_client_credentials;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_an_assertion_about_another_subject;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_an_assertion_addressed_to_an_audience_array;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_an_assertion_addressed_to_the_token_endpoint;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_an_assertion_from_a_public_client;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_an_assertion_from_an_unknown_client;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_an_assertion_outliving_the_lifetime_limit;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_an_assertion_signed_by_a_key_of_another_client;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_an_assertion_signed_with_an_algorithm_other_than_the_pinned_one;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_an_assertion_that_is_not_a_jwt;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_an_assertion_type_without_an_assertion;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_an_assertion_typed_for_another_purpose;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_an_assertion_without_an_identifier;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_an_assertion_without_its_type;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_an_expired_assertion;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_an_own_keys_assertion_typed_for_another_purpose;
mod refuses_an_unknown_public_client;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_an_unsupported_assertion_type;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_authentication_in_the_authorization_header;
#[cfg(feature = "tests_that_use_postgres")]
mod refuses_conflicting_client_identifiers;
#[cfg(feature = "tests_that_use_postgres")]
mod reports_an_assertion_that_cannot_be_remembered;
#[cfg(feature = "tests_that_use_postgres")]
mod requires_an_assertion_from_a_confidential_client;
