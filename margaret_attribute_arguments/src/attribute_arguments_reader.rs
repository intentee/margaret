use std::fmt::Display;
use std::str::FromStr;

use quote::ToTokens;
use syn::Expr;
use syn::ExprCall;
use syn::Ident;
use syn::Lit;
use syn::Path;

use crate::attribute_args::AttributeArgs;
use crate::attribute_arguments_error::AttributeArgumentsError;
use crate::format_path::format_path;
use crate::named_argument::NamedArgument;

fn positional_variant(expression: &Expr) -> Option<PositionalVariant> {
    match expression {
        Expr::Path(expression) => Some(PositionalVariant {
            arguments: Vec::new(),
            path: expression.path.clone(),
        }),
        Expr::Call(call) => match call.func.as_ref() {
            Expr::Path(function) => Some(PositionalVariant {
                arguments: call.args.iter().cloned().collect(),
                path: function.path.clone(),
            }),
            _ => None,
        },
        _ => None,
    }
}

fn call_name(call: &ExprCall) -> Option<&Ident> {
    match call.func.as_ref() {
        Expr::Path(function) => function.path.get_ident(),
        _ => None,
    }
}

struct PositionalVariant {
    arguments: Vec<Expr>,
    path: Path,
}

pub struct AttributeArgumentsReader {
    attribute_path: String,
    named: Vec<NamedArgument>,
    positional: Vec<Expr>,
}

impl AttributeArgumentsReader {
    pub(crate) fn new(
        attribute_path: String,
        named: Vec<NamedArgument>,
        positional: Vec<Expr>,
    ) -> Self {
        Self {
            attribute_path,
            named,
            positional,
        }
    }

    pub fn take_flag(&mut self, name: &str) -> bool {
        match self.positional.iter().position(|expression| {
            matches!(expression, Expr::Path(path_expression) if path_expression.path.is_ident(name))
        }) {
            Some(index) => {
                self.positional.remove(index);

                true
            }
            None => false,
        }
    }

    /// # Errors
    ///
    /// Returns `AttributeArgumentsError::DuplicateGroup` when the group is written twice,
    /// and the errors of reading the group or of its leftover arguments.
    pub fn take_group<Interpreted, InterpretError>(
        &mut self,
        name: &str,
        read: impl FnOnce(&mut Self) -> Result<Interpreted, InterpretError>,
    ) -> Result<Option<Interpreted>, InterpretError>
    where
        InterpretError: From<AttributeArgumentsError>,
    {
        let mut groups = Vec::new();
        let mut remaining = Vec::with_capacity(self.positional.len());

        for expression in self.positional.drain(..) {
            match expression {
                Expr::Call(call) if call_name(&call).is_some_and(|called| called == name) => {
                    groups.push(call);
                }
                other => remaining.push(other),
            }
        }

        self.positional = remaining;

        let mut groups = groups.into_iter();
        let Some(group) = groups.next() else {
            return Ok(None);
        };

        if groups.next().is_some() {
            return Err(AttributeArgumentsError::DuplicateGroup {
                attribute_path: self.attribute_path.clone(),
                group: name.to_string(),
            }
            .into());
        }

        AttributeArgs::from_expressions(self.nested_path(name), group.args)?
            .interpret(read)
            .map(Some)
    }

    /// # Errors
    ///
    /// Returns `AttributeArgumentsError::UnexpectedArgument`.
    pub fn take_path(&mut self, key: &str) -> Result<Option<Path>, AttributeArgumentsError> {
        match self.take_named(key) {
            None => Ok(None),
            Some(Expr::Path(expression)) => Ok(Some(expression.path)),
            Some(_) => Err(self.unexpected_argument(key, "path")),
        }
    }

    /// # Errors
    ///
    /// Returns `AttributeArgumentsError::UnexpectedArgument`.
    pub fn take_path_array(
        &mut self,
        key: &str,
    ) -> Result<Option<Vec<Path>>, AttributeArgumentsError> {
        let Some(expression) = self.take_named(key) else {
            return Ok(None);
        };
        let Expr::Array(array) = expression else {
            return Err(self.unexpected_argument(key, "array of paths"));
        };

        let mut paths = Vec::with_capacity(array.elems.len());

        for element in array.elems {
            let Expr::Path(element) = element else {
                return Err(self.unexpected_argument(key, "array of paths"));
            };

            paths.push(element.path);
        }

        Ok(Some(paths))
    }

    pub fn take_positional_path(&mut self) -> Option<Path> {
        match self.positional.first() {
            Some(Expr::Path(expression)) => {
                let path = expression.path.clone();

                self.positional.remove(0);

                Some(path)
            }
            _ => None,
        }
    }

    /// # Errors
    ///
    /// Returns the errors of reading the variant or of its leftover arguments.
    pub fn take_positional_variant<Interpreted, InterpretError>(
        &mut self,
        read: impl FnOnce(&Path, &mut Self) -> Result<Interpreted, InterpretError>,
    ) -> Result<Option<Interpreted>, InterpretError>
    where
        InterpretError: From<AttributeArgumentsError>,
    {
        let Some(PositionalVariant { arguments, path }) =
            self.positional.first().and_then(positional_variant)
        else {
            return Ok(None);
        };

        self.positional.remove(0);
        self.read_variant(&format_path(&path), &path, arguments, read)
    }

