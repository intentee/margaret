use std::ffi::OsString;
use std::net::SocketAddr;

use margaret_spiffe_svid::svid_service_bundle_params::SvidServiceBundleParams;

#[must_use]
pub fn mesh_serve_arguments(
    mesh: SocketAddr,
    public: SocketAddr,
    SvidServiceBundleParams {
        spiffe_trust_domain,
        spire_agent_addr,
    }: &SvidServiceBundleParams,
) -> Vec<OsString> {
    vec![
        OsString::from("margaret_coverage_spiffe_fixture"),
        OsString::from("serve"),
        OsString::from("--spiffe-trust-domain"),
        OsString::from(spiffe_trust_domain),
        OsString::from("--spire-agent-addr"),
        OsString::from(spire_agent_addr),
        OsString::from("--mesh-addr"),
        OsString::from(mesh.to_string()),
        OsString::from("--mesh-url"),
        OsString::from(format!("https://{mesh}")),
        OsString::from("--mesh-transport"),
        OsString::from("spiffe_mtls"),
        OsString::from("--public-addr"),
        OsString::from(public.to_string()),
        OsString::from("--public-url"),
        OsString::from(format!("http://{public}")),
        OsString::from("--public-transport"),
        OsString::from("plain"),
    ]
}
