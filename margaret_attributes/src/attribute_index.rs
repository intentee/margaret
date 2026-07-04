use std::collections::HashMap;

use crate::attribute_location::AttributeLocation;
use crate::attribute_selector::AttributeSelector;
use crate::canonical_path::CanonicalPath;
use crate::identifier::Identifier;
use crate::indexed_item::IndexedItem;
use crate::item_kind::ItemKind;
use crate::matched_attribute::MatchedAttribute;
use crate::name_allocator::NameAllocator;
use crate::resolution_index::ResolutionIndex;

pub struct AttributeIndex {
    identifiers: HashMap<CanonicalPath, Identifier>,
    items: Vec<IndexedItem>,
    locations_by_leaf: HashMap<String, Vec<AttributeLocation>>,
    struct_resolution: ResolutionIndex,
    trait_resolution: ResolutionIndex,
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
    pub(crate) fn new(items: Vec<IndexedItem>) -> Self {
        let mut locations_by_leaf: HashMap<String, Vec<AttributeLocation>> = HashMap::new();
        let mut struct_candidates = Vec::new();
        let mut trait_candidates = Vec::new();

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

            if item.kind().is_struct() {
                struct_candidates.push(item.canonical_path().clone());
            } else if item.kind() == ItemKind::Trait {
                trait_candidates.push(item.canonical_path().clone());
            }
        }

        Self {
            identifiers: allocate_identifiers(&items),
            items,
            locations_by_leaf,
            struct_resolution: ResolutionIndex::new(struct_candidates),
            trait_resolution: ResolutionIndex::new(trait_candidates),
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

    pub fn items(&self) -> &[IndexedItem] {
        &self.items
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

    pub fn struct_resolution(&self) -> &ResolutionIndex {
        &self.struct_resolution
    }

    pub fn trait_resolution(&self) -> &ResolutionIndex {
        &self.trait_resolution
    }

    pub fn type_name(&self, path: &CanonicalPath) -> &str {
        self.identifier(path).type_name()
    }

    fn identifier(&self, path: &CanonicalPath) -> &Identifier {
        self.identifiers
            .get(path)
            .expect("every indexed struct is assigned an identifier")
    }
}
