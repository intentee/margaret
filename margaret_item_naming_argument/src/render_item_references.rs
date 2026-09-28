use proc_macro2::TokenStream;
use quote::quote;

use margaret_attribute_arguments::attribute_args::AttributeArgs;

use crate::item_naming_argument::ItemNamingArgument;
use crate::named_item::NamedItem;

#[must_use]
pub fn render_item_references(
    arguments: &AttributeArgs,
    item_naming_arguments: &[ItemNamingArgument],
) -> TokenStream {
    let references: Vec<TokenStream> = item_naming_arguments
        .iter()
        .filter_map(|item_naming_argument| {
            arguments
                .named_path(item_naming_argument.key())
                .map(|path| match item_naming_argument.named_item() {
                    NamedItem::Type => quote! {
                        let _: ::core::marker::PhantomData<#path> = ::core::marker::PhantomData;
                    },
                    NamedItem::Value => quote! {
                        let _ = &#path;
                    },
                })
        })
        .collect();

    if references.is_empty() {
        return TokenStream::new();
    }

    quote! {
        const _: () = {
            #(#references)*
        };
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::parse_quote;

    use margaret_attribute_arguments::attribute_args::AttributeArgs;

    use super::render_item_references;
    use crate::item_naming_argument::ItemNamingArgument;

    fn rendered(arguments: &AttributeArgs, item_naming_arguments: &[ItemNamingArgument]) -> String {
        render_item_references(arguments, item_naming_arguments).to_string()
    }

    #[test]
    fn references_a_named_type_through_phantom_data() {
        let arguments = AttributeArgs::from_attribute(&parse_quote!(
            #[infers_authenticated_user(user_model = User)]
        ))
        .expect("the arguments parse");

        assert_eq!(
            rendered(&arguments, &[ItemNamingArgument::UserModel]),
            quote! {
                const _: () = {
                    let _: ::core::marker::PhantomData<User> = ::core::marker::PhantomData;
                };
            }
            .to_string()
        );
    }

    #[test]
    fn references_every_named_value_in_one_constant() {
        let arguments = AttributeArgs::from_attribute(&parse_quote!(
            #[scheduled_with_tick_timer(interval = TICK_INTERVAL, behavior = MissedTickBehavior::Delay)]
        ))
        .expect("the arguments parse");

        assert_eq!(
            rendered(
                &arguments,
                &[
                    ItemNamingArgument::TickInterval,
                    ItemNamingArgument::TickBehavior,
                ]
            ),
            quote! {
                const _: () = {
                    let _ = &TICK_INTERVAL;
                    let _ = &MissedTickBehavior::Delay;
                };
            }
            .to_string()
        );
    }

    #[test]
    fn renders_nothing_when_no_argument_names_an_item() {
        let arguments = AttributeArgs::from_attribute(&parse_quote!(#[scheduled_with_tick_timer]))
            .expect("the arguments parse");

        assert!(render_item_references(&arguments, &[ItemNamingArgument::TickInterval]).is_empty());
    }
}
