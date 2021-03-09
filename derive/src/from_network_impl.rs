use proc_macro2::TokenStream;
use quote::quote;

use crate::{get_field_type, NetworkField};

pub(crate) fn map(fields: &Vec<NetworkField>) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|field| {
            let field_rename = match &field.rename {
                Some(name) => name.clone(),
                None => format!("{}", field.ident.as_ref().unwrap()).into(),
            };

            let field_name = field.ident.as_ref().unwrap();

            let field_type = get_field_type(&field);

            if field.from_map.is_some() {
                let field_map: syn::ExprClosure =
                    syn::parse_str(&field.from_map.as_ref().unwrap()).unwrap();

                quote! {
                    #field_name: match &input.get(#field_rename).unwrap() {
                            crate::primitive::Variant::#field_type(input) => input.iter().map(#field_map).collect(),
                            _ => unimplemented!()
                        },
                }
            } else {
                quote! {
                    #field_name: match &input.get(#field_rename).unwrap() {
                            crate::primitive::Variant::#field_type(input) => input.clone(),
                            _ => unimplemented!()
                        },
                }
            }
        })
        .collect()
}

pub(crate) fn map_list(fields: &Vec<NetworkField>) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|field| {
            let field_rename = match &field.rename {
                Some(name) => name.clone(),
                None => format!("{}", field.ident.as_ref().unwrap()).into(),
            };

            let field_name = field.ident.as_ref().unwrap();

            let field_type = get_field_type(&field);

            if field.from_map.is_some() {
                let field_map: syn::ExprClosure =
                    syn::parse_str(&field.from_map.as_ref().unwrap()).unwrap();

                quote! {
                    #field_name: match input.get_mut(#field_rename).unwrap() {
                        crate::primitive::Variant::VariantList(input) => match &input.remove(0) {
                            crate::primitive::Variant::#field_type(input) => input.iter().map(#field_map).collect(),
                            _ => unimplemented!()
                        },
                        _ => unimplemented!()
                    },
                }
            } else {
                quote! {
                    #field_name: match input.get_mut(#field_rename).unwrap() {
                        crate::primitive::Variant::VariantList(input) => match &input.remove(0) {
                            crate::primitive::Variant::#field_type(input) => input.clone(),
                            _ => unimplemented!()
                        },
                        _ => unimplemented!()
                    },
                }
            }
        })
        .collect()
}

pub(crate) fn list(fields: &Vec<NetworkField>) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|field| {
            let field_rename = match &field.rename {
                Some(name) => name.clone(),
                None => format!("{}", field.ident.as_ref().unwrap()).into(),
            };

            let field_name = field.ident.as_ref().unwrap();

            let field_type = get_field_type(&field);

            if field.from_map.is_some() {
                let field_map: syn::ExprClosure =
                    syn::parse_str(&field.from_map.as_ref().unwrap()).unwrap();

                quote! {
                    #field_name: match_variant!(
                        input
                            .iter()
                            .nth(input
                                 .iter()
                                 .position(|x| *x == crate::primitive::Variant::ByteArray(#field_rename.to_string()))
                                 .unwrap())
                            .unwrap(),
                        crate::primitive::Variant::#field_type
                    ).iter().map(#field_map).collect(),
                }
            } else {
                quote! {
                    #field_name: match_variant!(
                        input
                            .iter()
                            .nth(input
                                 .iter()
                                 .position(|x| *x == Variant::ByteArray(#field_rename.to_string()))
                                 .unwrap() + 1)
                            .unwrap(),
                        crate::primitive::Variant::#field_type
                    ),
                }
            }
        })
        .collect()
}
