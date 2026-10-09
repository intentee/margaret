use crate::cluster_doors::ClusterDoors;
use crate::instance_ports::InstancePorts;

pub struct ClusterMember {
    pub doors: ClusterDoors,
    pub ports: InstancePorts,
}
