use std::collections::BTreeSet;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use margaret_container::generated_source::GeneratedSource;

use crate::generated_module::GeneratedModule;

fn remove_stale_sources(directory: &Path, written: &BTreeSet<PathBuf>) {
    for entry in fs::read_dir(directory).expect("the generated directory is read") {
        let path = entry.expect("the generated directory entry is read").path();

        if path.extension() == Some(OsStr::new("rs")) && !written.contains(&path) {
            fs::remove_file(&path).expect("the stale generated source is removed");
        }
    }
}

#[derive(Debug)]
pub struct GeneratedCode {
    modules: Vec<GeneratedModule>,
}

impl GeneratedCode {
    pub fn new(modules: Vec<GeneratedModule>) -> Self {
        Self { modules }
    }

    pub fn modules(&self) -> &[GeneratedModule] {
        &self.modules
    }

    pub fn write_to(&self, directory: &Path) {
        fs::create_dir_all(directory).expect("the generated directory is created");

        let mut written = BTreeSet::new();

        for module in &self.modules {
            let path = directory.join(format!("{}.rs", module.name()));

            GeneratedSource::new(module.source().to_string())
                .write_if_changed(&path)
                .expect("the generated source is written");
            written.insert(path);
        }

        remove_stale_sources(directory, &written);
    }
}
