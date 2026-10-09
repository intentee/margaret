use margaret_model::check_predicate::CheckPredicate;
use margaret_model::column_type::ColumnType;
use margaret_schema_identifier_naming::byte_length_constraint_name::byte_length_constraint_name;
use margaret_schema_identifier_naming::minimum_constraint_name::minimum_constraint_name;

use crate::model_codegen_error::ModelCodegenError;
use crate::resolved_check::ResolvedCheck;

fn accepts_byte_length(column_type: ColumnType) -> bool {
    match column_type {
        ColumnType::Bytea => true,
        ColumnType::BigInt
        | ColumnType::Boolean
        | ColumnType::DoublePrecision
        | ColumnType::Integer
        | ColumnType::Numeric { .. }
        | ColumnType::Real
        | ColumnType::Text
        | ColumnType::Timestamptz
        | ColumnType::Uuid => false,
    }
}

fn accepts_minimum(column_type: ColumnType) -> bool {
    match column_type {
        ColumnType::BigInt
        | ColumnType::DoublePrecision
        | ColumnType::Integer
        | ColumnType::Numeric { .. }
        | ColumnType::Real => true,
        ColumnType::Boolean
        | ColumnType::Bytea
        | ColumnType::Text
        | ColumnType::Timestamptz
        | ColumnType::Uuid => false,
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum DeclaredColumnCheck {
    ByteLength { length: u32 },
    Minimum { minimum: u32 },
    NotDeclared,
}

impl DeclaredColumnCheck {
    pub(crate) fn parse(
        byte_length: Option<u32>,
        minimum: Option<u32>,
        model: &str,
        field: &str,
    ) -> Result<Self, ModelCodegenError> {
        match byte_length {
            None => Ok(match minimum {
                None => DeclaredColumnCheck::NotDeclared,
                Some(minimum) => DeclaredColumnCheck::Minimum { minimum },
            }),
            Some(_) if minimum.is_some() => Err(ModelCodegenError::ConflictingColumnChecks {
                field: field.to_string(),
                model: model.to_string(),
            }),
            Some(0) => Err(ModelCodegenError::ByteLengthMustBePositive {
                field: field.to_string(),
                model: model.to_string(),
            }),
            Some(length) => Ok(DeclaredColumnCheck::ByteLength { length }),
        }
    }

    pub(crate) fn is_declared(&self) -> bool {
        !matches!(self, DeclaredColumnCheck::NotDeclared)
    }

    pub(crate) fn resolve(
        &self,
        column_type: ColumnType,
        table: &str,
        column: &str,
        model: &str,
    ) -> Result<Vec<ResolvedCheck>, ModelCodegenError> {
        match self {
            DeclaredColumnCheck::NotDeclared => Ok(Vec::new()),
            DeclaredColumnCheck::ByteLength { length } => {
                if !accepts_byte_length(column_type) {
                    return Err(ModelCodegenError::ByteLengthOnNonByteaColumn {
                        column: column.to_string(),
                        model: model.to_string(),
                    });
                }

                Ok(vec![ResolvedCheck {
                    name: byte_length_constraint_name(table, column).map_err(|source| {
                        ModelCodegenError::ByteLengthConstraintNameTooLong {
                            model: model.to_string(),
                            source,
                        }
                    })?,
                    predicate: CheckPredicate::ByteLength { length: *length },
                }])
            }
            DeclaredColumnCheck::Minimum { minimum } => {
                if !accepts_minimum(column_type) {
                    return Err(ModelCodegenError::MinimumOnNonNumericColumn {
                        column: column.to_string(),
                        model: model.to_string(),
                    });
                }

                Ok(vec![ResolvedCheck {
                    name: minimum_constraint_name(table, column).map_err(|source| {
                        ModelCodegenError::MinimumConstraintNameTooLong {
                            model: model.to_string(),
                            source,
                        }
                    })?,
                    predicate: CheckPredicate::Minimum { minimum: *minimum },
                }])
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use margaret_model::check_predicate::CheckPredicate;
    use margaret_model::column_type::ColumnType;
    use margaret_schema_identifier_naming::max_identifier_bytes::MAX_IDENTIFIER_BYTES;

    use crate::declared_column_check::DeclaredColumnCheck;
    use crate::model_codegen_error::ModelCodegenError;
    use crate::resolved_check::ResolvedCheck;

    fn parse(
        byte_length: Option<u32>,
        minimum: Option<u32>,
    ) -> Result<DeclaredColumnCheck, ModelCodegenError> {
        DeclaredColumnCheck::parse(byte_length, minimum, "crate::Model", "value")
    }

    fn resolve(
        declared: &DeclaredColumnCheck,
        column_type: ColumnType,
    ) -> Result<Vec<ResolvedCheck>, ModelCodegenError> {
        declared.resolve(column_type, "fragment_metadata", "hash", "crate::Model")
    }

    #[test]
    fn reads_absent_arguments_as_not_declared() {
        assert_eq!(
            parse(None, None).expect("absent arguments parse"),
            DeclaredColumnCheck::NotDeclared
        );
    }

    #[test]
    fn reads_a_byte_length() {
        assert_eq!(
            parse(Some(32), None).expect("a byte length parses"),
            DeclaredColumnCheck::ByteLength { length: 32 }
        );
    }

    #[test]
    fn reads_a_minimum() {
        assert_eq!(
            parse(None, Some(0)).expect("a minimum parses"),
            DeclaredColumnCheck::Minimum { minimum: 0 }
        );
    }

    #[test]
    fn rejects_a_zero_byte_length() {
        assert!(matches!(
            parse(Some(0), None).expect_err("a zero byte length is rejected"),
            ModelCodegenError::ByteLengthMustBePositive { ref field, ref model }
                if field == "value" && model == "crate::Model"
        ));
    }

    #[test]
    fn rejects_a_byte_length_declared_together_with_a_minimum() {
        assert!(matches!(
            parse(Some(32), Some(0)).expect_err("two checks on one column are rejected"),
            ModelCodegenError::ConflictingColumnChecks { ref field, .. }
                if field == "value"
        ));
    }

    #[test]
    fn a_declared_check_reports_itself_as_declared() {
        assert!(
            parse(Some(32), None)
                .expect("a byte length parses")
                .is_declared()
        );
    }

    #[test]
    fn an_absent_check_reports_itself_as_not_declared() {
        assert!(
            !parse(None, None)
                .expect("absent arguments parse")
                .is_declared()
        );
    }

    #[test]
    fn resolves_no_checks_when_none_is_declared() {
        assert!(
            resolve(&DeclaredColumnCheck::NotDeclared, ColumnType::Bytea)
                .expect("an absent check resolves")
                .is_empty()
        );
    }

    #[test]
    fn resolves_a_byte_length_check_on_a_bytea_column() {
        assert_eq!(
            resolve(
                &DeclaredColumnCheck::ByteLength { length: 32 },
                ColumnType::Bytea
            )
            .expect("a byte length on a bytea column resolves"),
            vec![ResolvedCheck {
                name: "fragment_metadata_hash_byte_length".to_string(),
                predicate: CheckPredicate::ByteLength { length: 32 },
            }]
        );
    }

    #[test]
    fn rejects_a_byte_length_on_a_non_bytea_column() {
        assert!(matches!(
            resolve(
                &DeclaredColumnCheck::ByteLength { length: 32 },
                ColumnType::Text
            )
            .expect_err("a byte length on a text column is rejected"),
            ModelCodegenError::ByteLengthOnNonByteaColumn { ref column, .. }
                if column == "hash"
        ));
    }

    #[test]
    fn resolves_a_minimum_check_on_a_numeric_column() {
        assert_eq!(
            resolve(
                &DeclaredColumnCheck::Minimum { minimum: 0 },
                ColumnType::BigInt
            )
            .expect("a minimum on a big int column resolves"),
            vec![ResolvedCheck {
                name: "fragment_metadata_hash_minimum".to_string(),
                predicate: CheckPredicate::Minimum { minimum: 0 },
            }]
        );
    }

    #[test]
    fn rejects_a_minimum_on_a_non_numeric_column() {
        assert!(matches!(
            resolve(&DeclaredColumnCheck::Minimum { minimum: 0 }, ColumnType::Text)
                .expect_err("a minimum on a text column is rejected"),
            ModelCodegenError::MinimumOnNonNumericColumn { ref column, .. }
                if column == "hash"
        ));
    }

    #[test]
    fn rejects_a_byte_length_constraint_name_over_the_byte_limit() {
        let column = "a".repeat(MAX_IDENTIFIER_BYTES);

        assert!(matches!(
            DeclaredColumnCheck::ByteLength { length: 32 }
                .resolve(ColumnType::Bytea, "fragments", &column, "crate::Model")
                .expect_err("the derived constraint name exceeds the limit"),
            ModelCodegenError::ByteLengthConstraintNameTooLong { ref model, .. }
                if model == "crate::Model"
        ));
    }

    #[test]
    fn rejects_a_minimum_constraint_name_over_the_byte_limit() {
        let column = "a".repeat(MAX_IDENTIFIER_BYTES);

        assert!(matches!(
            DeclaredColumnCheck::Minimum { minimum: 0 }
                .resolve(ColumnType::BigInt, "fragments", &column, "crate::Model")
                .expect_err("the derived constraint name exceeds the limit"),
            ModelCodegenError::MinimumConstraintNameTooLong { ref model, .. }
                if model == "crate::Model"
        ));
    }
}
