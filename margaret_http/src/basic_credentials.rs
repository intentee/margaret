use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;

use headers::Authorization;
use headers::authorization::Basic;

#[derive(PartialEq)]
pub struct BasicCredentials {
    authorization: Authorization<Basic>,
}

impl BasicCredentials {
    pub(crate) fn new(authorization: Authorization<Basic>) -> Self {
        Self { authorization }
    }

    #[must_use]
    pub fn password(&self) -> &str {
        self.authorization.password()
    }

    #[must_use]
    pub fn user_id(&self) -> &str {
        self.authorization.username()
    }
}

impl Debug for BasicCredentials {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("BasicCredentials")
            .field("user_id", &self.user_id())
            .finish_non_exhaustive()
    }
}
