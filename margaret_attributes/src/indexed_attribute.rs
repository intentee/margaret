use syn::Attribute;
use syn::Meta;
use syn::Path;

use crate::attribute_args::AttributeArgs;
use crate::attribute_args_parse_error::AttributeArgsParseError;
use crate::attribute_error::AttributeError;
use crate::canonical_path::CanonicalPath;
use crate::framework_attribute::FrameworkAttribute;

enum IndexedAttributeArgs {
    DuplicateNamedArgument {
        attribute_path: String,
        key: String,
    },
    Malformed {
        attribute_path: String,
        message: String,
    },
    Parsed(AttributeArgs),
}

pub struct IndexedAttribute {
    args: IndexedAttributeArgs,
    attribute: Attribute,
    framework_attribute: Option<FrameworkAttribute>,
}

impl IndexedAttribute {
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

    pub(crate) fn from_canonical(attribute: &Attribute, canonical_path: &CanonicalPath) -> Self {
        let args = match AttributeArgs::from_attribute(attribute) {
            Ok(args) => IndexedAttributeArgs::Parsed(args),
            Err(AttributeArgsParseError::Malformed {
                attribute_path,
                source,
            }) => IndexedAttributeArgs::Malformed {
                attribute_path,
                message: source.to_string(),
            },
            Err(AttributeArgsParseError::DuplicateNamedArgument {
                attribute_path,
                key,
            }) => IndexedAttributeArgs::DuplicateNamedArgument {
                attribute_path,
                key,
            },
        };

        let framework_attribute = FrameworkAttribute::from_canonical_path(canonical_path);

        Self {
            args,
            attribute: attribute.clone(),
            framework_attribute,
        }
    }

    #[must_use]
    pub fn path(&self) -> &Path {
        self.attribute.path()
    }

    #[must_use]
    pub fn framework_attribute(&self) -> Option<FrameworkAttribute> {
        self.framework_attribute
    }

    /// # Errors
    ///
    /// Returns `AttributeError::DuplicateNamedArgument` or `AttributeError::AttributeArguments`.
    pub fn args(&self) -> Result<&AttributeArgs, AttributeError> {
        match &self.args {
            IndexedAttributeArgs::DuplicateNamedArgument {
                attribute_path,
                key,
            } => Err(AttributeError::DuplicateNamedArgument {
                attribute_path: attribute_path.clone(),
                key: key.clone(),
            }),
            IndexedAttributeArgs::Malformed {
                attribute_path,
                message,
            } => Err(AttributeError::AttributeArguments {
                attribute_path: attribute_path.clone(),
                source: syn::Error::new(proc_macro2::Span::call_site(), message),
            }),
            IndexedAttributeArgs::Parsed(args) => Ok(args),
        }
    }

    #[must_use]
    pub fn is_bare(&self) -> bool {
        matches!(self.attribute.meta, Meta::Path(_))
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
