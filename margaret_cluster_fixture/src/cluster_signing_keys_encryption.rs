use margaret::framework::macros::encrypts_signing_keys;

#[encrypts_signing_keys(key_from = "MARGARET_CLUSTER_SIGNING_KEYS_ENCRYPTION_KEY")]
pub struct ClusterSigningKeysEncryption;
