use quote::quote;
use syn::{self, parse_macro_input};

use darling::{FromDeriveInput, FromField, FromMeta};

mod list;
mod map;
mod maplist;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(network), supports(struct_any))]
/// Derive to and from network methods for quassel objects
pub struct Network {
    ident: syn::Ident,
    attrs: Vec<syn::Attribute>,
    /// Representation to choose for the network format
    /// see Repr enum
    repr: Repr,
}

#[derive(Debug, Clone, Copy, FromMeta)]
#[darling(default)]
pub enum Repr {
    List,
    Map,
    Maplist,
}

impl Default for Repr {
    fn default() -> Self {
        Repr::List
    }
}

// #[derive(Debug, Clone, Copy, FromMeta)]
// #[darling(default)]
// pub enum Variant {
//     Variantlist,
//     Stringlist,
//     Variantmap,
//     Default,
// }

// impl Default for Variant {
//     fn default() -> Self {
//         Self::Default
//     }
// }

#[derive(Debug, FromField)]
#[darling(attributes(network))]
pub struct NetworkField {
    ident: Option<syn::Ident>,
    ty: syn::Type,

    #[darling(default)]
    rename: Option<String>,
    #[darling(default)]
    override_type: Option<String>,
    /// Variant to encapsulate this field
    /// VariantList (default) or StringList
    #[darling(default)]
    variant: Option<String>,
    /// field is a nested network repr so
    /// use to_network and from_network on it
    #[darling(default)]
    network: bool,
}

pub fn network(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    // println!("{:#?}", input);

    let network = Network::from_derive_input(&input).unwrap();
    // println!("{:#?}", network);

    let fields: Vec<NetworkField> = match &input.data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Named(fields) => fields
                .named
                .iter()
                .map(|field| NetworkField::from_field(field).expect("Could not parse field"))
                .collect(),
            _ => panic!("network: not a named field"),
        },
        _ => panic!("network: not a Struct"),
    };

    // println!("{:#?}", fields);

    let name = &input.ident;

    let to_network_impl_center = match network.repr {
        // Repr::Aos => {}
        Repr::Map => map::to(&fields),
        Repr::Maplist => maplist::to(&fields),
        Repr::List => list::to(&fields),
    };

    let from_network_impl_center = match network.repr {
        // Repr::Aos => {}
        Repr::Map => map::from(&fields),
        Repr::Maplist => maplist::from(&fields),
        Repr::List => list::from(&fields),
    };

    let network_impl_item = match network.repr {
        Repr::Map => quote! {crate::primitive::VariantMap;},
        Repr::Maplist => quote! {crate::primitive::VariantMap;},
        Repr::List => quote! {crate::primitive::VariantList;},
    };

    let mut gen = quote! {
        impl crate::message::signalproxy::Network for #name {
            type Item = #network_impl_item

            fn to_network(&self) -> Self::Item {
                let mut res = Self::Item::new();

                #(#to_network_impl_center)*

                return res;
            }

            fn from_network(input: &mut Self::Item) -> Self {
                Self {
                    #(#from_network_impl_center)*
                }
            }
        }
    };

    if let Repr::Maplist | Repr::Map = network.repr {
        let network_impl_item_vec = match network.repr {
            Repr::Map => quote! {crate::primitive::VariantList;},
            Repr::Maplist => quote! {crate::primitive::VariantMap;},
            Repr::List => quote! {crate::primitive::VariantList;},
        };

        let to_network_impl_vec_center = match network.repr {
            Repr::Maplist => maplist::to_vec(name, &fields),
            Repr::Map => map::to_vec(name, &fields),
            _ => unimplemented!(),
        };

        let from_network_impl_vec_center = match network.repr {
            Repr::Maplist => maplist::from_vec(name, &fields),
            Repr::Map => map::from_vec(name, &fields),
            _ => unimplemented!(),
        };

        let vec = quote! {
            impl crate::message::signalproxy::Network for Vec<#name> {
                type Item = #network_impl_item_vec

                fn to_network(&self) -> Self::Item {
                    #to_network_impl_vec_center
                }

                fn from_network(input: &mut Self::Item) -> Self {
                    #from_network_impl_vec_center
                }
            }
        };

        gen.extend(vec);
    }

    // println!("{}", gen);

    gen.into()
}

fn get_field_type(field: &NetworkField) -> syn::Type {
    if let Some(override_type) = &field.override_type {
        gen_type(override_type)
    } else {
        field.ty.clone()
    }
}

fn get_field_variant_type(field: &NetworkField) -> syn::Type {
    match &field.variant {
        Some(ty) => gen_type(&ty),
        None => {
            if let Some(override_type) = &field.override_type {
                gen_type(override_type)
            } else {
                field.ty.clone()
            }
        }
    }
}

fn gen_type(typ: &str) -> syn::Type {
    syn::Type::from(syn::TypePath {
        qself: None,
        path: syn::Path {
            leading_colon: None,
            segments: {
                let mut res =
                    syn::punctuated::Punctuated::<syn::PathSegment, syn::token::Colon2>::new();

                res.push(syn::PathSegment {
                    ident: syn::Ident::new(typ, proc_macro2::Span::call_site()),
                    arguments: syn::PathArguments::None,
                });

                res
            },
        },
    })
}

fn get_field_type_colon(mut ty: syn::Type) -> syn::Type {
    match &mut ty {
        syn::Type::Path(path) => {
            let first_seg = path.path.segments.first_mut().unwrap();
            match &mut first_seg.arguments {
                syn::PathArguments::AngleBracketed(bracket) => {
                    bracket.colon2_token = Some(syn::parse_str("::").unwrap());
                }
                _ => (),
            }
        }
        _ => (),
    };

    return ty;
}
