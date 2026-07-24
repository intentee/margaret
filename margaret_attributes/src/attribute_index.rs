use std::collections::HashMap;
use std::collections::HashSet;

use syn::Path;
use syn::Type;

use crate::attribute_selector::AttributeSelector;
use crate::canonical_path::CanonicalPath;
use crate::identifier::Identifier;
use crate::indexed_item::IndexedItem;
use crate::matched_attribute::MatchedAttribute;
use crate::module_imports::ModuleImports;
use crate::name_allocator::NameAllocator;
use crate::resolve_path::resolve_path;
use crate::resolve_type::resolve_type;

pub struct AttributeIndex {
    empty_imports: ModuleImports,
    identifiers: HashMap<CanonicalPath, Identifier>,
    imports: HashMap<CanonicalPath, ModuleImports>,
    item_paths: HashSet<CanonicalPath>,
    items: Vec<IndexedItem>,
}

fn allocate_identifiers(items: &[IndexedItem]) -> HashMap<CanonicalPath, Identifier> {
    let mut paths: Vec<&CanonicalPath> = items
        .iter()
        .filter(|item| item.kind().is_struct())
        .map(IndexedItem::canonical_path)
        .collect();

    paths.sort();

    let mut allocator = NameAllocator::new();

    paths
        .into_iter()
        .map(|path| (path.clone(), allocator.allocate(&path.field_name())))
        .collect()
}

impl AttributeIndex {
    pub(crate) fn new(
        items: Vec<IndexedItem>,
        imports: HashMap<CanonicalPath, ModuleImports>,
    ) -> Self {
        let mut item_paths = HashSet::new();

        for item in &items {
            item_paths.insert(item.canonical_path().clone());
        }

        Self {
            empty_imports: ModuleImports::default(),
            identifiers: allocate_identifiers(&items),
            imports,
            item_paths,
            items,
        }
    }

    #[must_use]
    pub fn has(&self, selector: &AttributeSelector) -> bool {
        self.items.iter().any(|item| item.has_attribute(selector))
    }

    #[must_use]
    pub fn items(&self) -> &[IndexedItem] {
        &self.items
    }

    #[must_use]
    pub fn resolve_item_path(&self, item: &IndexedItem, path: &Path) -> Option<CanonicalPath> {
        self.resolve_module_path(self.module_of(item), path)
    }

    #[must_use]
    pub fn resolve_item_type(&self, item: &IndexedItem, declared: &Type) -> Option<CanonicalPath> {
        self.resolve_module_type(self.module_of(item), declared)
    }

    #[must_use]
    pub fn resolve_module_path(&self, module: &[String], path: &Path) -> Option<CanonicalPath> {
        resolve_path(
            path,
            module,
            self.imports_for_module(module),
            &self.item_paths,
        )
    }

    #[must_use]
    pub fn resolve_module_type(&self, module: &[String], declared: &Type) -> Option<CanonicalPath> {
        resolve_type(
            declared,
            module,
            self.imports_for_module(module),
            &self.item_paths,
        )
    }

    #[must_use]
    pub fn select(&self, selector: &AttributeSelector) -> Vec<MatchedAttribute<'_>> {
        self.items
            .iter()
            .flat_map(|item| {
                item.attributes()
                    .iter()
                    .filter(|attribute| selector.matches(attribute.path()))
                    .map(move |attribute| MatchedAttribute::new(item, attribute))
            })
            .collect()
    }

    #[must_use]
    pub fn struct_identifier(&self, path: &CanonicalPath) -> Option<&Identifier> {
        self.identifiers.get(path)
    }

    fn imports_for_module(&self, module: &[String]) -> &ModuleImports {
        self.imports
            .get(&CanonicalPath::new(module.to_vec()))
            .unwrap_or(&self.empty_imports)
    }

    fn module_of<'index>(&self, item: &'index IndexedItem) -> &'index [String] {
        let segments = item.canonical_path().segments();

        &segments[..segments.len().saturating_sub(1)]
    }
}
