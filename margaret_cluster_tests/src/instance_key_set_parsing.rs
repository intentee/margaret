use margaret::framework::jws_verification::key_set_document_parsing::KeySetDocumentParsing;
use margaret::framework::jws_verification::verification_key_set::VerificationKeySet;

use crate::cluster::Cluster;
use crate::instance_key_set::instance_key_set;

pub async fn instance_key_set_parsing(cluster: &Cluster, index: usize) -> KeySetDocumentParsing {
    VerificationKeySet::parse(&instance_key_set(cluster, index).await)
}
