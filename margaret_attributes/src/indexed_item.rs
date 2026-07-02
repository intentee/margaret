use syn::Attribute;

use crate::attribute_selector::AttributeSelector;
use crate::canonical_path::CanonicalPath;
use crate::indexed_associated_type::IndexedAssociatedType;
use crate::indexed_method::IndexedMethod;
use crate::item_kind::ItemKind;

pub struct IndexedItem {
    associated_types: Vec<IndexedAssociatedType>,
    attributes: Vec<Attribute>,
    canonical_path: CanonicalPath,
    identifier: String,
    kind: ItemKind,
    methods: Vec<IndexedMethod>,
}

impl IndexedItem {
    pub(crate) fn new(
        kind: ItemKind,
        identifier: String,
        canonical_path: CanonicalPath,
        attributes: Vec<Attribute>,
    ) -> Self {
        Self {
            associated_types: Vec::new(),
            attributes,
            canonical_path,
            identifier,
            kind,
            methods: Vec::new(),
        }
    }

    pub(crate) fn add_associated_type(&mut self, associated_type: IndexedAssociatedType) {
        self.associated_types.push(associated_type);
    }

    pub(crate) fn add_method(&mut self, method: IndexedMethod) {
        self.methods.push(method);
    }

    pub(crate) fn sort_members(&mut self) {
        self.associated_types
            .sort_by(|left, right| left.name().cmp(right.name()));
        self.methods
            .sort_by(|left, right| left.identifier().cmp(right.identifier()));
    }

    pub fn associated_types(&self) -> &[IndexedAssociatedType] {
        &self.associated_types
    }

    pub fn attributes(&self) -> &[Attribute] {
        &self.attributes
    }

    pub fn canonical_path(&self) -> &CanonicalPath {
        &self.canonical_path
    }

    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    pub fn kind(&self) -> ItemKind {
        self.kind
    }

    pub fn methods(&self) -> &[IndexedMethod] {
        &self.methods
    }

    pub fn method_matching(&self, selector: &AttributeSelector) -> Option<&IndexedMethod> {
        self.methods.iter().find(|method| {
            method
                .attributes()
                .iter()
                .any(|attribute| selector.matches(attribute.path()))
        })
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::IndexedItem;
    use crate::attribute_selector::AttributeSelector;
    use crate::canonical_path::CanonicalPath;
    use crate::indexed_method::IndexedMethod;
    use crate::item_kind::ItemKind;
    use crate::struct_shape::StructShape;

    fn item_with_marked_method(method_attributes: Vec<syn::Attribute>) -> IndexedItem {
        let mut item = IndexedItem::new(
            ItemKind::Struct(StructShape::Unit),
            "Widget".to_string(),
            CanonicalPath::new(vec!["crate".to_string(), "Widget".to_string()]),
            Vec::new(),
        );

        item.add_method(IndexedMethod::new(
            "run".to_string(),
            method_attributes,
            parse_quote!(fn run(&self)),
        ));

        item
    }

    fn selector(input: &str) -> AttributeSelector {
        AttributeSelector::parse(input).expect("the selector parses")
    }

    #[test]
    fn method_matching_finds_a_method_bearing_the_selector() {
        let item = item_with_marked_method(vec![parse_quote!(#[runner])]);

        assert!(item.method_matching(&selector("runner")).is_some());
    }

    #[test]
    fn method_matching_returns_none_when_no_method_bears_the_selector() {
        let item = item_with_marked_method(vec![parse_quote!(#[runner])]);

        assert!(item.method_matching(&selector("responder")).is_none());
    }
}
