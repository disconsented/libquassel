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

            if field.to_map.is_some() {
                let field_map: syn::ExprClosure =
                    syn::parse_str(&field.to_map.as_ref().unwrap()).unwrap();

                quote! {
                    res.insert(#field_rename.to_string(),
                        crate::primitive::Variant::#field_type(self.#field_name.iter().map(#field_map).collect()));
                }
            } else {
                quote! {
                    res.insert(#field_rename.to_string(),
                        crate::primitive::Variant::#field_type(self.#field_name.clone()));
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

            if field.to_map.is_some() {
                let field_map: syn::ExprClosure =
                    syn::parse_str(&field.to_map.as_ref().unwrap()).unwrap();

                quote! {
                    res.insert(#field_rename.to_string(),
                        crate::primitive::Variant::VariantList(
                            std::vec::from_elem(crate::primitive::Variant::#field_type(self.#field_name.iter().map(#field_map).collect()), 1)));
                }
            } else {
                quote! {
                    res.insert(#field_rename.to_string(),
                        crate::primitive::Variant::VariantList(
                            std::vec::from_elem(crate::primitive::Variant::#field_type(self.#field_name.clone()), 1)));
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

            if field.to_map.is_some() {
                let field_map: syn::ExprClosure =
                    syn::parse_str(&field.to_map.as_ref().unwrap()).unwrap();

                quote! {
                    res.push(crate::primitive::Variant::ByteArray(#field_rename.to_string()));
                    res.push(crate::primitive::Variant::#field_type(self.#field_name.iter().map(#field_map).collect()));
                }
            } else {
                quote! {
                    res.push(crate::primitive::Variant::ByteArray(#field_rename.to_string()));
                    res.push(crate::primitive::Variant::#field_type(self.#field_name.clone()));
                }
            }
        })
        .collect()
}
