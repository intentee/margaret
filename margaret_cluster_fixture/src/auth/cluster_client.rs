use margaret::framework::macros::oauth_client;

#[oauth_client(cluster, admitted_as = cluster_app)]
pub struct ClusterClient;
