use syn::GenericArgument;
use syn::PathArguments;
use syn::Type;

#[must_use]
pub fn anyhow_result_ok_type(declared: &Type) -> Option<&Type> {
    let Type::Path(type_path) = declared else {
        return None;
    };
    if type_path.qself.is_some() {
        return None;
    }
    let mut segments = type_path.path.segments.iter();
    let (Some(anyhow), Some(result), None) = (segments.next(), segments.next(), segments.next())
    else {
        return None;
    };

    if anyhow.ident != "anyhow"
        || !matches!(anyhow.arguments, PathArguments::None)
        || result.ident != "Result"
    {
        return None;
    }

    let PathArguments::AngleBracketed(arguments) = &result.arguments else {
        return None;
    };

    if arguments.args.len() != 1 {
        return None;
    }

    match arguments.args.first() {
        Some(GenericArgument::Type(ok_type)) => Some(ok_type),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use quote::ToTokens;
    use syn::Type;
    use syn::parse_quote;

    use super::anyhow_result_ok_type;

    #[test]
    fn returns_the_ok_type_from_an_anyhow_result() {
        let declared: Type = parse_quote!(anyhow::Result<Vec<u8>>);

        let ok_type = anyhow_result_ok_type(&declared).map(ToTokens::to_token_stream);

        assert_eq!(
            ok_type.map(|tokens| tokens.to_string()),
            Some("Vec < u8 >".to_string())
        );
    }

    #[test]
    fn accepts_an_absolute_anyhow_path() {
        let declared: Type = parse_quote!(::anyhow::Result<()>);

        assert!(anyhow_result_ok_type(&declared).is_some());
    }

    #[test]
    fn rejects_types_that_are_not_paths() {
        let declared: Type = parse_quote!((u8, u8));

        assert!(anyhow_result_ok_type(&declared).is_none());
    }

    #[test]
    fn rejects_qualified_paths() {
        let declared: Type = parse_quote!(<Provider as anyhow>::Result<u8>);

        assert!(anyhow_result_ok_type(&declared).is_none());
    }

    #[test]
    fn rejects_paths_with_the_wrong_number_of_segments() {
        for declared in [
            parse_quote!(Result<u8>),
            parse_quote!(crate::anyhow::Result<u8>),
        ] {
            assert!(anyhow_result_ok_type(&declared).is_none());
        }
    }

    #[test]
    fn rejects_paths_other_than_anyhow_result() {
        for declared in [
            parse_quote!(other::Result<u8>),
            parse_quote!(anyhow::Outcome<u8>),
        ] {
            assert!(anyhow_result_ok_type(&declared).is_none());
        }
    }

    #[test]
    fn rejects_result_without_angle_bracketed_arguments() {
        let declared: Type = parse_quote!(anyhow::Result);

        assert!(anyhow_result_ok_type(&declared).is_none());
    }

    #[test]
    fn rejects_arguments_on_the_anyhow_path_segment() {
        let declared: Type = parse_quote!(anyhow::<u8>::Result<u8>);

        assert!(anyhow_result_ok_type(&declared).is_none());
    }

    #[test]
    fn rejects_result_with_more_than_one_argument() {
        let declared: Type = parse_quote!(anyhow::Result<u8, Error>);

        assert!(anyhow_result_ok_type(&declared).is_none());
    }

    #[test]
    fn rejects_a_non_type_result_argument() {
        let declared: Type = parse_quote!(anyhow::Result<'static>);

        assert!(anyhow_result_ok_type(&declared).is_none());
    }
}
