#[cfg(feature = "tests_that_use_spire")]
mod admits_a_mesh_caller_declaring_a_role;
#[cfg(feature = "tests_that_use_spire")]
mod challenges_a_mesh_caller_declaring_no_role;
#[cfg(feature = "tests_that_use_spire")]
mod identifies_a_peer_over_a_mesh_websocket;
mod prints_help_without_a_command;
mod refuses_to_serve_without_a_trust_domain;
#[cfg(feature = "tests_that_use_spire")]
mod relays_the_peer_identity_the_mesh_verified;
