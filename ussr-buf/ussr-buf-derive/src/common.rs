use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    parse_str, punctuated::Punctuated, DeriveInput, Error, Expr, FieldsNamed, Meta, Path, Result,
    Signature, Token, Type,
};

pub(crate) fn wrap_result(
    result: Result<TokenStream>,
    input: &DeriveInput,
    trait_name: &str,
    function_name: &str,
) -> TokenStream {
    let tokens = result.unwrap_or_else(|err: Error| err.to_compile_error());
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let trait_name: Path = parse_str(trait_name).unwrap();
    let function_name: Signature = parse_str(function_name).unwrap();

    quote! {
        #[automatically_derived]
        impl #impl_generics ussr_buf::#trait_name for #name #ty_generics #where_clause {
            #function_name  {
                #tokens
            }
        }
    }
}

pub(crate) fn get_field_info(
    fields: &FieldsNamed,
) -> Result<
    Vec<(
        &Ident,
        &Type,
        bool,
        Option<(bool, Expr, Expr)>,
        Option<(Expr, Expr, Expr, Expr)>,
    )>,
> {
    fields
        .named
        .iter()
        .map(|f| {
            let mut var = false;
            let mut array = None;
            let mut with = None;

            for attr in &f.attrs {
                if attr.path().is_ident("var") {
                    if var {
                        return Err(Error::new_spanned(attr, "duplicate #[var] attribute"));
                    }

                    if with.is_some() {
                        return Err(Error::new_spanned(
                            attr,
                            "cannot have both #[var] and #[with]",
                        ));
                    }

                    if !matches!(attr.meta, Meta::Path(_)) {
                        return Err(Error::new_spanned(
                            attr,
                            "the #[var] attribute must not have arguments",
                        ));
                    }

                    var = true;
                } else if attr.path().is_ident("array") {
                    if array.is_some() {
                        return Err(Error::new_spanned(attr, "duplicate #[array] attribute"));
                    }

                    if with.is_some() {
                        return Err(Error::new_spanned(
                            attr,
                            "cannot have both #[array] and #[with]",
                        ));
                    }

                    match &attr.meta {
                        Meta::List(list) => {
                            let exprs: Punctuated<Expr, Token![,]> =
                                list.parse_args_with(Punctuated::parse_terminated)?;

                            match exprs.len() {
                                2 => array = Some((false, exprs[0].clone(), exprs[1].clone())),
                                3 => {
                                    if exprs[0].to_token_stream().to_string() != "var" {
                                        return Err(Error::new_spanned(
                                            &exprs[0],
                                            "the first argument of #[array] must be `var`",
                                        ));
                                    }
                                    array = Some((true, exprs[1].clone(), exprs[2].clone()));
                                }
                                _ => return Err(Error::new_spanned(
                                    attr,
                                    "the #[array] attribute must have exactly two or three arguments",
                                )),
                            }
                        }
                        _ => {
                            return Err(Error::new_spanned(
                                attr,
                                "the #[array] attribute must have exactly one or two arguments",
                            ));
                        }
                    }
                } else if attr.path().is_ident("with") {
                    if with.is_some() {
                        return Err(Error::new_spanned(attr, "duplicate #[with] attribute"));
                    }

                    if var {
                        return Err(Error::new_spanned(
                            attr,
                            "cannot have both #[var] and #[with]",
                        ));
                    }

                    match &attr.meta {
                        Meta::List(list) => {
                            let exprs: Punctuated<Expr, Token![,]> =
                                list.parse_args_with(Punctuated::parse_terminated)?;

                            if exprs.len() != 4 {
                                return Err(Error::new_spanned(
                                    attr,
                                    "the #[with] attribute must have exactly four arguments",
                                ));
                            }

                            with = Some((exprs[0].clone(), exprs[1].clone(), exprs[2].clone(), exprs[3].clone()));
                        }
                        _ => {
                            return Err(Error::new_spanned(
                                attr,
                                "the #[with] attribute must have exactly four arguments",
                            ));
                        }
                    }
                }
            }

            Ok((f.ident.as_ref().unwrap(), &f.ty, var, array, with))
        })
        .collect::<Result<Vec<_>>>()
}
