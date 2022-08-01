mod clientbound_derive;
mod serverbound_derive;

extern crate quote;
extern crate syn;

extern crate proc_macro;
extern crate core;

use proc_macro::TokenStream;

#[proc_macro_derive(Clientbound)]
pub fn derive_clientbound(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    clientbound_derive::derive_clientbound(ast).into()
}

#[proc_macro_derive(Serverbound)]
pub fn derive_serverbound(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    serverbound_derive::derive_serverbound(ast).into()
}