use quote::ToTokens;
use syn::ReturnType;
use syn::Token;
use syn::Type;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::indexed_method::IndexedMethod;

use crate::constructor_outcome::ConstructorOutcome;

fn returns_the_constructed_struct(
    index: &AttributeIndex,
    item: &IndexedItem,
    returned: &Type,
) -> bool {
    syn::parse2::<Token![Self]>(returned.to_token_stream()).is_ok()
        || index
            .resolve_item_type(item, returned)
            .is_some_and(|resolved| resolved == *item.canonical_path())
}

pub(crate) fn declared_constructor_outcome(
    index: &AttributeIndex,
    item: &IndexedItem,
    constructor: &IndexedMethod,
) -> ConstructorOutcome {
    match &constructor.signature().output {
        ReturnType::Type(_, returned) if returns_the_constructed_struct(index, item, returned) => {
            ConstructorOutcome::Infallible
        }
        ReturnType::Default | ReturnType::Type(..) => ConstructorOutcome::Fallible,
    }
}
