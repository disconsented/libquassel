use syn;

mod from;
mod network;
mod sync;

#[proc_macro_derive(NetworkList, attributes(network))]
pub fn network_list(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    network::network_list(input)
}

#[proc_macro_derive(NetworkMap, attributes(network))]
pub fn network_map(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    network::network_map(input)
}

#[proc_macro_derive(From, attributes(from))]
pub fn from(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    from::from(input)
}

#[proc_macro]
pub fn sync(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    sync::sync(input)
}
