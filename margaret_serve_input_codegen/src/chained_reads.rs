use proc_macro2::TokenStream;
use quote::quote;

use crate::chained_read::ChainedRead;
use crate::read_expression::ReadExpression;

enum Chained {
    Result(TokenStream),
    Value(TokenStream),
}

#[must_use]
pub fn chained_reads(reads: &[ChainedRead], finish: &TokenStream) -> TokenStream {
    let chained = reads.iter().rev().fold(
        Chained::Value(finish.clone()),
        |chained, ChainedRead { binding, read }| match (read, chained) {
            (ReadExpression::Infallible(expression), Chained::Value(body)) => {
                Chained::Value(quote! {{ let #binding = #expression; #body }})
            }
            (ReadExpression::Infallible(expression), Chained::Result(body)) => {
                Chained::Result(quote! {{ let #binding = #expression; #body }})
            }
            (ReadExpression::Fallible(expression), Chained::Value(body)) => {
                Chained::Result(quote! { #expression.map(|#binding| #body) })
            }
            (ReadExpression::Fallible(expression), Chained::Result(body)) => {
                Chained::Result(quote! { #expression.and_then(|#binding| #body) })
            }
        },
    );

    match chained {
        Chained::Result(body) => body,
        Chained::Value(body) => quote! { ::std::result::Result::Ok(#body) },
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::chained_reads;
    use crate::chained_read::ChainedRead;
    use crate::read_expression::ReadExpression;

    fn collapsed(reads: &[ChainedRead]) -> String {
        chained_reads(reads, &quote! { finish(first, second) })
            .to_string()
            .split_whitespace()
            .collect()
    }

    fn read(binding: &str, read: ReadExpression) -> ChainedRead {
        let binding = quote::format_ident!("{binding}");

        ChainedRead {
            binding: quote! { #binding },
            read,
        }
    }

    #[test]
    fn wraps_a_value_built_without_fallible_reads_in_ok() {
        assert_eq!(
            collapsed(&[]),
            "::std::result::Result::Ok(finish(first,second))"
        );
    }

    #[test]
    fn maps_the_last_fallible_read_into_the_value_so_no_ok_wrapping_is_needed() {
        assert_eq!(
            collapsed(&[
                read("first", ReadExpression::Fallible(quote! { read_first() })),
                read("second", ReadExpression::Fallible(quote! { read_second() })),
            ]),
            "read_first().and_then(|first|read_second().map(|second|finish(first,second)))"
        );
    }

    #[test]
    fn binds_an_infallible_read_inside_the_closure_of_the_fallible_read_before_it() {
        assert_eq!(
            collapsed(&[
                read("first", ReadExpression::Fallible(quote! { read_first() })),
                read(
                    "second",
                    ReadExpression::Infallible(quote! { derive(&first) })
                ),
            ]),
            "read_first().map(|first|{letsecond=derive(&first);finish(first,second)})"
        );
    }

    #[test]
    fn binds_an_infallible_read_ahead_of_a_later_fallible_read() {
        assert_eq!(
            collapsed(&[
                read("first", ReadExpression::Infallible(quote! { known() })),
                read("second", ReadExpression::Fallible(quote! { read_second() })),
            ]),
            "{letfirst=known();read_second().map(|second|finish(first,second))}"
        );
    }

    #[test]
    fn wraps_only_infallible_reads_in_ok() {
        assert_eq!(
            collapsed(&[read(
                "first",
                ReadExpression::Infallible(quote! { known() })
            )]),
            "::std::result::Result::Ok({letfirst=known();finish(first,second)})"
        );
    }
}
