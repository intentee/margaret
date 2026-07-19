use std::collections::HashMap;
use std::collections::HashSet;

use syn::Path;
use syn::Type;

use crate::attribute_selector::AttributeSelector;
use crate::canonical_path::CanonicalPath;
use crate::identifier::Identifier;
use crate::indexed_item::IndexedItem;
use crate::item_kind::ItemKind;
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
    trait_paths: HashSet<CanonicalPath>,
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
        let mut trait_paths = HashSet::new();

        for item in &items {
            item_paths.insert(item.canonical_path().clone());

            if item.kind() == ItemKind::Trait {
                trait_paths.insert(item.canonical_path().clone());
            }
        }

        Self {
            empty_imports: ModuleImports::default(),
            identifiers: allocate_identifiers(&items),
            imports,
            item_paths,
            items,
            trait_paths,
        }
    }

    #[must_use]
    pub fn has(&self, selector: &AttributeSelector) -> bool {
        self.items
            .iter()
            .flat_map(|item| item.attributes())
            .any(|attribute| selector.matches(attribute.path()))
    }

    #[must_use]
    pub fn is_indexed_trait(&self, path: &CanonicalPath) -> bool {
        self.trait_paths.contains(path)
    }

    #[must_use]
    pub fn items(&self) -> &[IndexedItem] {
        &self.items
    }

    #[must_use]
    pub fn resolve_item_path(&self, item: &IndexedItem, path: &Path) -> Option<CanonicalPath> {
        resolve_path(
            path,
            self.module_of(item),
            self.imports_of(item),
            &self.item_paths,
        )
    }

    #[must_use]
    pub fn resolve_item_type(&self, item: &IndexedItem, declared: &Type) -> Option<CanonicalPath> {
        resolve_type(
            declared,
            self.module_of(item),
            self.imports_of(item),
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

    fn imports_of(&self, item: &IndexedItem) -> &ModuleImports {
        self.imports
            .get(&CanonicalPath::new(self.module_of(item).to_vec()))
            .unwrap_or(&self.empty_imports)
    }

    fn module_of<'index>(&self, item: &'index IndexedItem) -> &'index [String] {
        let segments = item.canonical_path().segments();

        &segments[..segments.len().saturating_sub(1)]
    }
}
