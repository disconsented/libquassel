use proc_macro2::TokenStream;
use quote::quote;

use super::{get_field_type, get_field_type_colon, get_field_variant_type, NetworkField};

pub(crate) fn to(fields: &Vec<NetworkField>) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|field| {
            let field_rename = match &field.rename {
                Some(name) => name.clone(),
                None => format!("{}", field.ident.as_ref().unwrap()).into(),
            };

            let field_name = field.ident.as_ref().unwrap();
            let field_type = get_field_variant_type(&field);

            let field_inner = if field.network {
                quote! { self.#field_name.to_network() }
            } else {
                quote! { self.#field_name.clone() }
            };

            quote! {
                res.push(crate::primitive::Variant::ByteArray(#field_rename.to_string()));
                res.push(crate::primitive::Variant::#field_type(#field_inner));
            }
        })
        .collect()
}

pub(crate) fn from(fields: &Vec<NetworkField>) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|field| {
            let field_rename = match &field.rename {
                Some(name) => name.clone(),
                None => format!("{}", field.ident.as_ref().unwrap()).into(),
            };

            let field_name = field.ident.as_ref().unwrap();

            let field_type = get_field_type(&field);
            let field_variant_type = get_field_variant_type(&field);

            let field_type_colon = get_field_type_colon(field_type.clone());

            let extract_inner = quote! {
                let mut i = input.iter();
                i.position(|x| *x == crate::primitive::Variant::ByteArray(String::from(#field_rename)))
                    .expect(format!("failed to get field {}", #field_rename).as_str());

                match i.next().expect("failed to get next field") {
                    crate::primitive::Variant::#field_variant_type(var) => var.clone(),
                    _ => panic!("network::list::from: wrong variant type"),
                }
            };

            if field.network {
                quote! {
                    #field_name: #field_type_colon::from_network(&mut {
                        #extract_inner
                    }),
                }
            } else {
                quote! {
                    #field_name: {
                        #extract_inner
                    },
                }
            }
        })
        .collect()
}
