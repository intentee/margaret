use serde::Deserialize;
use serde::de::IgnoredAny;

#[derive(Deserialize)]
pub(crate) struct PrivateKeyMembers {
    #[serde(default)]
    d: Option<IgnoredAny>,
    #[serde(default)]
    dp: Option<IgnoredAny>,
    #[serde(default)]
    dq: Option<IgnoredAny>,
    #[serde(default)]
    oth: Option<IgnoredAny>,
    #[serde(default)]
    p: Option<IgnoredAny>,
    #[serde(default)]
    q: Option<IgnoredAny>,
    #[serde(default)]
    qi: Option<IgnoredAny>,
}

impl PrivateKeyMembers {
    pub(crate) fn discloses_private_key(&self) -> bool {
        let Self {
            d,
            dp,
            dq,
            oth,
            p,
            q,
            qi,
        } = self;

        [d, dp, dq, oth, p, q, qi].into_iter().any(Option::is_some)
    }
}
