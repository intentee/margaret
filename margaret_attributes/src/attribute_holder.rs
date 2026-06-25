use syn::Attribute;

use crate::indexed_item::IndexedItem;
use crate::indexed_method::IndexedMethod;

pub enum AttributeHolder {
    Item(IndexedItem),
    Method(IndexedMethod),
}

impl AttributeHolder {
    pub fn attributes(&self) -> &[Attribute] {
        match self {
            AttributeHolder::Item(item) => item.attributes(),
            AttributeHolder::Method(method) => method.attributes(),
        }
    }

    pub fn target_path(&self) -> String {
        match self {
            AttributeHolder::Item(item) => item.canonical_path().to_string(),
            AttributeHolder::Method(method) => {
                format!("{}::{}", method.self_type_path(), method.identifier())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use syn::Signature;
    use syn::parse_quote;

    use crate::attribute_holder::AttributeHolder;
    use crate::canonical_path::CanonicalPath;
    use crate::indexed_item::IndexedItem;
    use crate::indexed_method::IndexedMethod;
    use crate::item_kind::ItemKind;
    use crate::struct_shape::StructShape;

    fn item_holder() -> AttributeHolder {
        AttributeHolder::Item(IndexedItem::new(
            ItemKind::Struct(StructShape::Unit),
            "Config".to_string(),
            CanonicalPath::new(vec!["crate".to_string(), "Config".to_string()]),
            vec![parse_quote!(#[singleton])],
        ))
    }

    fn method_holder() -> AttributeHolder {
        let signature: Signature = parse_quote!(fn create() -> Self);

        AttributeHolder::Method(IndexedMethod::new(
            CanonicalPath::new(vec!["crate".to_string(), "Config".to_string()]),
            "create".to_string(),
            vec![parse_quote!(#[constructor])],
            signature,
        ))
    }

    #[test]
    fn item_target_path_is_its_canonical_path() {
        assert_eq!(item_holder().target_path(), "crate::Config");
    }

    #[test]
    fn method_target_path_joins_owner_type_and_method_identifier() {
        assert_eq!(method_holder().target_path(), "crate::Config::create");
    }

    #[test]
    fn item_attributes_are_the_items_own_attributes() {
        assert!(item_holder().attributes()[0].path().is_ident("singleton"));
    }

    #[test]
    fn method_attributes_are_the_methods_own_attributes() {
        assert!(
            method_holder().attributes()[0]
                .path()
                .is_ident("constructor")
        );
    }
}
