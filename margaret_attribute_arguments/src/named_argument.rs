use syn::Expr;

use crate::named_assignment::NamedAssignment;

#[derive(Clone, Debug)]
pub(crate) struct NamedArgument {
    pub(crate) name: String,
    pub(crate) value: Expr,
}

impl NamedArgument {
    pub(crate) fn from_expression(expression: &Expr) -> Option<Self> {
        NamedAssignment::of(expression).map(|NamedAssignment { name, value }| Self {
            name: name.to_string(),
            value: value.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use crate::named_argument::NamedArgument;

    #[test]
    fn rejects_an_assignment_to_a_non_path_target() {
        assert!(NamedArgument::from_expression(&parse_quote!(field.name = "value")).is_none());
    }

    #[test]
    fn rejects_an_assignment_to_a_multi_segment_path() {
        assert!(NamedArgument::from_expression(&parse_quote!(outer::inner = "value")).is_none());
    }
}
