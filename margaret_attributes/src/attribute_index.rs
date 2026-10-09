use std::collections::HashMap;

use syn::Path;
use syn::Type;

use crate::canonical_path::CanonicalPath;
use crate::field_base::field_base;
use crate::framework_attribute::FrameworkAttribute;
use crate::identifier::Identifier;
use crate::indexed_item::IndexedItem;
use crate::indexed_struct::IndexedStruct;
use crate::item_kind::ItemKind;
use crate::matched_attribute::MatchedAttribute;
use crate::name_allocator::NameAllocator;
use crate::path_resolver::PathResolver;

fn module_of(item: &IndexedItem) -> &[String] {
    item.canonical_path()
        .segments()
        .split_last()
        .map_or(&[], |(_, module)| module)
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
        .map(|path| (path.clone(), allocator.allocate(&field_base(path))))
        .collect()
}

pub struct AttributeIndex {
    identifiers: HashMap<CanonicalPath, Identifier>,
    items: Vec<IndexedItem>,
    resolver: PathResolver,
}

impl AttributeIndex {
    pub(crate) fn new(items: Vec<IndexedItem>, resolver: PathResolver) -> Self {
        Self {
            identifiers: allocate_identifiers(&items),
            items,
            resolver,
        }
    }

    #[must_use]
    pub fn has_framework_attribute(&self, attribute: FrameworkAttribute) -> bool {
        self.select_framework_attribute(attribute).next().is_some()
    }

    #[must_use]
    pub fn indexed_struct<'index>(
        &'index self,
        item: &'index IndexedItem,
    ) -> Option<IndexedStruct<'index>> {
        match item.kind() {
            ItemKind::Struct(shape) => self
                .identifiers
                .get(item.canonical_path())
                .map(|identifier| IndexedStruct { identifier, shape }),
            ItemKind::Enum | ItemKind::Function | ItemKind::Module | ItemKind::Trait => None,
        }
    }

    #[must_use]
    pub fn is_imported(&self, target: &CanonicalPath) -> bool {
        self.resolver.is_imported(target)
    }

    #[must_use]
    pub fn item(&self, path: &CanonicalPath) -> Option<&IndexedItem> {
        self.items
            .binary_search_by(|item| item.canonical_path().cmp(path))
            .ok()
            .and_then(|index| self.items.get(index))
    }

    #[must_use]
    pub fn items(&self) -> &[IndexedItem] {
        &self.items
    }

    #[must_use]
    pub fn reserved_allocator(&self) -> NameAllocator {
        let mut allocator = NameAllocator::new();

        for identifier in self.identifiers.values() {
            allocator.reserve(identifier.field());
        }

        allocator
    }

    #[must_use]
    pub fn resolve_item_path(&self, item: &IndexedItem, path: &Path) -> Option<CanonicalPath> {
        self.resolve_module_path(module_of(item), path)
    }

    #[must_use]
    pub fn resolve_item_type(&self, item: &IndexedItem, declared: &Type) -> Option<CanonicalPath> {
        self.resolve_module_type(module_of(item), declared)
    }

    #[must_use]
    pub fn resolve_module_path(&self, module: &[String], path: &Path) -> Option<CanonicalPath> {
        self.resolver.resolve_path(module, path)
    }

    #[must_use]
    pub fn resolve_module_type(&self, module: &[String], declared: &Type) -> Option<CanonicalPath> {
        self.resolver.resolve_type(module, declared)
    }

    pub fn select_framework_attribute(
        &self,
        attribute: FrameworkAttribute,
    ) -> impl Iterator<Item = MatchedAttribute<'_>> {
        self.items.iter().flat_map(move |item| {
            item.attributes()
                .iter()
                .filter(move |indexed| indexed.framework_attribute() == Some(attribute))
                .map(move |indexed| MatchedAttribute::new(item, indexed))
        })
    }

    #[must_use]
    pub fn struct_identifier(&self, path: &CanonicalPath) -> Option<&Identifier> {
        self.identifiers.get(path)
    }
}
