use syn::Expr;
use syn::Ident;

pub(crate) struct NamedAssignment<'expression> {
    pub(crate) name: &'expression Ident,
    pub(crate) value: &'expression Expr,
}

impl<'expression> NamedAssignment<'expression> {
    pub(crate) fn of(expression: &'expression Expr) -> Option<Self> {
        let Expr::Assign(assign) = expression else {
            return None;
        };
        let Expr::Path(left) = assign.left.as_ref() else {
            return None;
        };

        Some(Self {
            name: left.path.get_ident()?,
            value: assign.right.as_ref(),
        })
    }
}
