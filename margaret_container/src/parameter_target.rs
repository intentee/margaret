use syn::Path;
use syn::Type;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;

use crate::peel_target::peel_target;

pub(crate) enum ParameterTarget {
    Resolved {
        resolved: CanonicalPath,
        written: Path,
    },
    Unresolved {
        written: Path,
    },
    UnsupportedShape,
}

impl ParameterTarget {
    pub(crate) fn peel(index: &AttributeIndex, module_path: &[String], declared: &Type) -> Self {
        let Some(written) = peel_target(index, module_path, declared) else {
            return Self::UnsupportedShape;
        };

        match index.resolve_module_path(module_path, &written) {
            Some(resolved) => Self::Resolved { resolved, written },
            None => Self::Unresolved { written },
        }
    }

    pub(crate) fn resolves_to(&self, path: &CanonicalPath) -> bool {
        matches!(self, Self::Resolved { resolved, .. } if resolved == path)
    }
}