    /// # Errors
    ///
    /// Returns `AttributeArgumentsError::UnexpectedArgument`.
    pub fn take_string(&mut self, key: &str) -> Result<Option<String>, AttributeArgumentsError> {
        match self.take_named(key) {
            None => Ok(None),
            Some(Expr::Lit(expression)) => match expression.lit {
                Lit::Str(literal) => Ok(Some(literal.value())),
                _ => Err(self.unexpected_argument(key, "string literal")),
            },
            Some(_) => Err(self.unexpected_argument(key, "string literal")),
        }
    }

    /// # Errors
    ///
    /// Returns `AttributeArgumentsError::UnexpectedArgument` or
    /// `AttributeArgumentsError::DuplicateArrayElement`.
    pub fn take_string_array(
        &mut self,
        key: &str,
    ) -> Result<Option<Vec<String>>, AttributeArgumentsError> {
        let Some(expression) = self.take_named(key) else {
            return Ok(None);
        };
        let Expr::Array(array) = expression else {
            return Err(self.unexpected_argument(key, "array of string literals"));
        };

        let mut strings: Vec<String> = Vec::with_capacity(array.elems.len());

        for element in array.elems {
            let Expr::Lit(literal) = element else {
                return Err(self.unexpected_argument(key, "array of string literals"));
            };
            let Lit::Str(literal) = literal.lit else {
                return Err(self.unexpected_argument(key, "array of string literals"));
            };
            let value = literal.value();

            if strings.contains(&value) {
                return Err(AttributeArgumentsError::DuplicateArrayElement {
                    attribute_path: self.attribute_path.clone(),
                    element: value,
                    key: key.to_string(),
                });
            }

            strings.push(value);
        }

        Ok(Some(strings))
    }

    /// # Errors
    ///
    /// Returns `AttributeArgumentsError::UnexpectedArgument` or
    /// `AttributeArgumentsError::MalformedUnsignedInteger`.
    pub fn take_unsigned_integer<Integer>(
        &mut self,
        key: &str,
    ) -> Result<Option<Integer>, AttributeArgumentsError>
    where
        Integer: FromStr,
        Integer::Err: Display,
    {
        match self.take_named(key) {
            None => Ok(None),
            Some(Expr::Lit(expression)) => match expression.lit {
                Lit::Int(literal) => match literal.base10_parse::<Integer>() {
                    Ok(value) => Ok(Some(value)),
                    Err(source) => Err(AttributeArgumentsError::MalformedUnsignedInteger {
                        attribute_path: self.attribute_path.clone(),
                        key: key.to_string(),
                        source,
                    }),
                },
                _ => Err(self.unexpected_argument(key, "unsigned integer literal")),
            },
            Some(_) => Err(self.unexpected_argument(key, "unsigned integer literal")),
        }
    }

    /// # Errors
    ///
    /// Returns `AttributeArgumentsError::UnexpectedArgument` when the value is neither a path nor a
    /// call of a path, and the errors of reading the variant or of its leftover arguments.
    pub fn take_variant<Interpreted, InterpretError>(
        &mut self,
        key: &str,
        read: impl FnOnce(&Path, &mut Self) -> Result<Interpreted, InterpretError>,
    ) -> Result<Option<Interpreted>, InterpretError>
    where
        InterpretError: From<AttributeArgumentsError>,
    {
        match self.take_named(key) {
            None => Ok(None),
            Some(Expr::Path(expression)) => {
                self.read_variant(key, &expression.path, Vec::new(), read)
            }
            Some(Expr::Call(call)) => match call.func.as_ref() {
                Expr::Path(function) => {
                    self.read_variant(key, &function.path, call.args.into_iter().collect(), read)
                }
                _ => Err(self.unexpected_argument(key, "enum variant").into()),
            },
            Some(_) => Err(self.unexpected_argument(key, "enum variant").into()),
        }
    }

    pub(crate) fn into_result(self) -> Result<(), AttributeArgumentsError> {
        if let Some(argument) = self.named.first() {
            return Err(AttributeArgumentsError::UnrecognizedArgument {
                argument: argument.name.clone(),
                attribute_path: self.attribute_path,
            });
        }

        if let Some(argument) = self.positional.first() {
            return Err(AttributeArgumentsError::UnrecognizedArgument {
                argument: argument.to_token_stream().to_string(),
                attribute_path: self.attribute_path,
            });
        }

        Ok(())
    }

    fn nested_path(&self, name: &str) -> String {
        format!("{}::{name}", self.attribute_path)
    }

    fn read_variant<Interpreted, InterpretError>(
        &self,
        key: &str,
        variant: &Path,
        arguments: Vec<Expr>,
        read: impl FnOnce(&Path, &mut Self) -> Result<Interpreted, InterpretError>,
    ) -> Result<Option<Interpreted>, InterpretError>
    where
        InterpretError: From<AttributeArgumentsError>,
    {
        AttributeArgs::from_expressions(self.nested_path(key), arguments)?
            .interpret(|reader| read(variant, reader))
            .map(Some)
    }

    fn take_named(&mut self, key: &str) -> Option<Expr> {
        self.named
            .iter()
            .position(|argument| argument.name == key)
            .map(|index| self.named.remove(index).value)
    }

    fn unexpected_argument(&self, key: &str, expected: &str) -> AttributeArgumentsError {
        AttributeArgumentsError::UnexpectedArgument {
            attribute_path: self.attribute_path.clone(),
            key: key.to_string(),
            expected: expected.to_string(),
        }
    }
}
