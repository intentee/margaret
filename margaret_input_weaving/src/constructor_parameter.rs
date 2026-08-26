use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FormatResult;

use margaret_attributes::canonical_path::CanonicalPath;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstructorParameter {
    pub owner: CanonicalPath,
    pub parameter: String,
}

impl Display for ConstructorParameter {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FormatResult {
        write!(
            formatter,
            "parameter '{}' of '{}'",
            self.parameter, self.owner
        )
    }
}

#[cfg(test)]
mod tests {
    use margaret_attributes::canonical_path::CanonicalPath;

    use super::ConstructorParameter;

    #[test]
    fn describes_the_parameter_and_its_owner() {
        let site = ConstructorParameter {
            owner: CanonicalPath::new(vec!["crate".to_string(), "Config".to_string()]),
            parameter: "label".to_string(),
        };

        assert_eq!(site.to_string(), "parameter 'label' of 'crate::Config'");
    }
}
