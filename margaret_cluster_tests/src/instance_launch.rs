use std::path::PathBuf;

use tempfile::NamedTempFile;
use tokio::process::Command;
use url::Url;

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
            .env(TRUSTED_CERTIFICATES_VARIABLE, self.certificate_file.path())
            .kill_on_drop(true);

        command
    }
}
