use proc_macro2::TokenStream;
use quote::ToTokens;
use quote::quote;
use syn::GenericArgument;
use syn::PathArguments;
use syn::PathSegment;
use syn::Type;

use crate::inferred_column::InferredColumn;
use crate::model_codegen_error::ModelCodegenError;

fn single_generic_argument(segment: &PathSegment) -> Option<&Type> {
    let PathArguments::AngleBracketed(arguments) = &segment.arguments else {
        return None;
    };

    let mut arguments = arguments.args.iter();

    match (arguments.next(), arguments.next()) {
        (Some(GenericArgument::Type(generic_type)), None) => Some(generic_type),
        _ => None,
    }
}

fn option_inner(ty: &Type) -> Option<&Type> {
    let Type::Path(type_path) = ty else {
        return None;
    };

    match type_path.path.segments.last() {
        Some(segment) if segment.ident == "Option" => single_generic_argument(segment),
        _ => None,
    }
}

fn base_column_type(ty: &Type) -> Option<(TokenStream, TokenStream)> {
    let Type::Path(type_path) = ty else {
        return None;
    };

    match type_path.path.segments.last() {
        Some(segment) if segment.ident == "Uuid" => Some((
            quote!(margaret_model::column_type::ColumnType::Uuid),
            quote!(margaret_model::column_default::ColumnDefault::UuidV7),
        )),
        Some(segment) if segment.ident == "String" => Some((
            quote!(margaret_model::column_type::ColumnType::Text),
            quote!(margaret_model::column_default::ColumnDefault::NotSet),
        )),
        Some(segment) if segment.ident == "bool" => Some((
            quote!(margaret_model::column_type::ColumnType::Boolean),
            quote!(margaret_model::column_default::ColumnDefault::NotSet),
        )),
        Some(segment) if segment.ident == "i32" => Some((
            quote!(margaret_model::column_type::ColumnType::Integer),
            quote!(margaret_model::column_default::ColumnDefault::NotSet),
        )),
        Some(segment) if segment.ident == "i64" => Some((
            quote!(margaret_model::column_type::ColumnType::BigInt),
            quote!(margaret_model::column_default::ColumnDefault::NotSet),
        )),
        _ => None,
    }
}

pub(crate) fn infer_column_type(
    ty: &Type,
    model: &str,
    column: &str,
) -> Result<InferredColumn, ModelCodegenError> {
    let (base, nullable) = match option_inner(ty) {
        Some(inner) => (inner, true),
        None => (ty, false),
    };

    match base_column_type(base) {
        Some((column_type, default)) => Ok(InferredColumn {
            column_type,
            default,
            nullable,
        }),
        None => Err(ModelCodegenError::UninferrableColumnType {
            column: column.to_string(),
            model: model.to_string(),
            rust_type: ty.to_token_stream().to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::Type;

    use crate::infer_column_type::infer_column_type;
    use crate::inferred_column::InferredColumn;
    use crate::model_codegen_error::ModelCodegenError;

    fn infer(type_source: &str) -> Result<InferredColumn, ModelCodegenError> {
        let ty: Type = syn::parse_str(type_source).expect("the type fixture parses");

        infer_column_type(&ty, "crate::Model", "value")
    }

    fn column_type(type_source: &str) -> String {
        infer(type_source)
            .expect("the type is inferable")
            .column_type
            .to_string()
    }

    #[test]
    fn infers_a_uuid_column_with_a_v7_default() {
        let inferred = infer("uuid::Uuid").expect("a uuid is inferable");

        assert_eq!(
            inferred.column_type.to_string(),
            quote!(margaret_model::column_type::ColumnType::Uuid).to_string()
        );
        assert_eq!(
            inferred.default.to_string(),
            quote!(margaret_model::column_default::ColumnDefault::UuidV7).to_string()
        );
        assert!(!inferred.nullable);
    }

    #[test]
    fn infers_text_from_string() {
        assert_eq!(
            column_type("String"),
            quote!(margaret_model::column_type::ColumnType::Text).to_string()
        );
    }

    #[test]
    fn infers_boolean_from_bool() {
        assert_eq!(
            column_type("bool"),
            quote!(margaret_model::column_type::ColumnType::Boolean).to_string()
        );
    }

    #[test]
    fn infers_integer_from_i32() {
        assert_eq!(
            column_type("i32"),
            quote!(margaret_model::column_type::ColumnType::Integer).to_string()
        );
    }

    #[test]
    fn infers_big_int_from_i64() {
        assert_eq!(
            column_type("i64"),
            quote!(margaret_model::column_type::ColumnType::BigInt).to_string()
        );
    }

    #[test]
    fn treats_an_option_as_a_nullable_column() {
        let inferred = infer("Option<String>").expect("an optional string is inferable");

        assert!(inferred.nullable);
        assert_eq!(
            inferred.column_type.to_string(),
            quote!(margaret_model::column_type::ColumnType::Text).to_string()
        );
        assert_eq!(
            inferred.default.to_string(),
            quote!(margaret_model::column_default::ColumnDefault::NotSet).to_string()
        );
    }

    #[test]
    fn rejects_an_unsupported_type() {
        assert!(infer("u64").is_err());
    }

    #[test]
    fn rejects_a_non_path_type() {
        assert!(infer("[u8; 4]").is_err());
    }

    #[test]
    fn rejects_a_bare_option() {
        assert!(infer("Option").is_err());
    }

    #[test]
    fn rejects_an_option_with_multiple_arguments() {
        assert!(infer("Option<i32, i64>").is_err());
    }
}
