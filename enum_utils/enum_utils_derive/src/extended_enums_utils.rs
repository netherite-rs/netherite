use proc_macro2::TokenStream;
use quote::{quote};
use syn::{DataEnum, DeriveInput};

pub fn derive_ordinal(ast: DeriveInput) -> TokenStream {
    let name = ast.ident;
    let data = ast.data;
    if let syn::Data::Enum(DataEnum {
                               enum_token: _enum_token,
                               brace_token: _brace_token,
                               variants
                           }) = data {
        let mut index = 0_u32;
        let mut from_ordinal_tokens = Vec::new();
        let mut get_ordinal_tokens = Vec::new();
        for x in variants {
            let variant_name = x.ident;
            from_ordinal_tokens.push(quote! {
               #index => Ok(#name::#variant_name),
            });
            get_ordinal_tokens.push(quote! {
                #name::#variant_name => #index,
            });
            index += 1;
        }
        let output = quote! {
            impl enum_utils::OrdinalEnum for #name {
                fn from_ordinal(ordinal: u32) -> Result<Self, String> where Self: Sized {
                    match ordinal {
                        #(#from_ordinal_tokens)*
                        invalid => Err(format!(stringify!(No such #name with ordinal {}), invalid))
                    }
                }

                fn ordinal(&self) -> u32 {
                    match self {
                        #(#get_ordinal_tokens)*
                    }
                }

            }
        };
        return output.into();
    }
    panic!("OrdinalEnum cannot be used on non-enums.")
}

pub fn derive_named(ast: DeriveInput) -> TokenStream {
    let name = ast.ident;
    let data = ast.data;
    if let syn::Data::Enum(DataEnum {
                               enum_token: _enum_token,
                               brace_token: _brace_token,
                               variants
                           }) = data {
        let mut from_name_tokens = Vec::new();
        let mut get_name_tokens = Vec::new();
        for x in variants {
            let variant_name = x.ident;
            from_name_tokens.push(quote! {
               stringify!(#variant_name) => Ok(#name::#variant_name),
            });
            get_name_tokens.push(quote! {
                #name::#variant_name => stringify!(#variant_name),
            });
        }
        let output = quote! {
            impl enum_utils::NamedEnum for #name {
                fn from_name(name: &str) -> Result<Self, String> where Self: Sized {
                    match name {
                        #(#from_name_tokens)*
                        invalid => Err(format!(stringify!(No such #name with name {}), invalid))
                    }
                }

                fn name(&self) -> &'static str {
                    match self {
                        #(#get_name_tokens)*
                    }
                }
            }
        };
        return output.into();
    }
    panic!("NamedEnum cannot be used on non-enums.")
}