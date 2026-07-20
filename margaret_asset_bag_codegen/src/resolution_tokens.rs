use proc_macro2::TokenStream;
use quote::quote;

use crate::asset_slot::AssetSlot;
use crate::static_output_slots::StaticOutputSlots;

pub(crate) fn resolution_tokens(
    bundle: AssetSlot,
    StaticOutputSlots { image, file }: StaticOutputSlots,
) -> TokenStream {
    let bundle = bundle.into_tokens();
    let image = image.into_tokens();
    let file = file.into_tokens();

    quote! {
        ::margaret_asset_bag::asset_resolution::AssetResolution::new(#bundle, #image, #file)
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::resolution_tokens;
    use crate::asset_slot::AssetSlot;
    use crate::static_output_slots::StaticOutputSlots;

    #[test]
    fn composes_present_and_absent_slots_into_an_asset_resolution() {
        let tokens = resolution_tokens(
            AssetSlot::Present(quote! { BUNDLE }),
            StaticOutputSlots {
                image: AssetSlot::Present(quote! { IMAGE }),
                file: AssetSlot::Absent,
            },
        );

        assert_eq!(
            tokens.to_string(),
            quote! {
                ::margaret_asset_bag::asset_resolution::AssetResolution::new(
                    BUNDLE,
                    IMAGE,
                    ::margaret_asset_bag::absent::Absent
                )
            }
            .to_string()
        );
    }
}
