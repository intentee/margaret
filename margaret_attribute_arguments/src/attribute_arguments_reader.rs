use quote::ToTokens;
use syn::Expr;
use syn::Lit;
use syn::Path;

use crate::attribute_arguments_error::AttributeArgumentsError;
use crate::named_argument::NamedArgument;

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
    /// `AttributeArgumentsError::MalformedUnsignedInteger`.
    pub fn take_unsigned_integer(
        &mut self,
        key: &str,
    ) -> Result<Option<u32>, AttributeArgumentsError> {
        match self.take_named(key) {
            None => Ok(None),
            Some(Expr::Lit(expression)) => match expression.lit {
                Lit::Int(literal) => match literal.base10_parse::<u32>() {
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
