use std::collections::BTreeMap;

pub struct Forward {
    name: &'static str,
    path_params: BTreeMap<String, String>,
}

impl Forward {
    #[must_use]
    pub fn new(name: &'static str, path_params: BTreeMap<String, String>) -> Self {
        Self { name, path_params }
    }

    pub(crate) fn into_path_params(self) -> BTreeMap<String, String> {
        self.path_params
    }

    pub(crate) fn name(&self) -> &'static str {
        self.name
    }
}
