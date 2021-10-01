use proc_macro2::{Ident, TokenStream};
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
            let _field_type = get_field_type(&field);
            let field_variant_type = get_field_variant_type(&field);

            let field_inner = if field.network {
                if field.map {
                    quote! {
                        self.#field_name.to_network_map()
                    }
                } else {
                    match field.variant.as_ref().map_or("", |m| m.as_str()) {
                        "VariantMap" => quote! {
                            self.#field_name.to_network_map()
                        },
                        &_ => quote! {
                            self.#field_name.to_network()
                        },
                    }
                }
            } else {
                quote! {
                    self.#field_name.clone()
                }
            };

            quote! {
                res.insert(#field_rename.to_string(),
                    libquassel::primitive::Variant::#field_variant_type(#field_inner));
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
            let _field_variant_type = get_field_variant_type(&field);

            let field_type_colon = get_field_type_colon(field_type.clone());

            if field.network {
                if field.map {
                    quote! {
                        #field_name: #field_type_colon::from_network_map(
                            &mut std::convert::TryInto::try_into(input.remove(#field_rename).unwrap()).unwrap()),
                    }
                } else {
                    match field.variant.as_ref().map_or("", |m| m.as_str()) {
                        "VariantMap" => quote! {
                                    #field_name: #field_type_colon::from_network_map(
                                        &mut std::convert::TryInto::try_into(input.remove(#field_rename).unwrap()).unwrap()),
                                },

                        &_ => quote! {
                            #field_name: #field_type_colon::from_network(
                                &mut std::convert::TryInto::try_into(input.remove(#field_rename).unwrap()).unwrap()),
                        }
                    }
                }
            } else {
                quote! {
                    #field_name: std::convert::TryInto::try_into(input.remove(#field_rename).unwrap()).unwrap(),
                }
            }
        })
        .collect()
}

pub(crate) fn to_vec(_type_name: &Ident, _fields: &Vec<NetworkField>, new: bool) -> TokenStream {
    if new {
        quote! {
            self.iter().map(|item| {
                item.to_network_map().into()
            }).collect()
        }
    } else {
        quote! {
            self.iter().map(|item| {
                item.to_network().into()
            }).collect()
        }
    }
}

pub(crate) fn from_vec(type_name: &Ident, _fields: &Vec<NetworkField>, new: bool) -> TokenStream {
    if new {
        quote! {
            input.iter().map(
                |item| #type_name::from_network_map(
                    &mut std::convert::TryInto::try_into(item).unwrap()
                )).collect()
        }
    } else {
        quote! {
            input.iter().map(
                |item| #type_name::from_network(
                    &mut std::convert::TryInto::try_into(item).unwrap()
                )).collect()
        }
    }
}
