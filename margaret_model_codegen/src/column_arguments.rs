use syn::Path;

use margaret_attribute_arguments::attribute_args::AttributeArgs;
use margaret_item_naming_argument::item_naming_argument::ItemNamingArgument;

use crate::declared_column_check::DeclaredColumnCheck;
use crate::model_codegen_error::ModelCodegenError;
use crate::numeric_digits::NumericDigits;

pub(crate) struct ColumnArguments {
    pub(crate) check: DeclaredColumnCheck,
    pub(crate) default: Option<Path>,
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
            let byte_length = reader.take_unsigned_integer("byte_length")?;
            let default = reader.take_path(ItemNamingArgument::ColumnDefault.key())?;
            let minimum = reader.take_unsigned_integer("minimum")?;
            let name = reader.take_string("name")?;
            let precision = reader.take_unsigned_integer("precision")?;
            let scale = reader.take_unsigned_integer("scale")?;
            let primary_key = reader.take_flag("primary_key");
            let unique = reader.take_flag("unique");

            Ok(Self {
                check: DeclaredColumnCheck::parse(byte_length, minimum, model, field)?,
                default,
                name,
                numeric_digits: NumericDigits::parse(precision, scale, model, field)?,
                primary_key,
                unique,
            })
        })
    }
}
