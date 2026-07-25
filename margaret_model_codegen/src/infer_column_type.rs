use quote::ToTokens;
use quote::quote;
use syn::Type;

use crate::inferred_column::InferredColumn;
use crate::model_codegen_error::ModelCodegenError;
use crate::option_inner::option_inner;
use crate::single_generic_argument::single_generic_argument;

fn is_u8(ty: &Type) -> bool {
    let Type::Path(type_path) = ty else {
        return false;
    };

    matches!(type_path.path.segments.last(), Some(segment) if segment.ident == "u8")
}

fn base_column_type(ty: &Type) -> Option<InferredColumn> {
    let Type::Path(type_path) = ty else {
        return None;
    };

    match type_path.path.segments.last() {
        Some(segment) if segment.ident == "Uuid" => Some(InferredColumn {
            column_type: quote!(margaret::framework::model::column_type::ColumnType::Uuid),
            default: quote!(margaret::framework::model::column_default::ColumnDefault::UuidV7),
            nullable: false,
        }),
        Some(segment) if segment.ident == "String" => Some(InferredColumn {
            column_type: quote!(margaret::framework::model::column_type::ColumnType::Text),
            default: quote!(margaret::framework::model::column_default::ColumnDefault::NotSet),
            nullable: false,
        }),
        Some(segment) if segment.ident == "bool" => Some(InferredColumn {
            column_type: quote!(margaret::framework::model::column_type::ColumnType::Boolean),
            default: quote!(margaret::framework::model::column_default::ColumnDefault::NotSet),
            nullable: false,
        }),
        Some(segment) if segment.ident == "i32" => Some(InferredColumn {
            column_type: quote!(margaret::framework::model::column_type::ColumnType::Integer),
            default: quote!(margaret::framework::model::column_default::ColumnDefault::NotSet),
            nullable: false,
        }),
        Some(segment) if segment.ident == "i64" => Some(InferredColumn {
            column_type: quote!(margaret::framework::model::column_type::ColumnType::BigInt),
            default: quote!(margaret::framework::model::column_default::ColumnDefault::NotSet),
            nullable: false,
        }),
        Some(segment) if segment.ident == "DateTime" => Some(InferredColumn {
            column_type: quote!(margaret::framework::model::column_type::ColumnType::Timestamptz),
            default: quote!(margaret::framework::model::column_default::ColumnDefault::NotSet),
            nullable: false,
        }),
        Some(segment)
            if segment.ident == "Vec" && single_generic_argument(segment).is_some_and(is_u8) =>
        {
            Some(InferredColumn {
                column_type: quote!(margaret::framework::model::column_type::ColumnType::Bytea),
                default: quote!(margaret::framework::model::column_default::ColumnDefault::NotSet),
                nullable: false,
            })
        }
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
        Some(inferred) => Ok(InferredColumn {
            nullable,
            ..inferred
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
            quote!(margaret::framework::model::column_type::ColumnType::Uuid).to_string()
        );
        assert_eq!(
            inferred.default.to_string(),
            quote!(margaret::framework::model::column_default::ColumnDefault::UuidV7).to_string()
        );
        assert!(!inferred.nullable);
    }

    #[test]
    fn infers_text_from_string() {
        assert_eq!(
            column_type("String"),
            quote!(margaret::framework::model::column_type::ColumnType::Text).to_string()
        );
    }

    #[test]
    fn infers_boolean_from_bool() {
        assert_eq!(
            column_type("bool"),
            quote!(margaret::framework::model::column_type::ColumnType::Boolean).to_string()
        );
    }

    #[test]
    fn infers_integer_from_i32() {
        assert_eq!(
            column_type("i32"),
            quote!(margaret::framework::model::column_type::ColumnType::Integer).to_string()
        );
    }

    #[test]
    fn infers_big_int_from_i64() {
        assert_eq!(
            column_type("i64"),
            quote!(margaret::framework::model::column_type::ColumnType::BigInt).to_string()
        );
    }

    #[test]
    fn infers_timestamptz_from_datetime() {
        let inferred = infer("chrono::DateTime<chrono::Utc>").expect("a datetime is inferable");

        assert_eq!(
            inferred.column_type.to_string(),
            quote!(margaret::framework::model::column_type::ColumnType::Timestamptz).to_string()
        );
        assert_eq!(
            inferred.default.to_string(),
            quote!(margaret::framework::model::column_default::ColumnDefault::NotSet).to_string()
        );
        assert!(!inferred.nullable);
    }

    #[test]
    fn treats_an_option_as_a_nullable_column() {
        let inferred = infer("Option<String>").expect("an optional string is inferable");

        assert!(inferred.nullable);
        assert_eq!(
            inferred.column_type.to_string(),
            quote!(margaret::framework::model::column_type::ColumnType::Text).to_string()
        );
        assert_eq!(
            inferred.default.to_string(),
            quote!(margaret::framework::model::column_default::ColumnDefault::NotSet).to_string()
        );
    }

    #[test]
    fn infers_bytea_from_a_byte_vector() {
        let inferred = infer("Vec<u8>").expect("a byte vector is inferable");

        assert_eq!(
            inferred.column_type.to_string(),
            quote!(margaret::framework::model::column_type::ColumnType::Bytea).to_string()
        );
        assert_eq!(
            inferred.default.to_string(),
            quote!(margaret::framework::model::column_default::ColumnDefault::NotSet).to_string()
        );
        assert!(!inferred.nullable);
    }

    #[test]
    fn treats_an_optional_byte_vector_as_a_nullable_bytea_column() {
        let inferred = infer("Option<Vec<u8>>").expect("an optional byte vector is inferable");

        assert!(inferred.nullable);
        assert_eq!(
            inferred.column_type.to_string(),
            quote!(margaret::framework::model::column_type::ColumnType::Bytea).to_string()
        );
    }

    #[test]
    fn rejects_a_vector_of_non_bytes() {
        assert!(infer("Vec<i32>").is_err());
    }

    #[test]
    fn rejects_a_vector_of_a_non_path_element() {
        assert!(infer("Vec<[u8; 4]>").is_err());
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
