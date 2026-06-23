use crate::attribute_query::AttributeQuery;
use crate::canonical_path::CanonicalPath;
use crate::item_kind::ItemKind;
use crate::matched_attribute::MatchedAttribute;

pub struct SelectedItem<'index> {
    matched: MatchedAttribute<'index>,
}

impl<'index> SelectedItem<'index> {
    pub(crate) fn new(matched: MatchedAttribute<'index>) -> Self {
        Self { matched }
    }

    pub fn attribute(&self) -> &MatchedAttribute<'index> {
        &self.matched
    }

    pub fn attributes(&self) -> AttributeQuery<'index> {
        AttributeQuery::new(self.matched.item())
    }

    pub fn canonical_path(&self) -> &CanonicalPath {
        self.matched.item().canonical_path()
    }

    pub fn kind(&self) -> ItemKind {
        self.matched.item().kind()
    }
}
