use std::collections::BTreeSet;

use crate::jwks_server_part::JwksServerPart;

#[derive(Default)]
pub struct JwksServerModule {
    present: BTreeSet<JwksServerPart>,
}

impl JwksServerModule {
    #[must_use]
    pub fn contains(&self, part: JwksServerPart) -> bool {
        self.present.contains(&part)
    }

    pub fn enable_if(&mut self, part: JwksServerPart, present: bool) {
        if present {
            self.present.insert(part);
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.present.is_empty()
    }
}
