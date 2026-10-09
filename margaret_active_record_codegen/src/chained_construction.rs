use proc_macro2::TokenStream;
use quote::quote;

use crate::bound_read::BoundRead;

pub(crate) fn chained_construction(reads: &[BoundRead], construction: &TokenStream) -> TokenStream {
    reads.split_last().map_or_else(
        || quote! { ::std::result::Result::Ok(#construction) },
        |(BoundRead { read, value }, earlier)| {
            earlier.iter().rev().fold(
                quote! { #read.map(|#value| #construction) },
                |chained, BoundRead { read, value }| quote! { #read.and_then(|#value| #chained) },
            )
        },
    )
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::chained_construction;
    use crate::bound_read::BoundRead;
    use crate::value_identifier::value_identifier;

    #[test]
    fn constructs_directly_without_reads() {
        assert_eq!(
            chained_construction(&[], &quote! { Self {} }).to_string(),
            quote! { ::std::result::Result::Ok(Self {}) }.to_string()
        );
    }

    #[test]
    fn binds_each_read_before_the_construction() {
        assert_eq!(
            chained_construction(
                &[
                    BoundRead {
                        read: quote! { first(cursor) },
                        value: value_identifier(0),
                    },
                    BoundRead {
                        read: quote! { second(cursor) },
                        value: value_identifier(1),
                    },
                ],
                &quote! { Self { a: value_0, b: value_1 } },
            )
            .to_string(),
            quote! {
                first(cursor).and_then(|value_0| second(cursor).map(|value_1|
                    Self { a: value_0, b: value_1 }))
            }
            .to_string()
        );
    }
}
