#[cfg(feature = "runtime")]
pub use crate::construction_error;
#[cfg(feature = "runtime")]
pub use anyhow;
#[cfg(feature = "runtime")]
pub use margaret_access_token_minter as access_token_minter;
#[cfg(feature = "runtime")]
pub use margaret_asset_bag as asset_bag;
#[cfg(feature = "codegen")]
pub use margaret_codegen as codegen;
#[cfg(feature = "runtime")]
pub use margaret_console as console;
#[cfg(feature = "runtime")]
pub use margaret_http as http;
#[cfg(feature = "runtime")]
pub use margaret_http_validation as http_validation;
#[cfg(feature = "runtime")]
pub use margaret_identity as identity;
#[cfg(feature = "runtime")]
pub use margaret_identity_session as identity_session;
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
pub use margaret_macros as macros;
#[cfg(feature = "runtime")]
pub use margaret_model as model;
#[cfg(feature = "runtime")]
pub use margaret_peer_identity as peer_identity;
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
pub use margaret_token_signer as token_signer;
#[cfg(feature = "runtime")]
pub use margaret_validation as validation;
#[cfg(feature = "runtime")]
pub use margaret_views as views;
#[cfg(feature = "runtime")]
pub use margaret_websocket as websocket;

#[cfg(all(test, feature = "runtime"))]
mod tests {
    use crate::framework::macros::model;

    #[model(table = "facade_probes")]
    struct FacadeProbe {
        #[column(primary_key)]
        id: u32,
    }

    #[test]
    fn re_exports_attribute_macros_that_expand_through_the_umbrella() {
        let probe = FacadeProbe { id: 7 };

        assert_eq!(probe.id, 7);
    }
}
