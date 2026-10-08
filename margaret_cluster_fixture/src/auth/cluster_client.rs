use margaret::framework::macros::acts_as_oauth_client;

#[acts_as_oauth_client(cluster, admitted_as = cluster_app)]
pub struct ClusterClient;
