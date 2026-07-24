use quote::ToTokens;
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

                let mut named: Vec<NamedArgument> = Vec::new();
                let mut positional = Vec::new();

                for expression in expressions {
                    match named_argument(&expression) {
                        Some(found) => {
                            if named.iter().any(|existing| existing.name == found.name) {
                                return Err(AttributeError::DuplicateNamedArgument {
                                    attribute_path,
                                    key: found.name,
                                });
                            }

                            named.push(found);
                        }
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

    pub fn expect_only(
        &self,
        allowed_named: &[&str],
        allowed_positional_flags: &[&str],
    ) -> Result<(), AttributeError> {
        self.reject_unknown_named(allowed_named)?;

        for expression in &self.positional {
            let is_allowed_flag = match expression {
                Expr::Path(path_expression) => allowed_positional_flags
                    .iter()
                    .any(|flag| path_expression.path.is_ident(*flag)),
                _ => false,
            };

            if !is_allowed_flag {
                return Err(AttributeError::UnexpectedPositionalArgument {
                    argument: expression.to_token_stream().to_string(),
                    attribute_path: self.attribute_path.clone(),
                });
            }
        }

        Ok(())
    }

    #[must_use]
    pub fn has_positional_flag(&self, name: &str) -> bool {
        self.positional.iter().any(|expression| match expression {
            Expr::Path(expression) => expression.path.is_ident(name),
            _ => false,
        })
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.named.is_empty() && self.positional.is_empty()
    }

    #[must_use]
    pub fn named(&self, key: &str) -> Option<&Expr> {
        self.named
            .iter()
            .find(|argument| argument.name == key)
            .map(|argument| &argument.value)
    }

    pub fn path(&self, key: &str) -> Result<Option<Path>, AttributeError> {
        match self.named(key) {
            None => Ok(None),
            Some(Expr::Path(expression)) => Ok(Some(expression.path.clone())),
            Some(_) => Err(self.unexpected_argument(key, "path")),
        }
    }

    #[must_use]
    pub fn positional(&self, index: usize) -> Option<&Expr> {
        self.positional.get(index)
    }

    #[must_use]
    pub fn positional_path(&self, index: usize) -> Option<&Path> {
        match self.positional.get(index) {
            Some(Expr::Path(expression)) => Some(&expression.path),
            _ => None,
        }
    }

    pub fn reject_unknown_named(&self, allowed_named: &[&str]) -> Result<(), AttributeError> {
        for argument in &self.named {
            if !allowed_named.contains(&argument.name.as_str()) {
                return Err(AttributeError::UnknownNamedArgument {
                    attribute_path: self.attribute_path.clone(),
                    key: argument.name.clone(),
                });
            }
        }

        Ok(())
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

    fn unexpected_argument(&self, key: &str, expected: &str) -> AttributeError {
        AttributeError::UnexpectedArgument {
            attribute_path: self.attribute_path.clone(),
            key: key.to_string(),
            expected: expected.to_string(),
        }
    }
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

    #[test]
    fn rejects_a_duplicate_named_argument() {
        let message =
            AttributeArgs::from_attribute(&parse_quote!(#[route(method = Get, method = Post)]))
                .err()
                .expect("a duplicate named argument is rejected")
                .to_string();

        assert!(message.contains("provided more than once"));
    }

    #[test]
    fn reads_a_positional_path_and_ignores_non_paths() {
        let parsed = args(parse_quote!(#[tagged(crate::markers::Tag, 5)]));

        assert!(parsed.positional_path(0).is_some());
        assert!(parsed.positional_path(1).is_none());
        assert!(parsed.positional_path(99).is_none());
    }

    #[test]
    fn detects_a_bare_positional_flag() {
        let parsed = args(parse_quote!(#[column(primary_key, name = "id", 5)]));

        assert!(parsed.has_positional_flag("primary_key"));
        assert!(!parsed.has_positional_flag("unique"));
    }

    #[test]
    fn expect_only_accepts_declared_named_and_positional_flags() {
        let parsed = args(parse_quote!(#[column(primary_key, name = "id")]));

        parsed
            .expect_only(&["name"], &["primary_key", "unique"])
            .expect("the declared arguments are accepted");
    }

    #[test]
    fn expect_only_rejects_an_unknown_named_argument() {
        let parsed = args(parse_quote!(#[index(nmae = "id")]));

        let message = parsed
            .expect_only(&["name"], &[])
            .expect_err("the unknown named argument is rejected")
            .to_string();

        assert!(message.contains("unknown argument 'nmae'"));
    }

    #[test]
    fn expect_only_rejects_an_unexpected_positional_flag() {
        let parsed = args(parse_quote!(#[index(unexpected)]));

        let message = parsed
            .expect_only(&["name"], &[])
            .expect_err("the unexpected positional is rejected")
            .to_string();

        assert!(message.contains("unexpected positional argument 'unexpected'"));
    }

    #[test]
    fn expect_only_rejects_a_non_path_positional_argument() {
        let parsed = args(parse_quote!(#[index(5)]));

        assert!(parsed.expect_only(&["name"], &[]).is_err());
    }

    #[test]
    fn reject_unknown_named_accepts_only_declared_keys() {
        let parsed = args(parse_quote!(#[tagged(crate::Tag, attribute = Foo)]));

        parsed
            .reject_unknown_named(&["attribute"])
            .expect("the declared named key is accepted");
        assert!(parsed.reject_unknown_named(&[]).is_err());
    }
}
