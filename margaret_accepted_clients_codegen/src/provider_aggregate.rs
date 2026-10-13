#[derive(Clone, Copy)]
pub enum ProviderAggregate {
    AcceptedResources,
    ProviderSupport,
}

impl ProviderAggregate {
    pub const ALL: [Self; 2] = [Self::AcceptedResources, Self::ProviderSupport];

    #[must_use]
    pub fn constant_name(self) -> &'static str {
        match self {
            Self::AcceptedResources => "ACCEPTED_RESOURCES",
            Self::ProviderSupport => "PROVIDER_SUPPORT",
        }
    }

    #[must_use]
    pub fn module_name(self) -> &'static str {
        match self {
            Self::AcceptedResources => "accepted_resources",
            Self::ProviderSupport => "provider_support",
        }
    }
}
