use crate::serve_dispatch::ServeDispatch;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FrameworkCommand {
    Schema,
    Serve,
}

impl FrameworkCommand {
    #[must_use]
    pub fn generated(serve: &ServeDispatch, has_models: bool) -> Vec<Self> {
        let mut generated = Vec::new();

        if has_models {
            generated.push(Self::Schema);
        }

        if matches!(serve, ServeDispatch::Served(_)) {
            generated.push(Self::Serve);
        }

        generated
    }

    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Schema => "schema",
            Self::Serve => "serve",
        }
    }
}
