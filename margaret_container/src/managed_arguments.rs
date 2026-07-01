use syn::Path;

use margaret_attributes::attribute_args::AttributeArgs;

use crate::container_error::ContainerError;

pub(crate) struct ManagedArguments {
    pub(crate) collection: Option<Path>,
    pub(crate) provides: Option<Path>,
}

impl ManagedArguments {
    pub(crate) fn parse(arguments: &AttributeArgs) -> Result<Self, ContainerError> {
        let collection = arguments.path("collection")?;
        let provides = arguments.path("provides")?;

        Ok(Self {
            collection,
            provides,
        })
    }
}
