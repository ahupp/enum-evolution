use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{
    braced, parse::Parse, parse::ParseStream, parse_macro_input, Ident, ItemEnum, Result, Token,
    Variant,
};

mod kw {
    syn::custom_keyword!(derive);
    syn::custom_keyword!(from);
    syn::custom_keyword!(remove);
}

struct EnumEvolution {
    base: ItemEnum,
    derives: Vec<DerivedEnum>,
}

struct DerivedEnum {
    name: Ident,
    source: Ident,
    removed: Vec<Ident>,
}

impl Parse for EnumEvolution {
    fn parse(input: ParseStream) -> Result<Self> {
        let base: ItemEnum = input.parse()?;
        let mut derives = Vec::new();
        while !input.is_empty() {
            derives.push(input.parse()?);
        }
        Ok(EnumEvolution { base, derives })
    }
}

impl Parse for DerivedEnum {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<kw::derive>()?;
        let name: Ident = input.parse()?;
        input.parse::<kw::from>()?;
        let source: Ident = input.parse()?;
        let content; 
        braced!(content in input);
        let mut removed = Vec::new();
        while !content.is_empty() {
            content.parse::<kw::remove>()?;
            let ident: Ident = content.parse()?;
            removed.push(ident);
            let _ = content.parse::<Token![;]>().ok();
        }
        Ok(DerivedEnum { name, source, removed })
    }
}

#[proc_macro]
pub fn enum_evolution(input: TokenStream) -> TokenStream {
    let EnumEvolution { base, derives } = parse_macro_input!(input as EnumEvolution);

    // Keep track of generated enums so subsequent derives can build on earlier ones.
    let mut known: HashMap<String, ItemEnum> = HashMap::new();
    known.insert(base.ident.to_string(), base.clone());

    let mut tokens = quote!(#base);

    for d in derives {
        if let Some(src) = known.get(&d.source.to_string()) {
            let removed: std::collections::HashSet<String> =
                d.removed.iter().map(|i| i.to_string()).collect();
            let variants: syn::punctuated::Punctuated<Variant, Token![,]> = src
                .variants
                .iter()
                .filter(|v| !removed.contains(&v.ident.to_string()))
                .cloned()
                .collect();

            let mut new_enum = src.clone();
            new_enum.ident = d.name.clone();
            new_enum.variants = variants;
            tokens.extend(quote!(#new_enum));
            known.insert(new_enum.ident.to_string(), new_enum);
        }
    }

    tokens.into()
}
