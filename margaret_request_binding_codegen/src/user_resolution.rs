use proc_macro2::TokenStream;

pub(crate) enum UserResolution {
    Bound { value: TokenStream },
    Continued { requirement: TokenStream },
}
