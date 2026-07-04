use std::collections::HashMap;

pub struct Forward {
    name: &'static str,
    path_params: HashMap<String, String>,
}

impl Forward {
    pub(crate) fn new(name: &'static str, path_params: HashMap<String, String>) -> Self {
        Self { name, path_params }
    }

    pub(crate) fn into_path_params(self) -> HashMap<String, String> {
        self.path_params
    }

    pub(crate) fn name(&self) -> &'static str {
        self.name
    }
}
