use margaret_attribute_arguments::attribute_args::AttributeArgs;

use crate::model_codegen_error::ModelCodegenError;
use crate::numeric_digits::NumericDigits;

pub(crate) struct ColumnArguments {
    pub(crate) name: Option<String>,
    pub(crate) numeric_digits: NumericDigits,
    pub(crate) primary_key: bool,
    pub(crate) unique: bool,
}

impl ColumnArguments {
    pub(crate) fn parse(
        arguments: &AttributeArgs,
        model: &str,
        field: &str,
    ) -> Result<Self, ModelCodegenError> {
        arguments.interpret(|reader| {
            let name = reader.take_string("name")?;
            let precision = reader.take_unsigned_integer("precision")?;
            let scale = reader.take_unsigned_integer("scale")?;
            let primary_key = reader.take_flag("primary_key");
            let unique = reader.take_flag("unique");

            Ok(Self {
                name,
                numeric_digits: NumericDigits::parse(precision, scale, model, field)?,
                primary_key,
                unique,
            })
        })
    }
}
