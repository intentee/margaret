use proc_macro2::TokenStream;

use crate::read_expression::ReadExpression;

#[derive(Clone)]
pub struct ChainedRead {
    pub binding: TokenStream,
    pub read: ReadExpression,
}
