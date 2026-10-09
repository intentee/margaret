use proc_macro2::TokenStream;
use quote::quote;
use syn::PathArguments;
use syn::Type;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_injection_codegen::optional_parameter::OptionalParameter;

use crate::constructor_parameter::ConstructorParameter;
use crate::input_weaving_error::InputWeavingError;
use crate::weaving_kind::WeavingKind;

fn has_generic_arguments(value_type: &Type) -> bool {
    let Type::Path(type_path) = value_type else {
        return false;
    };

    matches!(
        type_path
            .path
            .segments
            .last()
            .map(|segment| &segment.arguments),
        Some(PathArguments::AngleBracketed(_))
    )
}

#[derive(Clone, Debug, PartialEq)]
pub struct InputValue {
    pub required: bool,
    pub value_type: CanonicalPath,
    pub weaving: WeavingKind,
}

impl InputValue {
    /// # Errors
    ///
    /// Returns `InputWeavingError::GenericValueType` or
    /// `InputWeavingError::UnresolvableValueType`.
    pub fn from_declared(
        index: &AttributeIndex,
        item: &IndexedItem,
        declared: &Type,
        site: &ConstructorParameter,
    ) -> Result<Self, InputWeavingError> {
        let OptionalParameter {
            required,
            value_type,
        } = OptionalParameter::from_type(index, item, declared);

        if has_generic_arguments(&value_type) {
            return Err(InputWeavingError::GenericValueType {
                site: site.clone(),
                value_type: quote! { #value_type }.to_string(),
            });
        }

        let canonical = index.resolve_item_type(item, &value_type).ok_or_else(|| {
            InputWeavingError::UnresolvableValueType {
                site: site.clone(),
                value_type: quote! { #value_type }.to_string(),
            }
        })?;
        let weaving = WeavingKind::from_canonical(index, &canonical, required);

        Ok(Self {
            required,
            value_type: canonical,
            weaving,
        })
    }

    #[must_use]
    pub fn field_type(&self) -> TokenStream {
        let type_tokens = path_tokens(&self.value_type);

        if self.required {
            type_tokens
        } else {
            quote! { ::std::option::Option<#type_tokens> }
        }
    }

    #[must_use]
    pub fn parameter_referent(&self) -> TokenStream {
        match self.weaving {
            WeavingKind::BorrowedStr => quote! { str },
            WeavingKind::BorrowedPath => quote! { ::std::path::Path },
            WeavingKind::Copy | WeavingKind::Cloned => self.field_type(),
        }
    }
}

#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;
    use syn::Type;
    use syn::parse_quote;

    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_attributes_tests::indexed_source::IndexedSource;

    use crate::constructor_parameter::ConstructorParameter;
    use crate::input_weaving_error::InputWeavingError;
    use crate::weaving_kind::WeavingKind;

    use super::InputValue;

    fn site() -> ConstructorParameter {
        ConstructorParameter {
            owner: CanonicalPath::new(vec!["crate".to_string(), "Config".to_string()]),
            parameter: "value".to_string(),
        }
    }

    fn from_declared(declared: &Type) -> Result<InputValue, InputWeavingError> {
        let indexed = IndexedSource::new("#[singleton]\nstruct Config;\n");

        InputValue::from_declared(&indexed.index, indexed.item("Config"), declared, &site())
    }

    fn collapsed(tokens: &TokenStream) -> String {
        tokens.to_string().split_whitespace().collect()
    }

    #[test]
    fn resolves_a_required_value_to_its_canonical_type() {
        let value = from_declared(&parse_quote!(String)).expect("the value type resolves");

        assert!(value.required);
        assert_eq!(value.value_type.to_string(), "std::string::String");
        assert_eq!(value.weaving, WeavingKind::BorrowedStr);
    }

    #[test]
    fn peels_an_option_into_an_optional_value() {
        let value = from_declared(&parse_quote!(Option<String>)).expect("the value type resolves");

        assert!(!value.required);
        assert_eq!(value.value_type.to_string(), "std::string::String");
        assert_eq!(value.weaving, WeavingKind::Cloned);
    }

    #[test]
    fn rejects_a_generic_value_type() {
        let error = from_declared(&parse_quote!(Vec<String>)).expect_err("a generic is rejected");

        assert!(matches!(
            error,
            InputWeavingError::GenericValueType { ref value_type, .. }
                if value_type == "Vec < String >"
        ));
    }

    #[test]
    fn rejects_an_unresolvable_value_type() {
        let error = from_declared(&parse_quote!(Widget)).expect_err("an unknown name is rejected");

        assert!(matches!(
            error,
            InputWeavingError::UnresolvableValueType { ref value_type, .. }
                if value_type == "Widget"
        ));
    }

    #[test]
    fn rejects_a_value_type_that_is_not_a_path() {
        let error = from_declared(&parse_quote!((u8, u8))).expect_err("a tuple is rejected");

        assert!(matches!(
            error,
            InputWeavingError::UnresolvableValueType { ref value_type, .. }
                if value_type == "(u8 , u8)"
        ));
    }

    #[test]
    fn a_required_field_is_the_qualified_value_type() {
        let value = from_declared(&parse_quote!(std::path::PathBuf)).expect("the type resolves");

        assert_eq!(collapsed(&value.field_type()), "::std::path::PathBuf");
    }

    #[test]
    fn an_optional_field_is_an_option_of_the_qualified_value_type() {
        let value = from_declared(&parse_quote!(Option<u16>)).expect("the type resolves");

        assert_eq!(collapsed(&value.field_type()), "::std::option::Option<u16>");
    }

    #[test]
    fn a_borrowed_string_refers_to_str() {
        let value = from_declared(&parse_quote!(String)).expect("the type resolves");

        assert_eq!(collapsed(&value.parameter_referent()), "str");
    }

    #[test]
    fn a_borrowed_path_buf_refers_to_path() {
        let value = from_declared(&parse_quote!(std::path::PathBuf)).expect("the type resolves");

        assert_eq!(collapsed(&value.parameter_referent()), "::std::path::Path");
    }

    #[test]
    fn a_copy_value_refers_to_its_own_field_type() {
        let value = from_declared(&parse_quote!(u16)).expect("the type resolves");

        assert_eq!(value.weaving, WeavingKind::Copy);
        assert_eq!(collapsed(&value.parameter_referent()), "u16");
    }

    #[test]
    fn a_cloned_value_refers_to_its_own_field_type() {
        let value = from_declared(&parse_quote!(Option<String>)).expect("the type resolves");

        assert_eq!(
            collapsed(&value.parameter_referent()),
            "::std::option::Option<::std::string::String>"
        );
    }
}
