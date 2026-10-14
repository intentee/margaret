use margaret::framework::macros::encrypts_signing_keys;

#[encrypts_signing_keys(key_from = "FIXTURE_SIGNING_KEYS_ENCRYPTION_KEY")]
pub struct FixtureSigningKeysEncryption;
