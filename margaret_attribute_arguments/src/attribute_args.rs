use proc_macro2::TokenStream;
use syn::Attribute;
use syn::Expr;
use syn::Meta;
use syn::Path;
use syn::Token;
use syn::parse::Parser as _;
use syn::punctuated::Punctuated;

use crate::attribute_arguments_error::AttributeArgumentsError;
use crate::attribute_arguments_reader::AttributeArgumentsReader;
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
    /// Returns `AttributeArgumentsError::Malformed` or `AttributeArgumentsError::DuplicateNamedArgument`.
    pub fn from_argument_tokens(
        attribute_path: String,
        tokens: TokenStream,
    ) -> Result<Self, AttributeArgumentsError> {
        match Punctuated::<Expr, Token![,]>::parse_terminated.parse2(tokens) {
            Ok(expressions) => Self::from_expressions(attribute_path, expressions),
            Err(source) => Err(AttributeArgumentsError::Malformed {
                attribute_path,
                source,
            }),
        }
    }

    pub(crate) fn from_expressions(
        attribute_path: String,
        expressions: impl IntoIterator<Item = Expr>,
    ) -> Result<Self, AttributeArgumentsError> {
        let mut named: Vec<NamedArgument> = Vec::new();
        let mut positional = Vec::new();

        for expression in expressions {
            match NamedArgument::from_expression(&expression) {
                Some(found) => {
                    if named.iter().any(|existing| existing.name == found.name) {
                        return Err(AttributeArgumentsError::DuplicateNamedArgument {
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

    /// # Errors
    ///
    /// Returns `AttributeArgumentsError::Malformed` or `AttributeArgumentsError::DuplicateNamedArgument`.
    pub fn from_attribute(attribute: &Attribute) -> Result<Self, AttributeArgumentsError> {
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
                Self::from_argument_tokens(attribute_path, meta_list.tokens.clone())
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
        InterpretError: From<AttributeArgumentsError>,
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

    #[must_use]
    pub fn named_path(&self, key: &str) -> Option<&Path> {
        self.named
            .iter()
            .find(|argument| argument.name == key)
            .and_then(|argument| match &argument.value {
                Expr::Path(expression) => Some(&expression.path),
                _ => None,
            })
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::Attribute;
    use syn::parse_quote;

    use crate::attribute_args::AttributeArgs;
    use crate::attribute_arguments_error::AttributeArgumentsError;
    use crate::format_path::format_path;

    #[derive(Debug, PartialEq, Eq)]
    struct ReadArguments {
        method_is_get: Option<bool>,
        pattern: Option<String>,
        primary: bool,
        positional_is_tag: Option<bool>,
    }

    #[derive(Debug, PartialEq, Eq)]
    struct ReadFlags {
        primary_key: bool,
        unique: bool,
    }

    fn parse(attribute: &Attribute) -> Result<AttributeArgs, AttributeArgumentsError> {
        AttributeArgs::from_attribute(attribute)
    }

    fn parsed(attribute: &Attribute) -> AttributeArgs {
        parse(attribute).expect("the arguments parse")
    }

    #[test]
    fn a_bare_attribute_is_empty() {
        assert!(parsed(&parse_quote!(#[singleton])).is_empty());
    }

    #[test]
    fn a_name_value_attribute_carries_one_named_argument() {
        let arguments = parsed(&parse_quote!(#[doc = "text"]));

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
        assert!(parsed(&parse_quote!(#[column()])).is_empty());
    }

    #[test]
    fn interpret_reads_named_and_positional_arguments_together() {
        let arguments =
            parsed(&parse_quote!(#[route(method = Get, pattern = "/home", primary, tag)]));

        let outcome = arguments
            .interpret(|reader| {
                let method = reader.take_path("method").expect("method is a path");
                let pattern = reader.take_string("pattern").expect("pattern is a string");

                Ok::<_, AttributeArgumentsError>(ReadArguments {
                    method_is_get: method.map(|path| path.is_ident("Get")),
                    pattern,
                    primary: reader.take_flag("primary"),
                    positional_is_tag: reader
                        .take_positional_path()
                        .map(|path| path.is_ident("tag")),
                })
            })
            .expect("every argument is consumed");

        assert_eq!(
            outcome,
            ReadArguments {
                method_is_get: Some(true),
                pattern: Some("/home".to_string()),
                primary: true,
                positional_is_tag: Some(true),
            }
        );
    }

    #[test]
    fn interpret_rejects_a_leftover_named_argument() {
        let error = parsed(&parse_quote!(#[index(name = "title", bogus = "x")]))
            .interpret(|reader| reader.take_string("name"))
            .expect_err("the stray named argument is rejected");

        assert!(matches!(
            error,
            AttributeArgumentsError::UnrecognizedArgument { argument, attribute_path }
                if argument == "bogus" && attribute_path == "index"
        ));
    }

    #[test]
    fn interpret_rejects_a_leftover_positional_argument() {
        let error = parsed(&parse_quote!(#[singleton(bogus)]))
            .interpret(|_reader| Ok::<(), AttributeArgumentsError>(()))
            .expect_err("the stray positional argument is rejected");

        assert!(matches!(
            error,
            AttributeArgumentsError::UnrecognizedArgument { argument, attribute_path }
                if argument == "bogus" && attribute_path == "singleton"
        ));
    }

    #[test]
    fn take_string_returns_none_when_the_argument_is_absent() {
        let value = parsed(&parse_quote!(#[column]))
            .interpret(|reader| reader.take_string("name"))
            .expect("an absent string reads as none");

        assert_eq!(value, None);
    }

    #[test]
    fn take_string_rejects_a_non_string_literal() {
        let error = parsed(&parse_quote!(#[column(name = 5)]))
            .interpret(|reader| reader.take_string("name"))
            .expect_err("an integer is not a string literal");

        assert!(matches!(
            error,
            AttributeArgumentsError::UnexpectedArgument { key, expected, .. }
                if key == "name" && expected == "string literal"
        ));
    }

    #[test]
    fn take_string_rejects_a_non_literal_expression() {
        let error = parsed(&parse_quote!(#[column(name = some::path)]))
            .interpret(|reader| reader.take_string("name"))
            .expect_err("a path is not a string literal");

        assert!(matches!(
            error,
            AttributeArgumentsError::UnexpectedArgument { expected, .. }
                if expected == "string literal"
        ));
    }

    #[test]
    fn take_unsigned_integer_returns_none_when_the_argument_is_absent() {
        let value = parsed(&parse_quote!(#[column]))
            .interpret(|reader| reader.take_unsigned_integer::<u32>("precision"))
            .expect("an absent unsigned integer reads as none");

        assert_eq!(value, None);
    }

    #[test]
    fn take_unsigned_integer_reads_an_integer_literal() {
        let value = parsed(&parse_quote!(#[column(precision = 12)]))
            .interpret(|reader| reader.take_unsigned_integer::<u32>("precision"))
            .expect("the unsigned integer reads");

        assert_eq!(value, Some(12));
    }

    #[test]
    fn take_unsigned_integer_rejects_a_non_integer_literal() {
        let error = parsed(&parse_quote!(#[column(precision = "12")]))
            .interpret(|reader| reader.take_unsigned_integer::<u32>("precision"))
            .expect_err("a string is not an unsigned integer literal");

        assert!(matches!(
            error,
            AttributeArgumentsError::UnexpectedArgument { key, expected, .. }
                if key == "precision" && expected == "unsigned integer literal"
        ));
    }

    #[test]
    fn take_unsigned_integer_rejects_a_negative_literal() {
        let error = parsed(&parse_quote!(#[column(precision = -12)]))
            .interpret(|reader| reader.take_unsigned_integer::<u32>("precision"))
            .expect_err("a negative literal is not an unsigned integer literal");

        assert!(matches!(
            error,
            AttributeArgumentsError::UnexpectedArgument { key, expected, .. }
                if key == "precision" && expected == "unsigned integer literal"
        ));
    }

    #[test]
    fn take_unsigned_integer_rejects_a_literal_that_overflows() {
        let error = parsed(&parse_quote!(#[column(precision = 4294967296)]))
            .interpret(|reader| reader.take_unsigned_integer::<u32>("precision"))
            .expect_err("a literal above u32::MAX is rejected");

        assert!(matches!(
            error,
            AttributeArgumentsError::MalformedUnsignedInteger { attribute_path, key, .. }
                if attribute_path == "column" && key == "precision"
        ));
    }

    #[test]
    fn take_path_returns_none_when_the_argument_is_absent() {
        let value = parsed(&parse_quote!(#[foreign_key]))
            .interpret(|reader| reader.take_path("on_delete"))
            .expect("an absent path reads as none");

        assert!(value.is_none());
    }

    #[test]
    fn take_path_rejects_a_non_path_expression() {
        let error = parsed(&parse_quote!(#[foreign_key(on_delete = "cascade")]))
            .interpret(|reader| reader.take_path("on_delete"))
            .expect_err("a string literal is not a path");

        assert!(matches!(
            error,
            AttributeArgumentsError::UnexpectedArgument { key, expected, .. }
                if key == "on_delete" && expected == "path"
        ));
    }

    #[test]
    fn take_flag_reports_absent_and_present_flags() {
        let outcome = parsed(&parse_quote!(#[column(unique)]))
            .interpret(|reader| {
                let unique = reader.take_flag("unique");
                let primary_key = reader.take_flag("primary_key");

                Ok::<_, AttributeArgumentsError>(ReadFlags {
                    primary_key,
                    unique,
                })
            })
            .expect("the flags are consumed");

        assert_eq!(
            outcome,
            ReadFlags {
                primary_key: false,
                unique: true,
            }
        );
    }

    #[test]
    fn take_positional_path_returns_none_when_there_is_no_positional_argument() {
        let value = parsed(&parse_quote!(#[provides]))
            .interpret(|reader| Ok::<_, AttributeArgumentsError>(reader.take_positional_path()))
            .expect("an absent positional path reads as none");

        assert!(value.is_none());
    }

    #[test]
    fn from_attribute_rejects_a_duplicated_named_argument() {
        let error = parse(&parse_quote!(#[index(name = "a", name = "b")]))
            .map(|_| ())
            .expect_err("duplicates fail");

        assert!(matches!(
            error,
            AttributeArgumentsError::DuplicateNamedArgument { attribute_path, key }
                if attribute_path == "index" && key == "name"
        ));
    }

    #[test]
    fn from_attribute_rejects_a_malformed_argument_list() {
        let error = parse(&parse_quote!(#[index(= 5)]))
            .map(|_| ())
            .expect_err("malformed lists fail");

        assert!(matches!(
            error,
            AttributeArgumentsError::Malformed { attribute_path, .. }
                if attribute_path == "index"
        ));
    }

    #[test]
    fn from_argument_tokens_reads_the_arguments_of_a_macro_invocation() {
        let arguments = AttributeArgs::from_argument_tokens(
            "scheduled_with_tick_timer".to_string(),
            quote!(
                interval = TICK_INTERVAL,
                behavior = MissedTickBehavior::Delay
            ),
        )
        .expect("the macro arguments parse");

        assert_eq!(
            arguments.named_path("behavior").map(format_path).as_deref(),
            Some("MissedTickBehavior::Delay")
        );
    }

    #[test]
    fn from_argument_tokens_rejects_malformed_macro_arguments() {
        let error = AttributeArgs::from_argument_tokens("index".to_string(), quote!(= 5))
            .map(|_| ())
            .expect_err("malformed macro arguments fail");

        assert!(matches!(
            error,
            AttributeArgumentsError::Malformed { attribute_path, .. }
                if attribute_path == "index"
        ));
    }

    #[test]
    fn named_path_ignores_an_argument_written_as_another_expression() {
        assert!(
            parsed(&parse_quote!(#[infers_authenticated_user(user_model = "User")]))
                .named_path("user_model")
                .is_none()
        );
    }

    #[test]
    fn named_path_reports_no_path_for_an_absent_argument() {
        assert!(
            parsed(&parse_quote!(#[infers_authenticated_user]))
                .named_path("user_model")
                .is_none()
        );
    }

    #[derive(Debug, PartialEq, Eq)]
    struct ReadGroup {
        refresh: bool,
        scopes: Option<Vec<String>>,
    }

    #[derive(Debug, PartialEq, Eq)]
    struct ReadKeyword {
        introspection: bool,
        keyword: String,
    }

    fn read_group(arguments: &AttributeArgs) -> Result<Option<ReadGroup>, AttributeArgumentsError> {
        arguments.interpret(|reader| {
            reader.take_group("authorization_code", |group| {
                Ok(ReadGroup {
                    refresh: group.take_flag("refresh_token"),
                    scopes: group.take_string_array("scopes")?,
                })
            })
        })
    }

    fn read_keyword(
        arguments: &AttributeArgs,
    ) -> Result<Option<ReadKeyword>, AttributeArgumentsError> {
        arguments.interpret(|reader| {
            reader.take_keyword("authentication", |keyword, arguments| {
                Ok(ReadKeyword {
                    introspection: arguments.take_flag("introspection"),
                    keyword: keyword.to_string(),
                })
            })
        })
    }

    #[test]
    fn reads_an_array_of_string_literals() {
        assert_eq!(
            parsed(&parse_quote!(#[client(scopes = ["openid", "profile"])]))
                .interpret(|reader| reader.take_string_array("scopes"))
                .expect("the array reads"),
            Some(vec!["openid".to_string(), "profile".to_string()])
        );
    }

    #[test]
    fn an_absent_string_array_reads_as_none() {
        assert_eq!(
            parsed(&parse_quote!(#[client]))
                .interpret(|reader| reader.take_string_array("scopes"))
                .expect("the absent array reads"),
            None
        );
    }

    #[test]
    fn rejects_a_string_array_argument_that_is_not_an_array() {
        let error = parsed(&parse_quote!(#[client(scopes = "openid")]))
            .interpret(|reader| reader.take_string_array("scopes"))
            .expect_err("a string is not an array");

        assert!(matches!(
            error,
            AttributeArgumentsError::UnexpectedArgument { expected, key, .. }
                if expected == "array of string literals" && key == "scopes"
        ));
    }

    #[test]
    fn rejects_a_string_array_element_that_is_not_a_literal() {
        let error = parsed(&parse_quote!(#[client(scopes = ["openid", profile])]))
            .interpret(|reader| reader.take_string_array("scopes"))
            .expect_err("a path element is not a string literal");

        assert!(matches!(
            error,
            AttributeArgumentsError::UnexpectedArgument { expected, .. }
                if expected == "array of string literals"
        ));
    }

    #[test]
    fn rejects_a_string_array_element_that_is_another_literal() {
        let error = parsed(&parse_quote!(#[client(scopes = ["openid", 5])]))
            .interpret(|reader| reader.take_string_array("scopes"))
            .expect_err("an integer element is not a string literal");

        assert!(matches!(
            error,
            AttributeArgumentsError::UnexpectedArgument { expected, .. }
                if expected == "array of string literals"
        ));
    }

    #[test]
    fn rejects_a_string_array_that_repeats_an_element() {
        let error = parsed(&parse_quote!(#[client(scopes = ["openid", "openid"])]))
            .interpret(|reader| reader.take_string_array("scopes"))
            .expect_err("a repeated element is ambiguous");

        assert!(matches!(
            error,
            AttributeArgumentsError::DuplicateArrayElement { attribute_path, element, key }
                if attribute_path == "client" && element == "openid" && key == "scopes"
        ));
    }

    #[test]
    fn reads_a_group_with_its_own_arguments() {
        assert_eq!(
            read_group(&parsed(
                &parse_quote!(#[client(authorization_code(refresh_token, scopes = ["openid"]))])
            ))
            .expect("the group reads"),
            Some(ReadGroup {
                refresh: true,
                scopes: Some(vec!["openid".to_string()]),
            })
        );
    }

    #[test]
    fn an_absent_group_reads_as_none() {
        assert_eq!(
            read_group(&parsed(&parse_quote!(#[client]))).expect("the absent group reads"),
            None
        );
    }

    #[test]
    fn rejects_a_group_given_twice() {
        let error = read_group(&parsed(
            &parse_quote!(#[client(authorization_code(), authorization_code())]),
        ))
        .expect_err("two groups are ambiguous");

        assert!(matches!(
            error,
            AttributeArgumentsError::DuplicateGroup { attribute_path, group }
                if attribute_path == "client" && group == "authorization_code"
        ));
    }

    #[test]
    fn reports_a_leftover_argument_of_a_group_under_its_path() {
        let error = read_group(&parsed(
            &parse_quote!(#[client(authorization_code(introspection))]),
        ))
        .expect_err("the group does not accept the argument");

        assert!(matches!(
            error,
            AttributeArgumentsError::UnrecognizedArgument { argument, attribute_path }
                if argument == "introspection" && attribute_path == "client::authorization_code"
        ));
    }

    #[test]
    fn rejects_a_named_argument_repeated_inside_a_group() {
        let error = read_group(&parsed(
            &parse_quote!(#[client(authorization_code(scopes = ["openid"], scopes = ["profile"]))]),
        ))
        .expect_err("a repeated argument is ambiguous");

        assert!(matches!(
            error,
            AttributeArgumentsError::DuplicateNamedArgument { attribute_path, key }
                if attribute_path == "client::authorization_code" && key == "scopes"
        ));
    }

    #[test]
    fn reports_a_malformed_argument_inside_a_group() {
        let error = read_group(&parsed(
            &parse_quote!(#[client(authorization_code(scopes = "openid"))]),
        ))
        .expect_err("the scopes of the group are not an array");

        assert!(matches!(
            error,
            AttributeArgumentsError::UnexpectedArgument { attribute_path, key, .. }
                if attribute_path == "client::authorization_code" && key == "scopes"
        ));
    }

    #[test]
    fn rejects_a_named_argument_repeated_inside_a_keyword() {
        let error = read_keyword(&parsed(
            &parse_quote!(#[client(authentication = private_key_jwt(jwks_uri = "a", jwks_uri = "b"))]),
        ))
        .expect_err("a repeated argument is ambiguous");

        assert!(matches!(
            error,
            AttributeArgumentsError::DuplicateNamedArgument { attribute_path, key }
                if attribute_path == "client::private_key_jwt" && key == "jwks_uri"
        ));
    }

    #[test]
    fn reads_a_bare_keyword() {
        assert_eq!(
            read_keyword(&parsed(&parse_quote!(#[client(authentication = none)])))
                .expect("the keyword reads"),
            Some(ReadKeyword {
                introspection: false,
                keyword: "none".to_string(),
            })
        );
    }

    #[test]
    fn reads_a_keyword_with_arguments() {
        assert_eq!(
            read_keyword(&parsed(
                &parse_quote!(#[client(authentication = private_key_jwt(introspection))])
            ))
            .expect("the keyword reads"),
            Some(ReadKeyword {
                introspection: true,
                keyword: "private_key_jwt".to_string(),
            })
        );
    }

    #[test]
    fn an_absent_keyword_reads_as_none() {
        assert_eq!(
            read_keyword(&parsed(&parse_quote!(#[client]))).expect("the absent keyword reads"),
            None
        );
    }

    #[test]
    fn rejects_a_keyword_written_as_a_qualified_path() {
        let error = read_keyword(&parsed(
            &parse_quote!(#[client(authentication = auth::none)]),
        ))
        .expect_err("a qualified path is not a keyword");

        assert!(matches!(
            error,
            AttributeArgumentsError::UnexpectedArgument { expected, key, .. }
                if expected == "keyword" && key == "authentication"
        ));
    }

    #[test]
    fn rejects_a_keyword_call_of_a_qualified_path() {
        let error = read_keyword(&parsed(
            &parse_quote!(#[client(authentication = auth::private_key_jwt(introspection))]),
        ))
        .expect_err("a qualified call is not a keyword");

        assert!(matches!(
            error,
            AttributeArgumentsError::UnexpectedArgument { expected, .. } if expected == "keyword"
        ));
    }

    #[test]
    fn rejects_a_keyword_call_of_an_expression() {
        let error = read_keyword(&parsed(
            &parse_quote!(#[client(authentication = (private_key_jwt)(introspection))]),
        ))
        .expect_err("a call of a parenthesized expression is not a keyword");

        assert!(matches!(
            error,
            AttributeArgumentsError::UnexpectedArgument { expected, .. } if expected == "keyword"
        ));
    }

    #[test]
    fn rejects_a_keyword_written_as_a_literal() {
        let error = read_keyword(&parsed(&parse_quote!(#[client(authentication = "none")])))
            .expect_err("a string literal is not a keyword");

        assert!(matches!(
            error,
            AttributeArgumentsError::UnexpectedArgument { expected, .. } if expected == "keyword"
        ));
    }

    #[test]
    fn reports_a_leftover_argument_of_a_keyword_under_its_path() {
        let error = read_keyword(&parsed(
            &parse_quote!(#[client(authentication = none(scopes = ["openid"]))]),
        ))
        .expect_err("the keyword does not accept the argument");

        assert!(matches!(
            error,
            AttributeArgumentsError::UnrecognizedArgument { argument, attribute_path }
                if argument == "scopes" && attribute_path == "client::none"
        ));
    }

    #[test]
    fn reads_an_array_of_paths() {
        let columns = parsed(&parse_quote!(#[foreign_key(columns = [partition, hash])]))
            .interpret(|reader| reader.take_path_array("columns"))
            .expect("the array reads")
            .expect("the argument is present");

        assert_eq!(
            columns.iter().map(format_path).collect::<Vec<String>>(),
            vec!["partition".to_string(), "hash".to_string()]
        );
    }

    #[test]
    fn an_absent_path_array_reads_as_none() {
        assert!(
            parsed(&parse_quote!(#[foreign_key]))
                .interpret(|reader| reader.take_path_array("columns"))
                .expect("the absent array reads")
                .is_none()
        );
    }

    #[test]
    fn rejects_a_path_array_argument_that_is_not_an_array() {
        let error = parsed(&parse_quote!(#[foreign_key(columns = "partition")]))
            .interpret(|reader| reader.take_path_array("columns"))
            .expect_err("a string is not an array of paths");

        assert!(matches!(
            error,
            AttributeArgumentsError::UnexpectedArgument { expected, key, .. }
                if expected == "array of paths" && key == "columns"
        ));
    }

    #[test]
    fn rejects_a_path_array_element_that_is_not_a_path() {
        let error = parsed(&parse_quote!(#[foreign_key(columns = [partition, "hash"])]))
            .interpret(|reader| reader.take_path_array("columns"))
            .expect_err("a string element is not a path");

        assert!(matches!(
            error,
            AttributeArgumentsError::UnexpectedArgument { expected, key, .. }
                if expected == "array of paths" && key == "columns"
        ));
    }
}
