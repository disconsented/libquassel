use proc_macro2::{Ident, TokenStream};
use quote::quote;

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
            let field_type = get_field_type(&field);

            match &field.variant {
                Some(variant_type) => match variant_type.as_str() {
                    "StringList" => quote! {
                        res.insert(#field_rename.to_string(),
                            crate::primitive::Variant::StringList(
                                std::vec::from_elem(self.#field_name.clone(), 1)));
                    },
                    _ => panic!("network::map::to: not one of the avaible variants")
                }
                None => quote! {
                    res.insert(#field_rename.to_string(),
                        crate::primitive::Variant::VariantList(
                            std::vec::from_elem(crate::primitive::Variant::#field_type(self.#field_name.clone()), 1)));
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
            let field_type = get_field_type(&field);

            match &field.variant {
                None => {
                    lists.push(quote! {
                        let mut #field_name: crate::primitive::VariantList = Vec::with_capacity(self.len());
                    });

                    for_each_inner.push(quote! {
                        #field_name.push(crate::primitive::Variant::#field_type(item.#field_name.clone()));
                    });

                    map_inserts.push(quote! {
                        map.insert(String::from(#field_rename), crate::primitive::Variant::VariantList(#field_name));
                    });
                }
                Some(variant_type) => match variant_type.as_str() {
                    "StringList" => {
                        lists.push(quote! {
                            let mut #field_name: crate::primitive::StringList = Vec::with_capacity(self.len());
                        });

                        for_each_inner.push(quote! {
                            #field_name.push(item.#field_name.clone());
                        });

                        map_inserts.push(quote! {
                            map.insert(String::from(#field_rename), crate::primitive::Variant::StringList(#field_name));
                        });
                    }
                    _ => panic!("network::map::to: not one of the avaible variants")
                }
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

            let _field_type = get_field_type(&field);

            match &field.variant {
                None => quote! {
                    #field_name: match input.get_mut(#field_rename).unwrap() {
                        crate::primitive::Variant::VariantList(input) => std::convert::TryInto::try_into(input.remove(0)).unwrap(),
                        _ => panic!("#field_name: wrong variant")
                    },
                },
                Some(variant_type) => match variant_type.as_str() {
                    "StringList" => quote! {
                        #field_name: match input.get_mut(#field_rename).unwrap() {
                            crate::primitive::Variant::StringList(input) => input.remove(0),
                            _ => panic!("#field_name: wrong variant")
                        },
                    },
                    _ => panic!("network::map::to: not one of the avaible variants"),
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

    let field_variant = match &field.variant {
        None => quote! {crate::primitive::VariantList},
        Some(variant_type) => match variant_type.as_str() {
            "StringList" => quote! {crate::primitive::StringList},
            "VariantMap" => quote! {crate::primitive::VariantMap},
            _ => panic!("network::map::from_vec: not one of the avaible variants"),
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
