pub struct SvidServiceBundleParams {
    pub spiffe_trust_domain: String,
    pub spire_agent_addr: String,
}

#[cfg(test)]
mod tests {
    use super::SvidServiceBundleParams;

    #[test]
    fn constructs_params_with_trust_domain_and_socket_path() {
        let params = SvidServiceBundleParams {
            spiffe_trust_domain: "example.org".to_string(),
            spire_agent_addr: "unix:///tmp/agent.sock".to_string(),
        };

        assert_eq!(params.spiffe_trust_domain, "example.org");
        assert_eq!(params.spire_agent_addr, "unix:///tmp/agent.sock");
    }
}
