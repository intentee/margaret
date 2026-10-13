use crate::consumed_sessions_declaration::ConsumedSessionsDeclaration;
use crate::issued_sessions_declaration::IssuedSessionsDeclaration;

pub(crate) enum SessionsDeclaration<'index> {
    Consumed(ConsumedSessionsDeclaration<'index>),
    Issued(IssuedSessionsDeclaration<'index>),
}

impl SessionsDeclaration<'_> {
    pub(crate) fn anchor(&self) -> String {
        match self {
            Self::Consumed(ConsumedSessionsDeclaration { anchor, .. })
            | Self::Issued(IssuedSessionsDeclaration { anchor, .. }) => {
                anchor.canonical_path().to_string()
            }
        }
    }
}
