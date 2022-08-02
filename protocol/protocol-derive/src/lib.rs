extern crate core;
extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use crate::options::PacketInfo;
use darling::FromDeriveInput;

mod clientbound_derive;
mod serverbound_derive;
mod options;


#[proc_macro_derive(Clientbound, attributes(packet))]
pub fn derive_clientbound(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    let opts = PacketInfo::from_derive_input(&ast).unwrap();
    clientbound_derive::derive_clientbound(ast, opts).into()
}

#[proc_macro_derive(Serverbound, attributes(packet))]
pub fn derive_serverbound(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    let opts = PacketInfo::from_derive_input(&ast).unwrap();
    serverbound_derive::derive_serverbound(ast, opts).into()
}