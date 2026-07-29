use syn::Attribute;
use syn::Expr;
use syn::Meta;
use syn::Token;
use syn::punctuated::Punctuated;

use crate::attribute_args_parse_error::AttributeArgsParseError;
use crate::attribute_arguments_reader::AttributeArgumentsReader;
use crate::attribute_error::AttributeError;
use crate::format_path::format_path;
use crate::named_argument::NamedArgument;

#[derive(Clone, Debug)]
pub struct AttributeArgs {
    attribute_path: String,
    named: Vec<NamedArgument>,
    positional: Vec<Expr>,
}

impl AttributeArgs {
    /// # Errors
    ///
    /// Returns `AttributeArgsParseError::Malformed` or `AttributeArgsParseError::DuplicateNamedArgument`.
    pub fn from_attribute(attribute: &Attribute) -> Result<Self, AttributeArgsParseError> {
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
                        return Err(AttributeArgsParseError::Malformed {
                            attribute_path,
                            source,
                        });
                    }
                };

                let mut named: Vec<NamedArgument> = Vec::new();
                let mut positional = Vec::new();

                for expression in expressions {
                    match NamedArgument::from_expression(&expression) {
                        Some(found) => {
                            if named.iter().any(|existing| existing.name == found.name) {
                                return Err(AttributeArgsParseError::DuplicateNamedArgument {
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

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    pub fn interpret<Interpreted, InterpretError>(
        &self,
        interpret: impl FnOnce(&mut AttributeArgumentsReader) -> Result<Interpreted, InterpretError>,
    ) -> Result<Interpreted, InterpretError>
    where
        InterpretError: From<AttributeError>,
    {
        let mut reader = AttributeArgumentsReader::new(
            self.attribute_path.clone(),
            self.named.clone(),
            self.positional.clone(),
        );
        let interpreted = interpret(&mut reader)?;

        reader.into_result()?;

        Ok(interpreted)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.named.is_empty() && self.positional.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use syn::Attribute;
    use syn::parse_quote;

    use crate::attribute_args::AttributeArgs;
    use crate::attribute_args_parse_error::AttributeArgsParseError;
    use crate::attribute_error::AttributeError;

    fn parse(attribute: Attribute) -> Result<AttributeArgs, AttributeArgsParseError> {
        AttributeArgs::from_attribute(&attribute)
    }

    fn parsed(attribute: Attribute) -> AttributeArgs {
        parse(attribute).expect("the arguments parse")
    }

    #[test]
    fn a_bare_attribute_is_empty() {
        assert!(parsed(parse_quote!(#[singleton])).is_empty());
    }

    #[test]
    fn a_name_value_attribute_carries_one_named_argument() {
        let arguments = parsed(parse_quote!(#[doc = "text"]));

        assert!(!arguments.is_empty());
        assert_eq!(
            arguments
                .interpret(|reader| reader.take_string("doc"))
                .expect("the value reads"),
            Some("text".to_string())
        );
    }

    #[test]
    fn a_list_attribute_is_empty_when_it_has_no_arguments() {
        assert!(parsed(parse_quote!(#[column()])).is_empty());
    }

    #[test]
    fn interpret_reads_named_and_positional_arguments_together() {
        let arguments =
            parsed(parse_quote!(#[route(method = Get, pattern = "/home", primary, tag)]));

        let outcome = arguments
            .interpret(|reader| {
                let method = reader.take_path("method").expect("method is a path");
                let pattern = reader.take_string("pattern").expect("pattern is a string");

                Ok::<_, AttributeError>((
                    method.map(|path| path.is_ident("Get")),
                    pattern,
                    reader.take_flag("primary"),
                    reader
                        .take_positional_path()
                        .map(|path| path.is_ident("tag")),
                ))
            })
            .expect("every argument is consumed");

        assert_eq!(
            outcome,
            (Some(true), Some("/home".to_string()), true, Some(true))
        );
    }

    #[test]
    fn interpret_rejects_a_leftover_named_argument() {
        let error = parsed(parse_quote!(#[index(name = "title", bogus = "x")]))
            .interpret(|reader| reader.take_string("name"))
            .expect_err("the stray named argument is rejected");

        assert!(matches!(
            error,
            AttributeError::UnrecognizedArgument { argument, attribute_path }
                if argument == "bogus" && attribute_path == "index"
        ));
    }

    #[test]
    fn interpret_rejects_a_leftover_positional_argument() {
        let error = parsed(parse_quote!(#[singleton(bogus)]))
            .interpret(|_reader| Ok::<(), AttributeError>(()))
            .expect_err("the stray positional argument is rejected");

        assert!(matches!(
            error,
            AttributeError::UnrecognizedArgument { argument, attribute_path }
                if argument == "bogus" && attribute_path == "singleton"
        ));
    }

    #[test]
    fn take_string_returns_none_when_the_argument_is_absent() {
        let value = parsed(parse_quote!(#[column]))
            .interpret(|reader| reader.take_string("name"))
            .expect("an absent string reads as none");

        assert_eq!(value, None);
    }

    #[test]
    fn take_string_rejects_a_non_string_literal() {
        let error = parsed(parse_quote!(#[column(name = 5)]))
            .interpret(|reader| reader.take_string("name"))
            .expect_err("an integer is not a string literal");

        assert!(matches!(
            error,
            AttributeError::UnexpectedArgument { key, expected, .. }
                if key == "name" && expected == "string literal"
        ));
    }

    #[test]
    fn take_string_rejects_a_non_literal_expression() {
        let error = parsed(parse_quote!(#[column(name = some::path)]))
            .interpret(|reader| reader.take_string("name"))
            .expect_err("a path is not a string literal");

        assert!(matches!(
            error,
            AttributeError::UnexpectedArgument { expected, .. }
                if expected == "string literal"
        ));
    }

    #[test]
    fn take_path_returns_none_when_the_argument_is_absent() {
        let value = parsed(parse_quote!(#[foreign_key]))
            .interpret(|reader| reader.take_path("on_delete"))
            .expect("an absent path reads as none");

        assert!(value.is_none());
    }

    #[test]
    fn take_path_rejects_a_non_path_expression() {
        let error = parsed(parse_quote!(#[foreign_key(on_delete = "cascade")]))
            .interpret(|reader| reader.take_path("on_delete"))
            .expect_err("a string literal is not a path");

        assert!(matches!(
            error,
            AttributeError::UnexpectedArgument { key, expected, .. }
                if key == "on_delete" && expected == "path"
        ));
    }

    #[test]
    fn take_flag_reports_absent_and_present_flags() {
        let outcome = parsed(parse_quote!(#[column(unique)]))
            .interpret(|reader| {
                let unique = reader.take_flag("unique");
                let primary_key = reader.take_flag("primary_key");

                Ok::<_, AttributeError>((unique, primary_key))
            })
            .expect("the flags are consumed");

        assert_eq!(outcome, (true, false));
    }

    #[test]
    fn take_positional_path_returns_none_when_there_is_no_positional_argument() {
        let value = parsed(parse_quote!(#[provides]))
            .interpret(|reader| Ok::<_, AttributeError>(reader.take_positional_path()))
            .expect("an absent positional path reads as none");

        assert!(value.is_none());
    }

    #[test]
    fn from_attribute_rejects_a_duplicated_named_argument() {
        let error = parse(parse_quote!(#[index(name = "a", name = "b")]))
            .map(|_| ())
            .expect_err("duplicates fail");

        assert!(matches!(
            error,
            AttributeArgsParseError::DuplicateNamedArgument { attribute_path, key }
                if attribute_path == "index" && key == "name"
        ));
    }

    #[test]
    fn from_attribute_rejects_a_malformed_argument_list() {
        let error = parse(parse_quote!(#[index(= 5)]))
            .map(|_| ())
            .expect_err("malformed lists fail");

        assert!(matches!(
            error,
            AttributeArgsParseError::Malformed { attribute_path, .. }
                if attribute_path == "index"
        ));
    }
}
