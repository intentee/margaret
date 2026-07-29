use syn::Type;

use margaret_syn_type_peeling::option_inner::option_inner;

pub struct OptionalParameter {
    pub required: bool,
    pub value_type: Type,
}

impl OptionalParameter {
    #[must_use]
    pub fn from_type(declared: &Type) -> Self {
        match option_inner(declared) {
            Some(inner) => Self {
                required: false,
                value_type: inner.clone(),
            },
            None => Self {
                required: true,
                value_type: declared.clone(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use syn::Type;
    use syn::parse_quote;

    use super::OptionalParameter;

    fn classify(declared: Type) -> OptionalParameter {
        OptionalParameter::from_type(&declared)
    }

    #[test]
    fn a_plain_type_is_required() {
        assert!(classify(parse_quote!(String)).required);
    }

    #[test]
    fn an_option_type_is_not_required_and_unwraps_its_inner_type() {
        let classified = classify(parse_quote!(Option<String>));
        let value_type = &classified.value_type;

        assert!(!classified.required);
        assert_eq!(quote::quote!(#value_type).to_string(), "String");
    }

    #[test]
    fn a_non_path_type_is_required() {
        assert!(classify(parse_quote!((u8, u8))).required);
    }

    #[test]
    fn an_option_without_a_type_argument_is_required() {
        assert!(classify(parse_quote!(Option)).required);
    }

    #[test]
    fn an_option_of_a_non_type_argument_is_required() {
        assert!(classify(parse_quote!(Option<'static>)).required);
    }
}
