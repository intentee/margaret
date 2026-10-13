use thiserror::Error;

#[derive(Debug, Error)]
pub enum SvidBundleError {
    #[error("the svid client side could not be configured: {0}")]
    ClientSide(#[source] margaret_spiffe_svid_client::svid_error::SvidError),

    #[error("the svid server side could not be configured: {0}")]
    ServerSide(#[source] margaret_spiffe_svid_server::svid_error::SvidError),
}
