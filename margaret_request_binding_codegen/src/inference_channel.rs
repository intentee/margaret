#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) enum InferenceChannel {
    BearerCredential,
    Session,
}
