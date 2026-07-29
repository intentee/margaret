use syn::Type;

use margaret_syn_type_peeling::single_generic_argument::single_generic_argument;

pub(crate) fn indirection_inner(ty: &Type) -> Option<&Type> {
    let Type::Path(type_path) = ty else {
        return None;
    };

    match type_path.path.segments.last() {
        Some(segment)
            if segment.ident == "Box" || segment.ident == "Rc" || segment.ident == "Arc" =>
        {
            single_generic_argument(segment)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::Type;

    use crate::indirection_inner::indirection_inner;

    fn inner(type_source: &str) -> Option<String> {
        let ty: Type = syn::parse_str(type_source).expect("the type fixture parses");

        indirection_inner(&ty).map(|inner| quote!(#inner).to_string())
    }

    #[test]
    fn peels_the_recognized_heap_pointers() {
        assert_eq!(inner("Box<Node>").as_deref(), Some("Node"));
        assert_eq!(inner("Rc<Node>").as_deref(), Some("Node"));
        assert_eq!(inner("Arc<Node>").as_deref(), Some("Node"));
    }

    #[test]
    fn peels_a_fully_qualified_pointer() {
        assert_eq!(inner("std::sync::Arc<Node>").as_deref(), Some("Node"));
    }

    #[test]
    fn ignores_a_non_pointer_path() {
        assert!(inner("Node").is_none());
    }

    #[test]
    fn ignores_a_non_path_type() {
        assert!(inner("[u8; 4]").is_none());
    }
}
