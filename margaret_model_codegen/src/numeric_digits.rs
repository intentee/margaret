use crate::model_codegen_error::ModelCodegenError;

const MAX_NUMERIC_PRECISION: u32 = 28;

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum NumericDigits {
    Declared { precision: u32, scale: u32 },
    NotDeclared,
}

impl NumericDigits {
    pub(crate) fn parse(
        precision: Option<u32>,
        scale: Option<u32>,
        model: &str,
        field: &str,
    ) -> Result<Self, ModelCodegenError> {
        let Some(precision) = precision else {
            return match scale {
                None => Ok(NumericDigits::NotDeclared),
                Some(_) => Err(ModelCodegenError::NumericPrecisionMissing {
                    field: field.to_string(),
                    model: model.to_string(),
                }),
            };
        };
        let Some(scale) = scale else {
            return Err(ModelCodegenError::NumericScaleMissing {
                field: field.to_string(),
                model: model.to_string(),
            });
        };

        if precision == 0 || precision > MAX_NUMERIC_PRECISION {
            return Err(ModelCodegenError::NumericPrecisionOutOfRange {
                field: field.to_string(),
                model: model.to_string(),
                precision,
            });
        }

        if scale > precision {
            return Err(ModelCodegenError::NumericScaleExceedsPrecision {
                field: field.to_string(),
                model: model.to_string(),
                precision,
                scale,
            });
        }

        Ok(NumericDigits::Declared { precision, scale })
    }
}

#[cfg(test)]
mod tests {
    use crate::model_codegen_error::ModelCodegenError;
    use crate::numeric_digits::MAX_NUMERIC_PRECISION;
    use crate::numeric_digits::NumericDigits;

    fn parse(
        precision: Option<u32>,
        scale: Option<u32>,
    ) -> Result<NumericDigits, ModelCodegenError> {
        NumericDigits::parse(precision, scale, "crate::Model", "value")
    }

    #[test]
    fn reads_absent_arguments_as_not_declared() {
        assert_eq!(
            parse(None, None).expect("absent arguments parse"),
            NumericDigits::NotDeclared
        );
    }

    #[test]
    fn reads_a_precision_and_scale_pair() {
        assert_eq!(
            parse(Some(12), Some(2)).expect("a digit pair parses"),
            NumericDigits::Declared {
                precision: 12,
                scale: 2
            }
        );
    }

    #[test]
    fn accepts_the_largest_representable_precision() {
        assert_eq!(
            parse(Some(MAX_NUMERIC_PRECISION), Some(MAX_NUMERIC_PRECISION))
                .expect("the largest representable precision parses"),
            NumericDigits::Declared {
                precision: MAX_NUMERIC_PRECISION,
                scale: MAX_NUMERIC_PRECISION
            }
        );
    }

    #[test]
    fn rejects_a_scale_without_a_precision() {
        assert!(matches!(
            parse(None, Some(2)).expect_err("a lone scale is rejected"),
            ModelCodegenError::NumericPrecisionMissing { ref field, ref model }
                if field == "value" && model == "crate::Model"
        ));
    }

    #[test]
    fn rejects_a_precision_without_a_scale() {
        assert!(matches!(
            parse(Some(12), None).expect_err("a lone precision is rejected"),
            ModelCodegenError::NumericScaleMissing { ref field, ref model }
                if field == "value" && model == "crate::Model"
        ));
    }

    #[test]
    fn rejects_a_zero_precision() {
        assert!(matches!(
            parse(Some(0), Some(0)).expect_err("a zero precision is rejected"),
            ModelCodegenError::NumericPrecisionOutOfRange { precision, .. }
                if precision == 0
        ));
    }

    #[test]
    fn rejects_a_precision_beyond_what_a_decimal_can_hold() {
        assert!(matches!(
            parse(Some(MAX_NUMERIC_PRECISION + 1), Some(0))
                .expect_err("an unrepresentable precision is rejected"),
            ModelCodegenError::NumericPrecisionOutOfRange { precision, .. }
                if precision == MAX_NUMERIC_PRECISION + 1
        ));
    }

    #[test]
    fn rejects_a_scale_greater_than_the_precision() {
        assert!(matches!(
            parse(Some(4), Some(5)).expect_err("an oversized scale is rejected"),
            ModelCodegenError::NumericScaleExceedsPrecision { precision, scale, .. }
                if precision == 4 && scale == 5
        ));
    }
}
