#[cfg(feature = "runtime")]
pub use anyhow;
#[cfg(feature = "runtime")]
pub use margaret_access_token_minter as access_token_minter;
#[cfg(feature = "runtime")]
pub use margaret_asset_bag as asset_bag;
#[cfg(feature = "runtime")]
pub use margaret_bearer_token_verification as bearer_token_verification;
#[cfg(feature = "codegen")]
pub use margaret_codegen as codegen;
#[cfg(feature = "runtime")]
pub use margaret_console as console;
#[cfg(feature = "runtime")]
pub use margaret_construction::construct_singleton;
#[cfg(feature = "runtime")]
pub use margaret_construction::construction_error;
#[cfg(feature = "runtime")]
pub use margaret_environment_variable as environment_variable;
#[cfg(feature = "runtime")]
pub use margaret_http as http;
#[cfg(feature = "runtime")]
pub use margaret_http_uploaded_file as http_uploaded_file;
#[cfg(feature = "runtime")]
pub use margaret_http_validation as http_validation;
#[cfg(feature = "runtime")]
pub use margaret_identity as identity;
#[cfg(feature = "runtime")]
pub use margaret_identity_session as identity_session;
#[cfg(feature = "runtime")]
pub use margaret_jose_parameters as jose_parameters;
#[cfg(feature = "runtime")]
pub use margaret_jwks_client as jwks_client;
#[cfg(feature = "runtime")]
pub use margaret_jwks_endpoint as jwks_endpoint;
#[cfg(feature = "runtime")]
pub use margaret_jwks_file_secret_storage as jwks_file_secret_storage;
#[cfg(feature = "runtime")]
pub use margaret_jwks_keygen as jwks_keygen;
#[cfg(feature = "runtime")]
pub use margaret_jwks_roller as jwks_roller;
#[cfg(feature = "runtime")]
pub use margaret_jwks_roller_server as jwks_roller_server;
#[cfg(feature = "runtime")]
pub use margaret_jwks_secret_storage_selection as jwks_secret_storage_selection;
#[cfg(feature = "runtime")]
pub use margaret_jwks_secret_store as jwks_secret_store;
#[cfg(feature = "runtime")]
pub use margaret_jws_verification as jws_verification;
#[cfg(feature = "runtime")]
pub use margaret_jwt_verification as jwt_verification;
#[cfg(feature = "runtime")]
pub use margaret_macros as macros;
#[cfg(feature = "runtime")]
pub use margaret_model as model;
#[cfg(feature = "runtime")]
pub use margaret_oidc_client as oidc_client;
#[cfg(feature = "runtime")]
pub use margaret_peer_identity as peer_identity;
#[cfg(feature = "runtime")]
pub use margaret_registered_claims as registered_claims;
#[cfg(feature = "runtime")]
pub use margaret_route_parameter_binding as route_parameter_binding;
#[cfg(feature = "runtime")]
pub use margaret_service as service;
#[cfg(feature = "runtime")]
pub use margaret_spiffe_svid as spiffe_svid;
#[cfg(feature = "runtime")]
pub use margaret_spiffe_svid_bundle as spiffe_svid_bundle;
#[cfg(feature = "runtime")]
pub use margaret_spiffe_svid_client as spiffe_svid_client;
#[cfg(feature = "runtime")]
pub use margaret_spiffe_svid_server as spiffe_svid_server;
#[cfg(feature = "runtime")]
pub use margaret_sync_holder as sync_holder;
#[cfg(feature = "runtime")]
pub use margaret_token_issuance as token_issuance;
#[cfg(feature = "runtime")]
pub use margaret_token_signer as token_signer;
#[cfg(feature = "runtime")]
pub use margaret_token_trust as token_trust;
#[cfg(feature = "runtime")]
pub use margaret_validation as validation;
#[cfg(feature = "runtime")]
pub use margaret_views as views;
#[cfg(feature = "runtime")]
pub use margaret_websocket as websocket;
#[cfg(feature = "runtime")]
pub use margaret_websocket_session as websocket_session;

#[cfg(all(test, feature = "runtime"))]
mod tests {
    use crate::framework::macros::model;

    #[model(table = "facade_probes")]
    #[primary_key(columns = [id, slot])]
    #[unique(columns = [slot, label])]
    #[index(name = "facade_probes_label_slot", columns = [label, slot])]
    struct FacadeProbe {
        #[column]
        id: u32,
        #[column]
        slot: u32,
        #[column]
        label: u32,
    }

    #[test]
    fn re_exports_attribute_macros_that_expand_through_the_umbrella() {
        let probe = FacadeProbe {
            id: 7,
            slot: 3,
            label: 1,
        };

        assert_eq!(probe.id, 7);
        assert_eq!(probe.slot, 3);
        assert_eq!(probe.label, 1);
    }
}
