use std::path::PathBuf;
use std::time::Duration;

pub struct SpireTestClusterParams {
    pub agent_binary_path: PathBuf,
    pub agent_socket_readiness_timeout: Duration,
    pub server_binary_path: PathBuf,
    pub server_socket_readiness_timeout: Duration,
    pub trust_domain: String,
    pub workload_api_readiness_timeout: Duration,
}

impl Default for SpireTestClusterParams {
    fn default() -> Self {
        Self {
            agent_binary_path: PathBuf::from("spire-agent"),
            agent_socket_readiness_timeout: Duration::from_secs(30),
            server_binary_path: PathBuf::from("spire-server"),
            server_socket_readiness_timeout: Duration::from_secs(30),
            trust_domain: "spiffe.test".to_owned(),
            workload_api_readiness_timeout: Duration::from_secs(30),
        }
    }
}
