use syn::Attribute;
use syn::Meta;
use syn::Path;

use margaret_attribute_arguments::attribute_args::AttributeArgs;
use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;

use crate::attribute_error::AttributeError;
use crate::canonical_path::CanonicalPath;
use crate::framework_attribute::FrameworkAttribute;

enum IndexedAttributeArgs {
    Parsed(AttributeArgs),
    Rejected(AttributeArgumentsError),
}

pub struct IndexedAttribute {
    args: IndexedAttributeArgs,
    attribute: Attribute,
    framework_attribute: Option<FrameworkAttribute>,
}

impl IndexedAttribute {
    pub(crate) fn from_canonical(attribute: &Attribute, canonical_path: &CanonicalPath) -> Self {
        let args = match AttributeArgs::from_attribute(attribute) {
            Ok(args) => IndexedAttributeArgs::Parsed(args),
            Err(rejected) => IndexedAttributeArgs::Rejected(rejected),
        };

        let framework_attribute = FrameworkAttribute::from_canonical_path(canonical_path);

        Self {
            args,
            attribute: attribute.clone(),
            framework_attribute,
        }
    }

    #[must_use]
    pub fn new(attribute: &Attribute) -> Self {
        let canonical_path = CanonicalPath::new(
            attribute
                .path()
                .segments
                .iter()
                .map(|segment| segment.ident.to_string())
                .collect(),
        );

        Self::from_canonical(attribute, &canonical_path)
    }

    /// # Errors
    ///
    /// Returns `AttributeError::Arguments`.
    pub fn args(&self) -> Result<&AttributeArgs, AttributeError> {
        match &self.args {
            IndexedAttributeArgs::Parsed(args) => Ok(args),
            IndexedAttributeArgs::Rejected(rejected) => Err(rejected.clone().into()),
        }
    }

    #[must_use]
    pub fn framework_attribute(&self) -> Option<FrameworkAttribute> {
        self.framework_attribute
    }

    #[must_use]
    pub fn is_bare(&self) -> bool {
        matches!(self.attribute.meta, Meta::Path(_))
    }

    #[must_use]
    pub fn path(&self) -> &Path {
        self.attribute.path()
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::IndexedAttribute;

    #[test]
    fn preserves_a_malformed_argument_error_until_the_attribute_is_consumed() {
        let attribute = IndexedAttribute::new(&parse_quote!(#[bad_args(= 5)]));
        let message = attribute
            .args()
            .expect_err("the malformed arguments must be rejected")
            .to_string();

        assert!(message.contains("expected an expression"));
    }

    #[test]
    fn preserves_a_duplicate_argument_error_until_the_attribute_is_consumed() {
        let attribute = IndexedAttribute::new(&parse_quote!(#[tag(value = 1, value = 2)]));
        let message = attribute
            .args()
            .expect_err("the duplicate must be rejected")
            .to_string();

        assert!(message.contains("provided more than once"));
    }

    #[test]
    fn distinguishes_bare_attributes_from_argument_lists() {
        let bare = IndexedAttribute::new(&parse_quote!(#[tag]));
        let listed = IndexedAttribute::new(&parse_quote!(#[tag()]));

        assert!(bare.is_bare());
        assert!(!listed.is_bare());
    }
}
