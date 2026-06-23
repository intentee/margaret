use syn::Path;

use crate::error::AttributeError;
use crate::path_text::format_path;

pub struct AttributeSelector {
    path: Path,
}

impl AttributeSelector {
    pub fn parse(input: &str) -> Result<Self, AttributeError> {
        match syn::parse_str::<Path>(input) {
            Ok(path) => Ok(Self { path }),
            Err(source) => Err(AttributeError::InvalidSelector {
                input: input.to_string(),
                source,
            }),
        }
    }

    pub(crate) fn matches(&self, attribute_path: &Path) -> bool {
        let selector_length = self.path.segments.len();
        let attribute_length = attribute_path.segments.len();

        if selector_length > attribute_length {
            return false;
        }

        let offset = attribute_length - selector_length;

        self.path
            .segments
            .iter()
            .zip(attribute_path.segments.iter().skip(offset))
            .all(|(selector_segment, attribute_segment)| {
                selector_segment.ident == attribute_segment.ident
            })
    }

    pub(crate) fn display_path(&self) -> String {
        format_path(&self.path)
    }
}
