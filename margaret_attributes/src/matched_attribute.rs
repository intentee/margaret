use syn::Attribute;

use crate::attribute_args::AttributeArgs;
use crate::attribute_error::AttributeError;
use crate::attribute_holder::AttributeHolder;
use crate::format_path::format_path;

pub struct MatchedAttribute<'index> {
    attribute: &'index Attribute,
    holder: &'index AttributeHolder,
}

impl<'index> MatchedAttribute<'index> {
    pub(crate) fn new(holder: &'index AttributeHolder, attribute: &'index Attribute) -> Self {
        Self { attribute, holder }
    }

    pub fn holder(&self) -> &'index AttributeHolder {
        self.holder
    }

    pub fn path(&self) -> String {
        format_path(self.attribute.path())
    }

    pub fn args(&self) -> Result<AttributeArgs, AttributeError> {
        AttributeArgs::from_attribute(self.attribute)
    }
}

#[cfg(test)]
mod tests {
    use syn::Attribute;
    use syn::parse_quote;

    use crate::attribute_holder::AttributeHolder;
    use crate::canonical_path::CanonicalPath;
    use crate::indexed_item::IndexedItem;
    use crate::item_kind::ItemKind;
    use crate::matched_attribute::MatchedAttribute;

    fn holder_bearing(attribute: Attribute) -> AttributeHolder {
        AttributeHolder::Item(IndexedItem::new(
            ItemKind::Struct,
            "Service".to_string(),
            CanonicalPath::new(vec!["crate".to_string(), "Service".to_string()]),
            vec![attribute],
        ))
    }

    #[test]
    fn path_is_the_attributes_written_path() {
        let holder = holder_bearing(parse_quote!(#[ns::tagged]));
        let matched = MatchedAttribute::new(&holder, &holder.attributes()[0]);

        assert_eq!(matched.path(), "ns::tagged");
    }

    #[test]
    fn args_parse_the_matched_attribute() {
        let holder = holder_bearing(parse_quote!(#[singleton]));
        let matched = MatchedAttribute::new(&holder, &holder.attributes()[0]);

        assert!(matched.args().expect("the arguments parse").is_empty());
    }

    #[test]
    fn holder_is_the_owning_holder() {
        let holder = holder_bearing(parse_quote!(#[singleton]));
        let matched = MatchedAttribute::new(&holder, &holder.attributes()[0]);

        assert_eq!(matched.holder().target_path(), "crate::Service");
    }
}
