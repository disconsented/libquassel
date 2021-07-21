use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::network::get_field_variant_type;

use super::{get_field_type, NetworkField};

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
                quote! {
                    self.#field_name.to_network().into()
                }
            } else {
                quote! {
                    self.#field_name.clone().into()
                }
            };

            if let Some(_) = field.variant {
                quote! {
                    res.insert(#field_rename.to_string(),
                        crate::primitive::Variant::#field_type(
                            std::vec::from_elem(#field_inner, 1)));
                }
            } else {
                quote! {
                    res.insert(#field_rename.to_string(),
                        crate::primitive::Variant::VariantList(
                            std::vec::from_elem(#field_inner, 1)));
                }
            }
        })
        .collect()
}

pub(crate) fn to_vec(_type_name: &Ident, fields: &Vec<NetworkField>) -> TokenStream {
    let (lists, for_each_inner, map_inserts): (
        Vec<TokenStream>,
        Vec<TokenStream>,
        Vec<TokenStream>,
    ) = fields.iter().fold(
        (Vec::new(), Vec::new(), Vec::new()),
        |(mut lists, mut for_each_inner, mut map_inserts), field| {
            let field_rename = match &field.rename {
                Some(name) => name.clone(),
                None => format!("{}", field.ident.as_ref().unwrap()).into(),
            };

            let field_name = field.ident.as_ref().unwrap();
            let field_type = get_field_variant_type(&field);

            let field_inner = if field.network {
                quote! {
                    item.#field_name.to_network()
                }
            } else {
                quote! {
                    item.#field_name.clone()
                }
            };

            if let Some(_) = field.variant {
                lists.push(quote! {
                    let mut #field_name: crate::primitive::StringList = Vec::with_capacity(self.len());
                });

                for_each_inner.push(quote! {
                    #field_name.push(#field_inner);
                });

                map_inserts.push(quote! {
                    map.insert(String::from(#field_rename), crate::primitive::Variant::StringList(#field_name));
                });
            } else {
                lists.push(quote! {
                    let mut #field_name: crate::primitive::VariantList = Vec::with_capacity(self.len());
                });

                for_each_inner.push(quote! {
                    #field_name.push(crate::primitive::Variant::#field_type(#field_inner));
                });

                map_inserts.push(quote! {
                    map.insert(String::from(#field_rename), crate::primitive::Variant::VariantList(#field_name));
                });
            }

            return (lists, for_each_inner, map_inserts);
        },
    );

    quote! {
        #(#lists)*

        let mut map = crate::primitive::VariantMap::new();

        self.iter().for_each(|item| {
            #(#for_each_inner)*
        });

        #(#map_inserts)*

        return map;
    }
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

            let field_type = get_field_variant_type(&field);

            let field_inner = if field.network {
                quote! {
                    crate::message::Network::from_network(&mut std::convert::TryInto::try_into(input.remove(0)).unwrap())
                }
            } else {
                quote! {
                    std::convert::TryInto::try_into(input.remove(0)).unwrap()
                }
            };

            if let Some(_) = field.variant {
                quote! {
                    #field_name: match input.get_mut(#field_rename).unwrap() {
                        crate::primitive::Variant::#field_type(input) => #field_inner,
                        _ => panic!("#field_name: wrong variant")
                    },
                }
            } else {
                quote! {
                    #field_name: match input.get_mut(#field_rename).unwrap() {
                        crate::primitive::Variant::VariantList(input) => #field_inner,
                        _ => panic!("#field_name: wrong variant")
                    },
                }
            }
        })
        .collect()
}

pub(crate) fn from_vec(type_name: &Ident, fields: &Vec<NetworkField>) -> TokenStream {
    let field = &fields[0];

    let field_rename = match &field.rename {
        Some(name) => name.clone(),
        None => format!("{}", field.ident.as_ref().unwrap()).into(),
    };

    let _field_name = field.ident.as_ref().unwrap();

    let field_type = get_field_variant_type(field);

    let field_variant = match &field.variant {
        None => quote! {crate::primitive::VariantList},
        Some(variant_type) => match variant_type.as_str() {
            "StringList" => quote! {crate::primitive::StringList},
            "VariantMap" => quote! {crate::primitive::VariantMap},
            _ => quote! {crate::primitive::VariantMap},
        },
    };

    quote! {
        let marker: #field_variant = std::convert::TryInto::try_into(input.get(#field_rename).unwrap()).unwrap();

        let mut res = Vec::new();
        for _ in 0..marker.len() {
            res.push(#type_name::from_network(input));
        }

        return res;
    }
}
