use crate::ignored_key_reason::IgnoredKeyReason;
use crate::key_disclosure::KeyDisclosure;

pub(crate) enum KeyExclusion {
    Disclosed(KeyDisclosure),
    Ignored(IgnoredKeyReason),
}
