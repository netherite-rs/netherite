extern crate proc_macro;

use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::DataStruct;

use crate::PacketInfo;

pub fn derive_clientbound(ast: syn::DeriveInput, options: PacketInfo) -> TokenStream {
    let id = options.id.expect("Clientbound packet must have an ID. Use #[packet(id=XXX)]");
    let name = ast.ident;
    let data = ast.data;
    let mut t: Vec<TokenStream> = vec![];
    if let syn::Data::Struct(DataStruct { struct_token: _struct_token, fields, semi_token: _semi_token }) = data {
        for x in fields {
            let field_name = x.ident.unwrap();
            match x.ty.into_token_stream().to_string().as_str() {
                "VarInt" => t.push(m_name("varint", &field_name)),
                "VarLong" => t.push(m_name("varlong", &field_name)),
                "String" => t.push(m_name("utf8", &field_name)),
                "Blob" => t.push(m_name("blob", &field_name)),
                "bool" => {
                    t.push(quote! {protocol::packet_io::PacketWriterExt::write_bool(output, self.#field_name)?;}.into())
                }
                type_name => {
                    match type_name {
                        "u8" | "i8" => t.push(write_bytes_ext(type_name, &field_name).into()),
                        "u16" | "u32" | "u64" | "u128" | "i16" | "i32" | "i64" | "i128" => {
                            t.push(big_endian(type_name, &field_name).into())
                        }
                        _ => {
                            t.push(write_name(&field_name).into())
                        }
                    }
                }
            }
        }
        let output = quote! {
            impl protocol::bound::Clientbound for #name {
                fn write_packet(&self, output: &mut impl std::io::Write) -> std::io::Result<()> {
                    #(#t)*
                    Ok(())
                }

                fn id(&self) -> i32 {
                    #id
                }
            }
        };
        return output.into();
    }
    panic!("Clientbound trait can only be implemented on flat structs.");
}

fn m_name(method: &str, field_name: &Ident) -> TokenStream {
    let t: TokenStream = format!("write_{}", method).as_str().parse::<TokenStream>().unwrap();
    return quote! {
        protocol::packet_io::PacketWriterExt::#t(output, &self.#field_name)?;
    };
}

fn big_endian(method: &str, field_name: &Ident) -> TokenStream {
    let t: TokenStream = format!("write_{}", method).as_str().parse::<TokenStream>().unwrap();
    return quote! {
        byteorder::WriteBytesExt::#t::<byteorder::BigEndian>(output, self.#field_name)?;
    };
}

fn write_bytes_ext(method: &str, field_name: &Ident) -> TokenStream {
    let t: TokenStream = format!("write_{}", method).as_str().parse::<TokenStream>().unwrap();
    return quote! {
        byteorder::WriteBytesExt::#t(output, self.#field_name)?;
    };
}

fn write_name(field_name: &Ident) -> TokenStream {
    return quote! {
        output.write_field(&self.#field_name)?;
    };
}