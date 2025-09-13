use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemEnum};

#[proc_macro]
pub fn enum_evolution(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemEnum);
    quote!(#item).into()
}
