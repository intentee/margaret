use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;

use crate::constants::SIGNING_KEY_SET_NAME;

#[must_use]
pub fn signing_keys_associated_data(generation: SigningKeysGeneration) -> Vec<u8> {
    [
        SIGNING_KEY_SET_NAME.as_bytes(),
        &generation.value().to_be_bytes(),
    ]
    .concat()
}
