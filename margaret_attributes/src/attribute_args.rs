use syn::Attribute;
use syn::Expr;
use syn::Lit;
use syn::Meta;
use syn::Path;
use syn::Token;
use syn::punctuated::Punctuated;

use crate::attribute_error::AttributeError;
use crate::format_path::format_path;

struct NamedArgument {
    name: String,
    value: Expr,
}

pub struct AttributeArgs {
    attribute_path: String,
    named: Vec<NamedArgument>,
    positional: Vec<Expr>,
}

impl AttributeArgs {
    pub fn from_attribute(attribute: &Attribute) -> Result<Self, AttributeError> {
        let attribute_path = format_path(attribute.path());

        match &attribute.meta {
            Meta::Path(_) => Ok(Self {
                attribute_path,
                named: Vec::new(),
                positional: Vec::new(),
            }),
            Meta::NameValue(meta_name_value) => Ok(Self {
                named: vec![NamedArgument {
                    name: format_path(&meta_name_value.path),
                    value: meta_name_value.value.clone(),
                }],
                attribute_path,
                positional: Vec::new(),
            }),
            Meta::List(meta_list) => {
                let parser = Punctuated::<Expr, Token![,]>::parse_terminated;
                let expressions = match meta_list.parse_args_with(parser) {
                    Ok(expressions) => expressions,
                    Err(source) => {
                        return Err(AttributeError::AttributeArguments {
                            attribute_path,
                            source,
                        });
                    }
                };

                let mut named = Vec::new();
                let mut positional = Vec::new();

                for expression in expressions {
                    match named_argument(&expression) {
                        Some(found) => named.push(found),
                        None => positional.push(expression),
                    }
                }

                Ok(Self {
                    attribute_path,
                    named,
                    positional,
                })
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.named.is_empty() && self.positional.is_empty()
    }

    pub fn named(&self, key: &str) -> Option<&Expr> {
        self.named
            .iter()
            .find(|argument| argument.name == key)
            .map(|argument| &argument.value)
    }

    pub fn positional(&self, index: usize) -> Option<&Expr> {
        self.positional.get(index)
    }

    pub fn string(&self, key: &str) -> Result<Option<String>, AttributeError> {
        match self.named(key) {
            None => Ok(None),
            Some(Expr::Lit(expression)) => match &expression.lit {
                Lit::Str(literal) => Ok(Some(literal.value())),
                _ => Err(self.unexpected_argument(key, "string literal")),
            },
            Some(_) => Err(self.unexpected_argument(key, "string literal")),
        }
    }

    pub fn path(&self, key: &str) -> Result<Option<Path>, AttributeError> {
        match self.named(key) {
            None => Ok(None),
            Some(Expr::Path(expression)) => Ok(Some(expression.path.clone())),
            Some(_) => Err(self.unexpected_argument(key, "path")),
        }
    }

    pub fn boolean(&self, key: &str) -> Result<Option<bool>, AttributeError> {
        match self.named(key) {
            None => Ok(None),
            Some(Expr::Lit(expression)) => match &expression.lit {
                Lit::Bool(literal) => Ok(Some(literal.value)),
                _ => Err(self.unexpected_argument(key, "boolean literal")),
            },
            Some(_) => Err(self.unexpected_argument(key, "boolean literal")),
        }
    }

    fn unexpected_argument(&self, key: &str, expected: &str) -> AttributeError {
        AttributeError::UnexpectedArgument {
            attribute_path: self.attribute_path.clone(),
            key: key.to_string(),
            expected: expected.to_string(),
        }
    }
}

fn named_argument(expression: &Expr) -> Option<NamedArgument> {
    let Expr::Assign(assign) = expression else {
        return None;
    };
    let Expr::Path(left) = assign.left.as_ref() else {
        return None;
    };
    let name = left.path.get_ident()?;

    Some(NamedArgument {
        name: name.to_string(),
        value: assign.right.as_ref().clone(),
    })
}

#[cfg(test)]
mod tests {
    use syn::Attribute;
    use syn::parse_quote;

    use crate::attribute_args::AttributeArgs;

    fn args(attribute: Attribute) -> AttributeArgs {
        AttributeArgs::from_attribute(&attribute).expect("the arguments parse")
    }

    #[test]
    fn a_bare_attribute_has_no_arguments() {
        assert!(args(parse_quote!(#[singleton])).is_empty());
    }

    #[test]
    fn a_name_value_attribute_exposes_its_named_argument() {
        assert!(args(parse_quote!(#[doc = "text"])).named("doc").is_some());
    }

    #[test]
    fn a_list_attribute_splits_named_and_positional_arguments() {
        let parsed = args(parse_quote!(#[route(method = Get, "/home")]));

        assert!(!parsed.is_empty());
        assert!(parsed.named("method").is_some());
        assert!(parsed.named("absent").is_none());
        assert!(parsed.positional(0).is_some());
        assert!(parsed.positional(99).is_none());
    }

    #[test]
    fn non_identifier_assignments_are_treated_as_positional() {
        let parsed = args(parse_quote!(#[route(0 = 1, outer::inner = 2)]));

        assert!(parsed.named("outer").is_none());
        assert!(parsed.positional(0).is_some());
        assert!(parsed.positional(1).is_some());
    }

    #[test]
    fn reads_a_string_argument_and_rejects_non_strings() {
        let parsed = args(parse_quote!(#[route(pattern = "/home", count = 5, method = Get)]));

        assert_eq!(
            parsed.string("pattern").expect("a string argument"),
            Some("/home".to_string())
        );
        assert_eq!(parsed.string("absent").expect("an absent argument"), None);
        assert!(parsed.string("count").is_err());
        assert!(parsed.string("method").is_err());
    }

    #[test]
    fn reads_a_path_argument_and_rejects_non_paths() {
        let parsed = args(parse_quote!(#[route(method = Get, pattern = "/home")]));

        assert!(parsed.path("method").expect("a path argument").is_some());
        assert!(parsed.path("absent").expect("an absent argument").is_none());
        assert!(parsed.path("pattern").is_err());
    }

    #[test]
    fn reads_a_boolean_argument_and_rejects_non_booleans() {
        let parsed =
            args(parse_quote!(#[console_argument(required = true, name = "x", kind = Flag)]));

        assert_eq!(parsed.boolean("required").expect("a boolean"), Some(true));
        assert_eq!(parsed.boolean("absent").expect("an absent argument"), None);
        assert!(parsed.boolean("name").is_err());
        assert!(parsed.boolean("kind").is_err());
    }

    #[test]
    fn reports_malformed_list_arguments() {
        let message = AttributeArgs::from_attribute(&parse_quote!(#[route(= 5)]))
            .err()
            .expect("malformed arguments fail to parse")
            .to_string();

        assert!(message.contains("could not be parsed"));
    }
}
