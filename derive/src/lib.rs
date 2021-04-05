use syn;

mod from;
mod network;

#[proc_macro_derive(Network, attributes(network))]
pub fn network(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    network::network(input)
}

#[proc_macro_derive(From, attributes(from))]
pub fn from(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    from::from(input)
}
