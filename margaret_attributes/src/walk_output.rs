use std::collections::HashMap;

use crate::canonical_path::CanonicalPath;
use crate::indexed_item::IndexedItem;
use crate::module_imports::ModuleImports;

pub(crate) struct WalkOutput {
    pub(crate) imports: HashMap<CanonicalPath, ModuleImports>,
    pub(crate) items: Vec<IndexedItem>,
}
