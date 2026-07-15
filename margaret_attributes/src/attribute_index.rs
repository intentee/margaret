use std::collections::HashMap;
use std::collections::HashSet;

use syn::Path;
use syn::Type;

use crate::attribute_location::AttributeLocation;
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
    locations_by_leaf: HashMap<String, Vec<AttributeLocation>>,
    struct_paths: HashSet<CanonicalPath>,
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
        let mut locations_by_leaf: HashMap<String, Vec<AttributeLocation>> = HashMap::new();
        let mut item_paths = HashSet::new();
        let mut struct_paths = HashSet::new();
        let mut trait_paths = HashSet::new();

        for (item_index, item) in items.iter().enumerate() {
            for (attribute_index, attribute) in item.attributes().iter().enumerate() {
                let leaf = attribute
                    .path()
                    .segments
                    .last()
                    .expect("an attribute path has at least one segment")
                    .ident
                    .to_string();

                locations_by_leaf
                    .entry(leaf)
                    .or_default()
                    .push(AttributeLocation::new(item_index, attribute_index));
            }

            item_paths.insert(item.canonical_path().clone());

            if item.kind().is_struct() {
                struct_paths.insert(item.canonical_path().clone());
            } else if item.kind() == ItemKind::Trait {
                trait_paths.insert(item.canonical_path().clone());
            }
        }

        Self {
            empty_imports: ModuleImports::default(),
            identifiers: allocate_identifiers(&items),
            imports,
            item_paths,
            items,
            locations_by_leaf,
            struct_paths,
            trait_paths,
        }
    }

    pub fn field_name(&self, path: &CanonicalPath) -> &str {
        self.identifier(path).field()
    }

    pub fn has(&self, selector: &AttributeSelector) -> bool {
        let leaf = selector.leaf_ident().to_string();
        let Some(locations) = self.locations_by_leaf.get(&leaf) else {
            return false;
        };

        locations.iter().any(|location| {
            selector.matches(self.items[location.item()].attributes()[location.attribute()].path())
        })
    }

    pub fn is_indexed_struct(&self, path: &CanonicalPath) -> bool {
        self.struct_paths.contains(path)
    }

    pub fn is_indexed_trait(&self, path: &CanonicalPath) -> bool {
        self.trait_paths.contains(path)
    }

    pub fn items(&self) -> &[IndexedItem] {
        &self.items
    }

    pub fn resolve_item_path(&self, item: &IndexedItem, path: &Path) -> Option<CanonicalPath> {
        resolve_path(
            path,
            self.module_of(item),
            self.imports_of(item),
            &self.item_paths,
        )
    }

    pub fn resolve_item_type(&self, item: &IndexedItem, declared: &Type) -> Option<CanonicalPath> {
        resolve_type(
            declared,
            self.module_of(item),
            self.imports_of(item),
            &self.item_paths,
        )
    }

    pub fn select(&self, selector: &AttributeSelector) -> Vec<MatchedAttribute<'_>> {
        let leaf = selector.leaf_ident().to_string();
        let Some(locations) = self.locations_by_leaf.get(&leaf) else {
            return Vec::new();
        };

        locations
            .iter()
            .filter_map(|location| {
                let item = &self.items[location.item()];
                let attribute_index = location.attribute();

                selector
                    .matches(item.attributes()[attribute_index].path())
                    .then(|| MatchedAttribute::new(item, attribute_index))
            })
            .collect()
    }

    pub fn type_name(&self, path: &CanonicalPath) -> &str {
        self.identifier(path).type_name()
    }

    fn identifier(&self, path: &CanonicalPath) -> &Identifier {
        self.identifiers
            .get(path)
            .expect("every indexed struct is assigned an identifier")
    }

    fn imports_of(&self, item: &IndexedItem) -> &ModuleImports {
        self.imports
            .get(&CanonicalPath::new(self.module_of(item).to_vec()))
            .unwrap_or(&self.empty_imports)
    }

    fn module_of<'index>(&self, item: &'index IndexedItem) -> &'index [String] {
        let segments = item.canonical_path().segments();

        &segments[..segments.len() - 1]
    }
}
