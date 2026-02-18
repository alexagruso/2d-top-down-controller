use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(SectorMessage)]
pub fn derive_sector_message(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote::quote! {
        impl SectorMessage for #ident {}
    };
    output.into()
}
