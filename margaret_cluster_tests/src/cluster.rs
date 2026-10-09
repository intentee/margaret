use std::collections::HashMap;
use std::future::Future;
use std::io::Write;
use std::path::Path;
use std::process::ExitStatus;
use std::process::Output;

use futures_util::future::join_all;
use reqwest::Client;
use tempfile::NamedTempFile;
use url::Url;

use margaret_cluster_fixture::margaret::routes::Routes;
use margaret_cluster_fixture::margaret::schema::schema;
use margaret_cluster_fixture::margaret::token_issuance::TOKEN_ISSUANCE;
use margaret_database_tests::apply_schema::apply_schema;
use margaret_database_tests::started_database::StartedDatabase;
use margaret_http_tests::tls_fixture::TlsFixture;

use crate::await_probe::await_probe;
use crate::cluster_client::cluster_client;
use crate::cluster_doors::ClusterDoors;
use crate::cluster_instance::ClusterInstance;
use crate::cluster_member::ClusterMember;
use crate::cluster_server::ClusterServer;
use crate::declared_port::declared_port;
use crate::external_issuer::ExternalIssuer;
use crate::front_door::FrontDoor;
use crate::instance_admission::InstanceAdmission;
use crate::instance_launch::InstanceLaunch;
use crate::instance_listening::instance_listening;
use crate::instance_ports::InstancePorts;
use crate::instance_ready::instance_ready;
use crate::relay_destination::RelayDestination;

async fn await_running<TProbe: Future<Output = bool>>(
    running: &mut HashMap<usize, ClusterInstance>,
    members: &[ClusterMember],
    indices: &[usize],
    probe: impl Fn(InstancePorts) -> TProbe,
) {
    join_all(
        running
            .iter_mut()
            .filter(|(index, _instance)| indices.contains(index))
            .map(|(index, instance)| {
                let ports = members[*index].ports;
                let probe = &probe;

                await_probe(instance, move || probe(ports))
            }),
    )
    .await;
}

pub struct Cluster {
    running: HashMap<usize, ClusterInstance>,
    pub client: Client,
    pub database: StartedDatabase,
    pub external_issuer: ExternalIssuer,
    front_door: FrontDoor,
    launch: InstanceLaunch,
    members: Vec<ClusterMember>,
    pub tls: TlsFixture,
}

impl Cluster {
    /// # Panics
    ///
    /// Panics when the database, the doors or the instances cannot be prepared.
    pub async fn start(binary: &Path, instances: usize) -> Self {
        let mut cluster = Self::prepare(binary).await;
        let mut started = Vec::new();

        for _ in 0..instances {
            started.push(cluster.add_member().await);
        }

        cluster.restart(&started).await;

        cluster
    }

    /// # Panics
    ///
    /// Panics when the database, the doors or the seed cannot be prepared.
    pub async fn prepare(binary: &Path) -> Self {
        let tls = TlsFixture::generate();
        let database = StartedDatabase::start().await;
        let mut certificate_file = NamedTempFile::new().expect("the certificate file is created");

        certificate_file
            .write_all(tls.certificate_authority.certificate_pem().as_bytes())
            .expect("the certificate authority is written");
        apply_schema(&database.database, &schema()).await;

        let front_door =
            FrontDoor::open(&tls.server_config, declared_port(TOKEN_ISSUANCE.issuer)).await;
        let cluster = Self {
            client: cluster_client(&tls.certificate_authority),
            external_issuer: ExternalIssuer::open(&tls.server_config).await,
            launch: InstanceLaunch {
                binary: binary.to_path_buf(),
                certificate_file,
                database_url: database.ephemeral.database_url().to_string(),
                public_url: front_door.doors.url(ClusterServer::Public),
            },
            running: HashMap::new(),
            database,
            front_door,
            members: Vec::new(),
            tls,
        };

        assert!(cluster.command(&["seed"]).await.status.success());

        cluster
    }

    /// # Panics
    ///
    /// Panics when the console command cannot be run.
    pub async fn command(&self, arguments: &[&str]) -> Output {
        self.launch
            .command()
            .args(arguments)
            .output()
            .await
            .expect("the console command runs")
    }

    pub fn admit(&self, index: usize) {
        self.front_door.backends.admit(self.members[index].ports);
    }

    pub fn drain(&self, index: usize) {
        self.front_door.backends.drain(self.members[index].ports);
    }

    #[must_use]
    pub fn url(&self, server: ClusterServer) -> Url {
        self.front_door.doors.url(server)
    }

    #[must_use]
    pub fn instance_url(&self, index: usize, server: ClusterServer) -> Url {
        self.members[index].doors.url(server)
    }

    #[must_use]
    pub fn instance_routes(&self, index: usize) -> Routes {
        self.members[index].doors.routes()
    }

    #[must_use]
    pub fn routes(&self) -> Routes {
        self.front_door.doors.routes()
    }

    pub async fn add_member(&mut self) -> usize {
        let ports = InstancePorts::reserve();

        self.members.push(ClusterMember {
            doors: ClusterDoors::open(&self.tls.server_config, 0, |server| {
                RelayDestination::Fixed(ports.address(server))
            })
            .await,
            ports,
        });

        self.members.len() - 1
    }

    /// # Panics
    ///
    /// Panics when the instance is not running.
    pub async fn crash(&mut self, index: usize) {
        self.withdrawn(index).kill().await;
    }

    /// # Panics
    ///
    /// Panics when the instance is not running.
    pub async fn stop(&mut self, index: usize) -> ExitStatus {
        self.withdrawn(index).terminate().await
    }

    /// # Panics
    ///
    /// Panics when an instance exits before it becomes ready.
    pub async fn restart(&mut self, indices: &[usize]) {
        self.launch(indices, InstanceAdmission::Admitted).await;
    }

    /// # Panics
    ///
    /// Panics when an instance exits before it becomes ready.
    pub async fn launch(&mut self, indices: &[usize], admission: InstanceAdmission) {
        for &index in indices {
            self.running.insert(
                index,
                ClusterInstance::spawn(&self.launch, self.members[index].ports),
            );
        }

        await_running(&mut self.running, &self.members, indices, |ports| {
            instance_listening(&self.client, ports)
        })
        .await;

        if admission == InstanceAdmission::Admitted {
            for &index in indices {
                self.admit(index);
            }
        }

        await_running(&mut self.running, &self.members, indices, |ports| {
            instance_ready(&self.client, ports, &self.external_issuer)
        })
        .await;
    }

    /// # Panics
    ///
    /// Panics when a running instance does not exit cleanly.
    pub async fn close(self) {
        for (_index, instance) in self.running {
            assert!(instance.terminate().await.success());
        }

        for member in self.members {
            member.doors.close().await;
        }

        self.front_door.doors.close().await;
        self.external_issuer.close().await;
    }

    fn withdrawn(&mut self, index: usize) -> ClusterInstance {
        self.drain(index);
        self.client = cluster_client(&self.tls.certificate_authority);
        self.running
            .remove(&index)
            .expect("the instance is running")
    }
}
