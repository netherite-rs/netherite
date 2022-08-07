extern crate proc_macro;

use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::DataStruct;
use crate::PacketInfo;

pub fn derive_serverbound(ast: syn::DeriveInput, options: PacketInfo) -> TokenStream {
    let id = options.id.expect("Serverbound packet must have an ID. Use #[packet(id=XXX)]");
    let name = ast.ident;
    let data = ast.data;
    let mut t: Vec<TokenStream> = vec![];
    if let syn::Data::Struct(DataStruct { struct_token: _struct_token, fields, semi_token: _semi_token }) = data {
        for x in fields {
            let field_name = x.ident.unwrap();
            match x.ty.into_token_stream().to_string().as_str() {
                "VarInt" => t.push(m_name("varint", &field_name).into()),
                "VarLong" => t.push(m_name("varlong", &field_name).into()),
                "String" => t.push(m_name("utf8", &field_name).into()),
                "Blob" => t.push(m_name("blob", &field_name).into()),
                "bool" => t.push(m_name("bool", &field_name).into()),
                type_name => {
                    match type_name {
                        "u8" | "i8" => t.push(read_bytes_ext(type_name, &field_name).into()),
                        "u16" | "u32" | "u64" | "u128" | "i16" | "i32" | "i64" | "i128" => {
                            t.push(big_endian(type_name, &field_name).into())
                        }
                        "bool" => t.push(m_name("bool", &field_name).into()),
                        packet_field => {
                            t.push(read_name(packet_field, &field_name).into())
                        }
                    }
                }
            }
        }
        let output = quote! {
            impl protocol::bound::Serverbound for #name {
                fn read_packet(input: &mut impl std::io::Read) -> #name {
                    #name {
                        #(#t)*
                    }
                }

                fn id(&self) -> i32 {
                    #id
                }
            }
        };
        return output.into();
    }
    panic!("Failed to expand Serverbound macro");
}

fn m_name(method: &str, field_name: &Ident) -> TokenStream {
    let t: TokenStream = format!("read_{}", method).as_str().parse::<TokenStream>().unwrap();
    return quote! {
        #field_name: protocol::packet_io::PacketReaderExt::#t(input).expect(stringify!(failed to read #field_name)),
    };
}

fn big_endian(method: &str, field_name: &Ident) -> TokenStream {
    let t: TokenStream = format!("read_{}", method).as_str().parse::<TokenStream>().unwrap();
    return quote! {
        #field_name: byteorder::ReadBytesExt::#t::<byteorder::BigEndian>(input).expect(stringify!(failed to read #field_name)),
    };
}

fn read_bytes_ext(method: &str, field_name: &Ident) -> TokenStream {
    let t: TokenStream = format!("read_{}", method).as_str().parse::<TokenStream>().unwrap();
    return quote! {
        #field_name: byteorder::ReadBytesExt::#t(input).expect(stringify!(failed to read #field_name)),
    };
}

fn read_name(type_name: &str, field_name: &Ident) -> TokenStream {
    let t = type_name.parse::<TokenStream>().unwrap();
    return quote! {
        #field_name: protocol::packet_io::PacketReaderExt::read_field::<#t>(input).expect(stringify!(failed to read #field_name)),
    };
}