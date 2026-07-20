use margaret_attributes::canonical_path::CanonicalPath;

#[derive(Debug)]
pub enum TransitionTrigger {
    InternalEvent {
        canonical_path: CanonicalPath,
        variant: String,
    },
    Message {
        canonical_path: CanonicalPath,
        method: String,
        variant: String,
    },
}

impl TransitionTrigger {
    #[must_use]
    pub fn canonical_path(&self) -> &CanonicalPath {
        match self {
            Self::InternalEvent { canonical_path, .. }
            | Self::Message { canonical_path, .. } => canonical_path,
        }
    }

    #[must_use]
    pub fn variant(&self) -> &str {
        match self {
            Self::InternalEvent { variant, .. } | Self::Message { variant, .. } => variant,
        }
    }

    #[must_use]
    pub fn wire_method(&self) -> Option<&str> {
        match self {
            Self::InternalEvent { .. } => None,
            Self::Message { method, .. } => Some(method),
        }
    }
}

#[cfg(test)]
mod tests {
    use margaret_attributes::canonical_path::CanonicalPath;

    use super::TransitionTrigger;

    fn path(leaf: &str) -> CanonicalPath {
        CanonicalPath::new(vec!["crate".to_owned(), leaf.to_owned()])
    }

    #[test]
    fn exposes_message_and_internal_event_facets() {
        let message = TransitionTrigger::Message {
            canonical_path: path("Speak"),
            method: "speak".to_owned(),
            variant: "Speak".to_owned(),
        };
        let event = TransitionTrigger::InternalEvent {
            canonical_path: path("Done"),
            variant: "Done".to_owned(),
        };

        assert_eq!(message.canonical_path().to_string(), "crate::Speak");
        assert_eq!(event.canonical_path().to_string(), "crate::Done");
        assert_eq!(message.variant(), "Speak");
        assert_eq!(event.variant(), "Done");
        assert_eq!(message.wire_method(), Some("speak"));
        assert_eq!(event.wire_method(), None);
    }
}
