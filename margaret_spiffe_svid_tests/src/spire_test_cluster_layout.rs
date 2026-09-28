use std::path::Path;
use std::path::PathBuf;

use anyhow::Result;
use tokio::fs;

use crate::parse_join_token::parse_join_token;
use crate::run_spire_command::run_spire_command;

const AGENT_ENTRY_SYNC_INTERVAL: &str = "100ms";
const SERVER_ENTRY_CACHE_RELOAD_INTERVAL: &str = "100ms";

pub struct SpireTestClusterLayout {
    pub agent_conf_path: PathBuf,
    pub agent_data_dir: PathBuf,
    pub agent_socket_path: PathBuf,
    pub server_conf_path: PathBuf,
    pub server_data_dir: PathBuf,
    pub server_port: u16,
    pub server_socket_path: PathBuf,
    pub trust_bundle_path: PathBuf,
    pub trust_domain: String,
}

impl SpireTestClusterLayout {
    #[must_use]
    pub fn new(data_dir: &Path, trust_domain: &str, server_port: u16) -> Self {
        Self {
            agent_conf_path: data_dir.join("agent.conf"),
            agent_data_dir: data_dir.join("agent-data"),
            agent_socket_path: data_dir.join("agent.sock"),
            server_conf_path: data_dir.join("server.conf"),
            server_data_dir: data_dir.join("server-data"),
            server_port,
            server_socket_path: data_dir.join("server.sock"),
            trust_bundle_path: data_dir.join("trust-bundle.pem"),
            trust_domain: trust_domain.to_owned(),
        }
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    pub async fn obtain_join_token(&self, server_binary_path: &Path) -> Result<String> {
        let output = run_spire_command(
            server_binary_path,
            &self.server_socket_path,
            &[
                "token",
                "generate",
                "-spiffeID",
                &format!("spiffe://{}/agent", self.trust_domain),
                "-output",
                "json",
            ],
        )
        .await?;

        parse_join_token(&output)
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    pub async fn obtain_trust_bundle(&self, server_binary_path: &Path) -> Result<Vec<u8>> {
        let trust_bundle = run_spire_command(
            server_binary_path,
            &self.server_socket_path,
            &["bundle", "show"],
        )
        .await?;

        fs::write(&self.trust_bundle_path, &trust_bundle).await?;

        Ok(trust_bundle)
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    pub async fn prepare_data_dirs(&self) -> Result<()> {
        fs::create_dir_all(&self.server_data_dir).await?;
        fs::create_dir_all(&self.agent_data_dir).await?;

        Ok(())
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    pub async fn register_workload_entry(&self, server_binary_path: &Path) -> Result<()> {
        let uid = unsafe { libc::geteuid() };

        run_spire_command(
            server_binary_path,
            &self.server_socket_path,
            &[
                "entry",
                "create",
                "-spiffeID",
                &format!("spiffe://{}/workload", self.trust_domain),
                "-parentID",
                &format!("spiffe://{}/agent", self.trust_domain),
                "-selector",
                &format!("unix:uid:{uid}"),
            ],
        )
        .await?;

        Ok(())
    }

    #[must_use]
    pub fn render_agent_config(&self) -> String {
        format!(
            r#"agent {{
    data_dir = "{data_dir}"
    log_level = "ERROR"
    server_address = "127.0.0.1"
    server_port = {server_port}
    socket_path = "{socket_path}"
    trust_bundle_path = "{trust_bundle_path}"
    trust_domain = "{trust_domain}"

    experimental {{
        sync_interval = "{sync_interval}"
    }}
}}

plugins {{
    KeyManager "memory" {{
        plugin_data {{}}
    }}

    NodeAttestor "join_token" {{
        plugin_data {{}}
    }}

    WorkloadAttestor "unix" {{
        plugin_data {{
            discover_workload_path = true
        }}
    }}
}}
"#,
            data_dir = self.agent_data_dir.display(),
            server_port = self.server_port,
            socket_path = self.agent_socket_path.display(),
            sync_interval = AGENT_ENTRY_SYNC_INTERVAL,
            trust_bundle_path = self.trust_bundle_path.display(),
            trust_domain = self.trust_domain,
        )
    }

    #[must_use]
    pub fn render_server_config(&self) -> String {
        format!(
            r#"server {{
    bind_address = "127.0.0.1"
    bind_port = "{bind_port}"
    socket_path = "{socket_path}"
    trust_domain = "{trust_domain}"
    data_dir = "{data_dir}"
    log_level = "ERROR"
    ca_ttl = "10m"
    default_x509_svid_ttl = "5m"

    experimental {{
        cache_reload_interval = "{cache_reload_interval}"
    }}
}}

plugins {{
    DataStore "sql" {{
        plugin_data {{
            database_type = "sqlite3"
            connection_string = "{data_dir}/datastore.sqlite3"
        }}
    }}

    KeyManager "memory" {{
        plugin_data {{}}
    }}

    NodeAttestor "join_token" {{
        plugin_data {{}}
    }}
}}
"#,
            bind_port = self.server_port,
            cache_reload_interval = SERVER_ENTRY_CACHE_RELOAD_INTERVAL,
            socket_path = self.server_socket_path.display(),
            trust_domain = self.trust_domain,
            data_dir = self.server_data_dir.display(),
        )
    }
}

#[cfg(test)]
mod tests {
    use tokio::fs;

    use super::SpireTestClusterLayout;

    #[tokio::test]
    async fn creates_both_data_dirs() {
        let tempdir = tempfile::tempdir().unwrap();
        let layout = SpireTestClusterLayout::new(tempdir.path(), "spiffe.test", 1234);

        layout.prepare_data_dirs().await.unwrap();

        assert!(layout.server_data_dir.is_dir());
        assert!(layout.agent_data_dir.is_dir());
    }

    #[tokio::test]
    async fn errors_when_server_data_dir_path_is_blocked_by_a_file() {
        let tempdir = tempfile::tempdir().unwrap();
        let layout = SpireTestClusterLayout::new(tempdir.path(), "spiffe.test", 1234);

        fs::write(&layout.server_data_dir, b"blocker")
            .await
            .unwrap();

        let result = layout.prepare_data_dirs().await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn errors_when_agent_data_dir_path_is_blocked_by_a_file() {
        let tempdir = tempfile::tempdir().unwrap();
        let layout = SpireTestClusterLayout::new(tempdir.path(), "spiffe.test", 1234);

        fs::write(&layout.agent_data_dir, b"blocker").await.unwrap();

        let result = layout.prepare_data_dirs().await;

        assert!(result.is_err());
    }
}
