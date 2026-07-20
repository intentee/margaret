use proc_macro2::TokenStream;
use quote::quote;

pub(crate) enum AssetSlot {
    Absent,
    Present(TokenStream),
}

impl AssetSlot {
    pub(crate) fn into_tokens(self) -> TokenStream {
        match self {
            AssetSlot::Absent => quote! { ::margaret_asset_bag::absent::Absent },
            AssetSlot::Present(tokens) => tokens,
        }
    }

    pub(crate) fn is_present(&self) -> bool {
        matches!(self, AssetSlot::Present(_))
    }
}
