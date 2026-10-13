use std::path::PathBuf;

use tempfile::NamedTempFile;
use tokio::process::Command;
use url::Url;

use margaret_database_tests::racing_instances::RACING_INSTANCES;

use crate::cluster_database_max_connections_variable::CLUSTER_DATABASE_MAX_CONNECTIONS_VARIABLE;
use crate::cluster_database_url_variable::CLUSTER_DATABASE_URL_VARIABLE;
use crate::trusted_certificates_variable::TRUSTED_CERTIFICATES_VARIABLE;

pub struct InstanceLaunch {
    pub binary: PathBuf,
    pub certificate_file: NamedTempFile,
    pub database_url: String,
    pub public_url: Url,
}

impl InstanceLaunch {
    #[must_use]
    pub fn command(&self) -> Command {
        let mut command = Command::new(&self.binary);

        command
            .env(CLUSTER_DATABASE_URL_VARIABLE, &self.database_url)
            .env(
                CLUSTER_DATABASE_MAX_CONNECTIONS_VARIABLE,
                RACING_INSTANCES.to_string(),
            )
            .env(TRUSTED_CERTIFICATES_VARIABLE, self.certificate_file.path())
            .kill_on_drop(true);

        command
    }
}
