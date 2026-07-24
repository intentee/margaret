use syn::Expr;

#[derive(Clone)]
pub(crate) struct NamedArgument {
    pub(crate) name: String,
    pub(crate) value: Expr,
}

impl NamedArgument {
    pub(crate) fn from_expression(expression: &Expr) -> Option<Self> {
        let Expr::Assign(assign) = expression else {
            return None;
        };
        let Expr::Path(left) = assign.left.as_ref() else {
            return None;
        };
        let name = left.path.get_ident()?;

        Some(Self {
            name: name.to_string(),
            value: assign.right.as_ref().clone(),
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
