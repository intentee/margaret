use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Result;
use tempfile::TempDir;
use tokio::process::Child;

use crate::spawn_test_subprocess::spawn_test_subprocess;
use crate::spire_test_cluster_layout::SpireTestClusterLayout;
use crate::spire_test_cluster_params::SpireTestClusterParams;
use crate::wait_until_unix_socket_ready::wait_until_unix_socket_ready;
use crate::wait_until_workload_api_ready::wait_until_workload_api_ready;
use crate::write_spire_config::write_spire_config;

pub struct SpireTestCluster {
    _agent_child: Child,
    _server_child: Child,
    agent_socket_path: PathBuf,
    server_socket_path: PathBuf,
    trust_domain: String,
    _data_dir: TempDir,
}

impl SpireTestCluster {
    #[must_use]
    pub fn new(
        server_child: Child,
        agent_child: Child,
        agent_socket_path: PathBuf,
        server_socket_path: PathBuf,
        trust_domain: String,
        data_dir: TempDir,
    ) -> Self {
        Self {
            _agent_child: agent_child,
            _server_child: server_child,
            agent_socket_path,
            server_socket_path,
            trust_domain,
            _data_dir: data_dir,
        }
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    pub async fn start(
        data_dir: TempDir,
        SpireTestClusterParams {
            agent_binary_path,
            agent_socket_readiness_timeout,
            server_binary_path,
            server_socket_readiness_timeout,
            trust_domain,
            workload_api_readiness_timeout,
        }: SpireTestClusterParams,
        server_port: u16,
    ) -> Result<Self> {
        let layout = SpireTestClusterLayout::new(data_dir.path(), &trust_domain, server_port);

        layout.prepare_data_dirs().await?;
        write_spire_config(&layout.server_conf_path, layout.render_server_config()).await?;

        let server_run_args: [&OsStr; 3] = [
            OsStr::new("run"),
            OsStr::new("-config"),
            layout.server_conf_path.as_os_str(),
        ];
        let server_child = spawn_test_subprocess(&server_binary_path, &server_run_args).await?;

        wait_until_unix_socket_ready(&layout.server_socket_path, server_socket_readiness_timeout)
            .await?;

        layout.obtain_trust_bundle(&server_binary_path).await?;
        let join_token = layout.obtain_join_token(&server_binary_path).await?;

        layout.register_workload_entry(&server_binary_path).await?;

        write_spire_config(&layout.agent_conf_path, layout.render_agent_config()).await?;

        let agent_run_args: [&OsStr; 5] = [
            OsStr::new("run"),
            OsStr::new("-config"),
            layout.agent_conf_path.as_os_str(),
            OsStr::new("-joinToken"),
            OsStr::new(&join_token),
        ];
        let agent_child = spawn_test_subprocess(&agent_binary_path, &agent_run_args).await?;

        wait_until_unix_socket_ready(&layout.agent_socket_path, agent_socket_readiness_timeout)
            .await?;

        wait_until_workload_api_ready(&layout.agent_socket_path, workload_api_readiness_timeout)
            .await?;

        Ok(Self::new(
            server_child,
            agent_child,
            layout.agent_socket_path,
            layout.server_socket_path,
            layout.trust_domain,
            data_dir,
        ))
    }

    #[must_use]
    pub fn agent_socket_path(&self) -> &Path {
        &self.agent_socket_path
    }

    #[must_use]
    pub fn server_socket_path(&self) -> &Path {
        &self.server_socket_path
    }

    #[must_use]
    pub fn spiffe_trust_domain(&self) -> &str {
        &self.trust_domain
    }
}
