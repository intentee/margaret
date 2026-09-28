use margaret_tag_codegen::jwks_secret_store_target::JwksSecretStoreTarget;

pub(crate) enum JwksStoreMarking {
    Marked(JwksSecretStoreTarget),
    Unmarked,
}
