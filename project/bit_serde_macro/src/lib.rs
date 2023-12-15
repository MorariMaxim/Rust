use std::default;
use std::num::IntErrorKind;
use std::sync::Arc;

use bit_serde_trait::BitSerdeDeserialization;
use bit_serde_trait::BitSerdeDeserializationMax;
use bit_serde_trait::BitSerdeSerialization;
use bit_serde_trait::BitSerdeSerializationMax;

use proc_macro::{token_stream, Ident, Span};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::ext::IdentExt;
use syn::spanned::Spanned;
use syn::token::Type;
use syn::{
    parse_macro_input, parse_quote, Attribute, AttributeArgs, Data, DeriveInput, Fields,
    GenericParam, Generics, Index, Meta,
};
#[proc_macro_attribute]
pub fn bit_serde(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut ser_flag: bool = false;
    let mut de_flag: bool = false;

    check_attributes(args, &mut ser_flag, &mut de_flag);

    if ser_flag {
        println!("Going to implement serialize")
    }
    if de_flag {
        println!("Going to implement deserialize")
    }

    let result: TokenStream;
    let input = parse_macro_input!(input as DeriveInput);
    let input_tokens = input.to_token_stream();
    let data = input.data;

    let struct_name = format_ident!("{}", input.ident.to_string());

    match data {
        Data::Struct(the_struct) => match the_struct.fields {
            Fields::Named(ref fields) => {
                let fields = &fields.named;

                let field_write_bits_to = fields.iter().map(|f| {
                    let field_name = format_ident!("{}", f.ident.clone().unwrap().to_string());
                    

                    quote_spanned! {f.ty.span() =>

                        self.#field_name.write_bits_to(destination)?;
                    }
                });

                let field_deserialize_from = fields.iter().map(|f| {
                    let temp = format_ident!("{}_temp", f.ident.clone().unwrap().to_string());
                    let type_name = f.ty.to_token_stream();

                    
                    quote_spanned! {f.ty.span() =>       
                        let parts:(&BitSlice<u8,Lsb0>, #type_name )  = BitSerdeDeserialization::deserialize_from(&data);
                        data = parts.0;
                        let #temp = parts.1;
                    }
                });

                let struct_initializtion = fields.iter().map(|f| {
                    let field_name = format_ident!("{}", f.ident.clone().unwrap().to_string());
                    let temp = format_ident!("{}_temp", f.ident.clone().unwrap().to_string());
                    let type_name = f.ty.to_token_stream();

                    
                    quote_spanned! {f.ty.span() =>       
                        #field_name : #temp
                    }
                });

                result = quote! {
                    

                    #input_tokens 
                    
                    impl BitSerdeSerialization for #struct_name {
                        fn write_bits_to(&self, destination: &mut BitVec<u8, Lsb0>) -> std::io::Result<()> {

                            #(#field_write_bits_to)*
                            Ok (())
                        }

                    }

                    impl BitSerdeDeserialization for #struct_name {

                        fn deserialize(data: &Vec<u8>) -> Self {

                            let bs = data.view_bits::<Lsb0>();

                            BitSerdeDeserialization::deserialize_from(bs).1
                        }

                        fn deserialize_from(mut data: &BitSlice<u8,Lsb0>) -> (&BitSlice<u8, Lsb0>,Self) {

                            #(#field_deserialize_from)*
                                
                            let the_object  = Self {  #(#struct_initializtion),* };
                            (data,the_object)
                        }
                    } 
                };
            }

            _ => {
                unimplemented!()
            }
        },
        _ => {
            unimplemented!()
        }
    }

    proc_macro::TokenStream::from(result)
}

fn check_attributes(
    args: proc_macro::TokenStream,
    ser_flag: &mut bool,
    de_flag: &mut bool,
) -> proc_macro::TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);

    let attribute_error_message = "only Serialize | Deserialize attributes allowed";
    for a in args.iter() {
        match a {
            syn::NestedMeta::Meta(meta) => match meta {
                syn::Meta::Path(path) => {
                    if let Some(id) = path.get_ident() {
                        if id == "Deserialize" {
                            *ser_flag = true;
                        } else if id == "Serialize" {
                            *de_flag = true;
                        } else {
                            unimplemented!("{attribute_error_message}");
                        }
                    } else {
                        unimplemented!("{attribute_error_message}");
                    }
                }
                _ => {
                    unimplemented!("{attribute_error_message}");
                }
            },
            _ => {
                unimplemented!("{attribute_error_message}");
            }
        }
    }
    let a = quote! {
        " "
    };

    proc_macro::TokenStream::from(a)
}

fn is_vec(type_path: &syn::TypePath) -> bool {
    type_path
        .path
        .segments
        .last()
        .map_or(false, |segment| segment.ident == "Vec")
}
