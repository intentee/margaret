use margaret_spiffe_svid::extract_server_credentials::extract_server_credentials;
use margaret_spiffe_svid::svid_certified_key::SvidCertifiedKey;

use crate::build_workload_svid::build_workload_svid;

#[must_use]
pub fn build_workload_credentials() -> SvidCertifiedKey {
    extract_server_credentials(&build_workload_svid()).unwrap()
}
