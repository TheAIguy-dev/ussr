use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse, Error, Ident, Item, ItemMod, Result};

#[proc_macro_attribute]
pub fn packets(attr: TokenStream, input: TokenStream) -> TokenStream {
    match try_expand(attr, input) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error(),
    }
    .into()
}

fn try_expand(attr: TokenStream, input: TokenStream) -> Result<TokenStream2> {
    let state = parse::<Ident>(attr)?;

    let ItemMod {
        attrs,
        vis,
        unsafety,
        mod_token,
        ident,
        content,
        ..
    } = parse(input).map_err(|_| Error::new_spanned(&state, "#[packets] must be used on a mod"))?;
    let Some((_, items)) = content else {
        return Err(Error::new_spanned(
            &state,
            "#[packets] must be used on a mod with items",
        ));
    };

    let enum_name = Ident::new(&format!("{}Packets", state), state.span());
    let packet_names = items
        .iter()
        .filter_map(|item| match item {
            Item::Struct(s) => Some(&s.ident),
            _ => None,
        })
        .collect::<Vec<_>>();

    let packets_enum = if packet_names.len() > 1 {
        quote! {
            pub enum #enum_name {
                #( #packet_names(#packet_names) ),*
            }
        }
    } else {
        quote! {}
    };

    Ok(quote! {
        #( #attrs )*
        #vis #unsafety #mod_token #ident {
            #( #items )*
            #packets_enum
        }
    })
}
