use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Data, DeriveInput, Error, Fields, Result};

use crate::common::get_field_info;

pub(crate) fn try_derive_encode(input: &DeriveInput, is_async: bool) -> Result<TokenStream> {
    let module = is_async
        .then_some(quote! {async_encode})
        .unwrap_or(quote! {encode});
    let dot_await = is_async.then_some(quote! {.await}).unwrap_or(quote! {});

    let Data::Struct(data) = &input.data else {
        return Err(Error::new_spanned(
            input,
            "Encode can only be derived for structs",
        ));
    };
    let Fields::Named(fields) = &data.fields else {
        return Err(Error::new_spanned(
            input,
            "Encode can only be derived for structs with named fields",
        ));
    };
    let fields = get_field_info(fields)?;

    let encode = fields.iter().map(|(name, _, var, array, with)| {
        if let Some((length_var, length_ty, item_ty)) = array {
            let length = if *length_var {
                quote_spanned! {length_ty.span()=>
                    <#length_ty as ussr_buf:: #module ::VarEncode>::var_encode
                }
            } else {
                quote_spanned! {length_ty.span()=>
                    <#length_ty as ussr_buf:: #module ::Encode>::encode
                }
            };

            let item = if *var {
                quote_spanned! {item_ty.span()=>
                    <#item_ty as ussr_buf:: #module ::VarEncode>::var_encode
                }
            } else {
                quote_spanned! {item_ty.span()=>
                    <#item_ty as ussr_buf:: #module ::Encode>::encode
                }
            };

            is_async
                .then_some(quote! {{
                    #length (
                        &self. #name
                            .len()
                            .try_into()
                            .expect("Could not convert from usize"),
                        writer,
                    )
                    .await?;

                    for item in self. #name .iter() {
                        #item (item, writer).await?
                    }
                }})
                .unwrap_or(quote! {
                    #length (
                        &self. #name
                            .len()
                            .try_into()
                            .expect("Could not convert from usize"),
                        writer,
                    )?;
                    self. #name .iter().try_for_each(|item| #item (item, writer))?;
                })
        } else if *var {
            quote_spanned! {name.span()=>
                ussr_buf:: #module ::VarEncode::var_encode(&self.#name, writer) #dot_await ?;
            }
        } else if let Some((_, encode_with, _, async_encode_with)) = with {
            if is_async {
                quote_spanned! {name.span()=>
                    #async_encode_with (&self. #name , writer).await?;
                }
            } else {
                quote_spanned! {name.span()=>
                    #encode_with (&self. #name , writer)?;
                }
            }
        } else {
            quote_spanned! {name.span()=>
                ussr_buf:: #module ::Encode::encode(&self.#name, writer) #dot_await ?;
            }
        }
    });

    Ok(quote! {
        #(#encode)*
        Ok(())
    })
}
