use std::collections::HashMap;

use crate::attribute_location::AttributeLocation;
use crate::attribute_selector::AttributeSelector;
use crate::indexed_item::IndexedItem;
use crate::item_kind::ItemKind;
use crate::matched_attribute::MatchedAttribute;
use crate::resolution_index::ResolutionIndex;

pub struct AttributeIndex {
    items: Vec<IndexedItem>,
    locations_by_leaf: HashMap<String, Vec<AttributeLocation>>,
    struct_resolution: ResolutionIndex,
    trait_resolution: ResolutionIndex,
}

impl AttributeIndex {
    pub(crate) fn new(items: Vec<IndexedItem>) -> Self {
        let mut locations_by_leaf: HashMap<String, Vec<AttributeLocation>> = HashMap::new();

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
        }

        let struct_resolution = ResolutionIndex::new(
            items
                .iter()
                .filter(|item| item.kind().is_struct())
                .map(|item| item.canonical_path().clone()),
        );
        let trait_resolution = ResolutionIndex::new(
            items
                .iter()
                .filter(|item| item.kind() == ItemKind::Trait)
                .map(|item| item.canonical_path().clone()),
        );

        Self {
            items,
            locations_by_leaf,
            struct_resolution,
            trait_resolution,
        }
    }

    pub fn items(&self) -> &[IndexedItem] {
        &self.items
    }

    pub fn struct_resolution(&self) -> &ResolutionIndex {
        &self.struct_resolution
    }

    pub fn trait_resolution(&self) -> &ResolutionIndex {
        &self.trait_resolution
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
                let attribute = &item.attributes()[location.attribute()];

                selector
                    .matches(attribute.path())
                    .then(|| MatchedAttribute::new(item, attribute))
            })
            .collect()
    }

    pub fn has(&self, selector: &AttributeSelector) -> bool {
        !self.select(selector).is_empty()
    }
}
