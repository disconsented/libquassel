use quote::quote;
use syn;

use syn::parse_macro_input;

use darling::{FromDeriveInput, FromField, FromMeta, FromVariant};

mod from_network_impl;
mod to_network_impl;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(network), supports(struct_any))]
struct Network {
    ident: syn::Ident,
    attrs: Vec<syn::Attribute>,
    repr: Repr,
}

#[derive(Debug, Clone, Copy, FromMeta)]
#[darling(default)]
enum Repr {
    Aos,
    List,
    Map,
    Maplist,
}

impl Default for Repr {
    fn default() -> Self {
        Repr::List
    }
}

#[derive(Debug, FromField)]
#[darling(attributes(network))]
struct NetworkField {
    ident: Option<syn::Ident>,
    ty: syn::Type,

    #[darling(default)]
    rename: Option<String>,
    #[darling(default)]
    override_type: Option<String>,
    #[darling(default)]
    to_map: Option<String>,
    #[darling(default)]
    from_map: Option<String>,
}

#[proc_macro_derive(Network, attributes(network))]
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
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    };

    // println!("{:#?}", fields);

    let name = &input.ident;

    let to_network_impl_center = match network.repr {
        // Repr::Aos => {}
        Repr::Map => to_network_impl::map(&fields),
        Repr::Maplist => to_network_impl::map_list(&fields),
        Repr::List => to_network_impl::list(&fields),
        _ => unimplemented!(),
    };

    let from_network_impl_center = match network.repr {
        // Repr::Aos => {}
        Repr::Map => from_network_impl::map(&fields),
        Repr::Maplist => from_network_impl::map_list(&fields),
        Repr::List => from_network_impl::list(&fields),
        _ => unimplemented!(),
    };

    let network_impl_item = match network.repr {
        Repr::Aos => quote! {crate::primitive::VariantList;},
        Repr::Map => quote! {crate::primitive::VariantMap;},
        Repr::Maplist => quote! {crate::primitive::VariantMap;},
        Repr::List => quote! {crate::primitive::VariantList;},
    };

    // do things with `args`
    let gen = quote! {
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

    println!("{}", gen);

    gen.into()
}

fn get_field_type(field: &NetworkField) -> syn::Type {
    match &field.override_type {
        Some(override_type) => syn::Type::from(syn::TypePath {
            qself: None,
            path: syn::Path {
                leading_colon: None,
                segments: {
                    let mut res =
                        syn::punctuated::Punctuated::<syn::PathSegment, syn::token::Colon2>::new();

                    res.push(syn::PathSegment {
                        ident: syn::Ident::new(override_type, proc_macro2::Span::call_site()),
                        arguments: syn::PathArguments::None,
                    });

                    res
                },
            },
        }),
        None => field.ty.clone(),
    }
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(from), supports(enum_any))]
struct Enum {
    ident: syn::Ident,
    attrs: Vec<syn::Attribute>,
}

#[derive(Debug, FromVariant)]
#[darling(attributes(from))]
struct EnumField {
    ident: syn::Ident,
    fields: darling::ast::Fields<syn::Type>,

    #[darling(default)]
    ignore: bool,
}

#[proc_macro_derive(From, attributes(from))]
pub fn from(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    // println!("{:#?}", input);

    let network = Enum::from_derive_input(&input).unwrap();
    // println!("{:#?}", network);

    let enum_name = network.ident;

    let fields: Vec<EnumField> = match &input.data {
        syn::Data::Enum(data) => data
            .variants
            .iter()
            .map(|field| EnumField::from_variant(field).expect("Could not parse field"))
            .collect(),
        _ => unimplemented!(),
    };

    let derives = fields
        .iter()
        .filter(|field| field.fields.fields.len() > 0 && !field.ignore)
        .map(|field| {
            let variant = &field.ident;
            let inner_type = &field.fields.fields[0];

            quote! {
                impl From<#inner_type> for #enum_name {
                    fn from(input: #inner_type) -> Self {
                        Self::#variant(input)
                    }
                }

                impl Into<#inner_type> for #enum_name {
                    fn into(self) -> #inner_type {
                        match self {
                            Self::#variant(input) => input,
                            _ => unimplemented!(),
                        }
                    }
                }
            }
        });

    // println!("{:#?}", fields);

    let gen = quote! {
        #(#derives)*
    };

    // println!("{}", gen);

    gen.into()
}
