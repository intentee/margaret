pub(crate) enum CachePolicy {
    Immutable,
    Revalidate,
}

impl CachePolicy {
    pub(crate) fn header_value(&self) -> &'static str {
        match self {
            Self::Immutable => "public, max-age=31536000, immutable",
            Self::Revalidate => "no-cache",
        }
    }
}
