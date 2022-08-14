mod extended_enum_derive;

use proc_macro::TokenStream;

#[proc_macro_derive(OrdinalEnum)]
pub fn derive_ordinal(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    extended_enum_derive::derive_ordinal(ast).into()
}

#[proc_macro_derive(NamedEnum)]
pub fn derive_named(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    extended_enum_derive::derive_named(ast).into()
}
