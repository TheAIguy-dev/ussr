use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Data, DeriveInput, Error, Fields, Result};

use crate::common::get_field_info;

pub(crate) fn try_derive_decode(input: &DeriveInput, is_async: bool) -> Result<TokenStream> {
    let module = is_async
        .then_some(quote! {async_decode})
        .unwrap_or(quote! {decode});
    let dot_await = is_async.then_some(quote! {.await}).unwrap_or(quote! {});

    let Data::Struct(data) = &input.data else {
        return Err(Error::new_spanned(
            input,
            "Decode can only be derived for structs",
        ));
    };
    let Fields::Named(fields) = &data.fields else {
        return Err(Error::new_spanned(
            input,
            "Decode can only be derived for structs with named fields",
        ));
    };
    let fields = get_field_info(fields)?;
    let names = fields.iter().map(|(name, _, _, _, _)| name);

    let decode = fields.iter().map(|(name, ty, var, array, with)| {
        if let Some((length_var, length_ty, item_ty)) = array {
            let length = if *length_var {
                quote_spanned! {length_ty.span()=>
                    <#length_ty as ussr_buf:: #module ::VarDecode>::var_decode
                }
            } else {
                quote_spanned! {length_ty.span()=>
                    <#length_ty as ussr_buf:: #module ::Decode>::decode
                }
            };

            let item = if *var {
                quote_spanned! {item_ty.span()=>
                    <#item_ty as ussr_buf:: #module ::VarDecode>::var_decode
                }
            } else {
                quote_spanned! {item_ty.span()=>
                    <#item_ty as ussr_buf:: #module ::Decode>::decode
                }
            };

            let decode_array = is_async
                .then_some(quote! {{
                    let length: usize = #length (reader)
                        .await?
                        .try_into()
                        .expect("Could not convert to usize");
                    let mut buf = Vec::with_capacity(length);

                    for _ in 0..length {
                        buf.push( #item (reader).await?);
                    }

                    buf
                }})
                .unwrap_or(quote! {{
                    let length: usize = #length (reader)?.try_into().expect("Could not convert to usize");
                    (0..length).map(|_| #item (reader)).collect::<Result<_, _>>()?
                }});

            quote! {
                let #name: #ty = #decode_array ;
            }
        } else if *var {
            quote_spanned! {name.span()=>
                let #name: #ty = <#ty as ussr_buf:: #module ::VarDecode>::var_decode(reader) #dot_await ?;
            }
        } else if let Some((read_with, _, async_read_with, _)) = with {
            if is_async {
                quote_spanned! {name.span()=>
                    let #name: #ty = #async_read_with (reader).await?;
                }
            } else {
                quote_spanned! {name.span()=>
                    let #name: #ty = #read_with (reader)?;
                }
            }
        } else {
            quote_spanned! {name.span()=>
                let #name: #ty = <#ty as ussr_buf:: #module ::Decode>::decode(reader) #dot_await ?;
            }
        }
    });

    Ok(quote! {
        #(#decode)*
        Ok(Self {
            #(#names),*
        })
    })
}
