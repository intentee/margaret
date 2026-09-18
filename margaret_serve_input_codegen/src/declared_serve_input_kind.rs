use margaret_attributes::framework_attribute::FrameworkAttribute;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum DeclaredServeInputKind {
    ConsoleArgument,
    EnvironmentVariable,
    SpiffeHttpClient,
    SpiffeWebSocketClient,
}

impl DeclaredServeInputKind {
    pub(crate) const ALL: [Self; 4] = [
        Self::ConsoleArgument,
        Self::EnvironmentVariable,
        Self::SpiffeHttpClient,
        Self::SpiffeWebSocketClient,
    ];

    pub(crate) fn attribute(self) -> FrameworkAttribute {
        match self {
            Self::ConsoleArgument => FrameworkAttribute::ConsoleArgument,
            Self::EnvironmentVariable => FrameworkAttribute::EnvironmentVariable,
            Self::SpiffeHttpClient => FrameworkAttribute::SpiffeHttpClient,
            Self::SpiffeWebSocketClient => FrameworkAttribute::SpiffeWebsocketClient,
        }
    }

    pub(crate) fn name(self) -> &'static str {
        self.attribute().name()
    }
}
