use std::collections::HashMap;

use cookie::Cookie;

pub struct Forward {
    cookies: Vec<Cookie<'static>>,
    name: &'static str,
    path_params: HashMap<String, String>,
}

impl Forward {
    #[must_use]
    pub fn new(name: &'static str, path_params: HashMap<String, String>) -> Self {
        Self {
            cookies: Vec::new(),
            name,
            path_params,
        }
    }

    pub(crate) fn carried_cookies(&self) -> &[Cookie<'static>] {
        &self.cookies
    }

    pub(crate) fn carrying(mut self, cookies: &[Cookie<'static>]) -> Self {
        self.cookies.extend_from_slice(cookies);

        self
    }

    pub(crate) fn into_path_params(self) -> HashMap<String, String> {
        self.path_params
    }

    pub(crate) fn name(&self) -> &'static str {
        self.name
    }
}
