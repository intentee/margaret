use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) struct DeclaredServer {
    name: String,
    path: CanonicalPath,
}

impl DeclaredServer {
    pub(crate) fn new(name: String, path: CanonicalPath) -> Self {
        Self { name, path }
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn path(&self) -> &CanonicalPath {
        &self.path
    }
}
