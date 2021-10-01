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
    /// Representation to choose for the network format
    /// see Repr enum
    #[darling(default)]
    repr: Repr,
}

/// List:
/// Map:
/// Maplist:
#[derive(Debug, Clone, Copy, PartialEq, FromMeta)]
pub enum Repr {
    List,
    Map,
    Maplist,
}

impl Default for Repr {
    fn default() -> Self {
        Repr::Map
    }
}

#[derive(Debug, FromField)]
#[darling(attributes(network))]
pub struct NetworkField {
    ident: Option<syn::Ident>,
    ty: syn::Type,

    #[darling(default)]
    rename: Option<String>,
    #[darling(default)]
    override_type: Option<String>,
    #[darling(default, rename = "type")]
    typ: Option<String>,
    /// Variant to encapsulate this field
    /// VariantList (default) or StringList
    #[darling(default)]
    variant: Option<String>,
    /// field is a nested network repr so
    /// use to_network and from_network on it
    #[darling(default)]
    network: bool,
    /// When network is true, use map
    /// network representation for this field
    #[darling(default)]
    map: bool,
}

fn parse_fields(input: &syn::DeriveInput) -> Vec<NetworkField> {
    match &input.data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Named(fields) => fields
                .named
                .iter()
                .map(|field| NetworkField::from_field(field).expect("Could not parse field"))
                .collect(),
            _ => panic!("network: not a named field"),
        },
        _ => panic!("network: not a Struct"),
    }
}

pub fn network_map(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    let network = Network::from_derive_input(&input).unwrap();

    let fields = parse_fields(&input);

    let name = &input.ident;

    let to_network_map = match network.repr {
        Repr::Maplist => maplist::to(&fields),
        Repr::Map | _ => map::to(&fields),
    };

    let from_network_map = match network.repr {
        Repr::Maplist => maplist::from(&fields),
        Repr::Map | _ => map::from(&fields),
    };

    let mut gen = quote! {
        impl libquassel::message::signalproxy::NetworkMap for #name {
            type Item = libquassel::primitive::VariantMap;

            fn to_network_map(&self) -> libquassel::primitive::VariantMap {
                let mut res = libquassel::primitive::VariantMap::new();

                #(#to_network_map)*

                return res;
            }

            fn from_network_map(input: &mut libquassel::primitive::VariantMap) -> Self {
                Self {
                    #(#from_network_map)*
                }
            }
        }
    };

    let network_map_vec_item = match network.repr {
        Repr::Map => quote! {libquassel::primitive::VariantList},
        Repr::Maplist => quote! {libquassel::primitive::VariantMap},
        Repr::List => quote! {libquassel::primitive::VariantList},
    };

    let to_network_map_vec = match network.repr {
        Repr::Maplist => maplist::to_vec(name, &fields),
        Repr::Map => map::to_vec(name, &fields, true),
        _ => unimplemented!(),
    };

    let from_network_map_vec = match network.repr {
        Repr::Maplist => maplist::from_vec(name, &fields, true),
        Repr::Map => map::from_vec(name, &fields, true),
        _ => unimplemented!(),
    };

    let list_map = quote! {
        impl libquassel::message::signalproxy::NetworkMap for Vec<#name> {
            type Item = #network_map_vec_item;

            fn to_network_map(&self) -> Self::Item {
                #to_network_map_vec
            }

            fn from_network_map(input: &mut Self::Item) -> Self {
                #from_network_map_vec
            }
        }
    };

    gen.extend(list_map);

    gen.into()
}

pub fn network_list(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    let _network = Network::from_derive_input(&input).unwrap();

    let fields = parse_fields(&input);

    let name = &input.ident;

    let to_network_list = list::to(&fields);
    let from_network_list = list::from(&fields);

    let gen = quote! {
        impl libquassel::message::signalproxy::NetworkList for #name {
            fn to_network_list(&self) -> libquassel::primitive::VariantList {
                let mut res = libquassel::primitive::VariantList::new();

                #(#to_network_list)*

                return res;
            }

            fn from_network_list(input: &mut libquassel::primitive::VariantList) -> Self {
                Self {
                    #(#from_network_list)*
                }
            }
        }
    };

    gen.into()
}

pub fn network(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    // println!("{:#?}", input);

    let network = Network::from_derive_input(&input).unwrap();
    // println!("{:#?}", network);

    let fields = parse_fields(&input);

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
        Repr::Map => quote! {libquassel::primitive::VariantMap;},
        Repr::Maplist => quote! {libquassel::primitive::VariantMap;},
        Repr::List => quote! {libquassel::primitive::VariantList;},
    };

    let mut gen = quote! {
        impl libquassel::message::signalproxy::Network for #name {
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
            Repr::Map => quote! {libquassel::primitive::VariantList;},
            Repr::Maplist => quote! {libquassel::primitive::VariantMap;},
            Repr::List => quote! {libquassel::primitive::VariantList;},
        };

        let to_network_impl_vec_center = match network.repr {
            Repr::Maplist => maplist::to_vec(name, &fields),
            Repr::Map => map::to_vec(name, &fields, false),
            _ => unimplemented!(),
        };

        let from_network_impl_vec_center = match network.repr {
            Repr::Maplist => maplist::from_vec(name, &fields, false),
            Repr::Map => map::from_vec(name, &fields, false),
            _ => unimplemented!(),
        };

        let vec = quote! {
            impl libquassel::message::signalproxy::Network for Vec<#name> {
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
    } else if let Some(typ) = &field.typ {
        gen_type(typ)
    } else {
        field.ty.clone()
    }
}

fn get_field_variant_type(field: &NetworkField) -> syn::Type {
    match &field.variant {
        Some(ty) => gen_type(&ty),
        None => get_field_type(field),
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
