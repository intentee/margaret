use std::collections::HashMap;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::struct_shape::StructShape;

use crate::model_codegen_error::ModelCodegenError;

pub(crate) struct EnumVariants {
    validated: HashMap<CanonicalPath, Vec<String>>,
}

impl EnumVariants {
    pub(crate) fn new() -> Self {
        Self {
            validated: HashMap::new(),
        }
    }

    pub(crate) fn of(
        &mut self,
        enum_item: &IndexedItem,
        model: &str,
        field: &str,
    ) -> Result<Vec<String>, ModelCodegenError> {
        if let Some(variants) = self.validated.get(enum_item.canonical_path()) {
            return Ok(variants.clone());
        }

        if enum_item.variants().is_empty() {
            return Err(ModelCodegenError::EmptyEnumColumn {
                enum_type: enum_item.canonical_path().to_string(),
                field: field.to_string(),
                model: model.to_string(),
            });
        }

        let mut variants = Vec::with_capacity(enum_item.variants().len());

        for variant in enum_item.variants() {
            if !matches!(variant.shape(), StructShape::Unit) {
                return Err(ModelCodegenError::EnumColumnVariantNotUnit {
                    enum_type: enum_item.canonical_path().to_string(),
                    field: field.to_string(),
                    model: model.to_string(),
                    variant: variant.identifier().to_string(),
                });
            }

            variants.push(variant.identifier().to_string());
        }

        self.validated
            .insert(enum_item.canonical_path().clone(), variants.clone());

        Ok(variants)
    }
}
