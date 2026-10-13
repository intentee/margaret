use proc_macro2::TokenStream;

#[derive(Clone)]
pub enum ReadExpression {
    Fallible(TokenStream),
    Infallible(TokenStream),
}
