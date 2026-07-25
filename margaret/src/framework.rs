pub use margaret_access_token_minter as access_token_minter;
pub use margaret_asset_bag as asset_bag;
#[cfg(feature = "codegen")]
pub use margaret_codegen as codegen;
pub use margaret_console as console;
pub use margaret_endpoint as endpoint;
pub use margaret_http as http;
pub use margaret_http_validation as http_validation;
pub use margaret_identity as identity;
pub use margaret_identity_session as identity_session;
pub use margaret_jwks_client as jwks_client;
pub use margaret_jwks_file_secret_storage as jwks_file_secret_storage;
pub use margaret_jwks_keygen as jwks_keygen;
pub use margaret_jwks_roller as jwks_roller;
pub use margaret_jwks_roller_server as jwks_roller_server;
pub use margaret_jwks_secret_storage_selection as jwks_secret_storage_selection;
pub use margaret_jwks_secret_store as jwks_secret_store;
pub use margaret_macros as macros;
pub use margaret_model as model;
pub use margaret_peer_identity as peer_identity;
pub use margaret_service as service;
pub use margaret_spiffe_svid as spiffe_svid;
pub use margaret_spiffe_svid_client as spiffe_svid_client;
pub use margaret_spiffe_svid_server as spiffe_svid_server;
pub use margaret_sync_holder as sync_holder;
pub use margaret_token_signer as token_signer;
pub use margaret_validation as validation;
pub use margaret_views as views;
pub use margaret_websocket as websocket;

#[cfg(test)]
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
