use std::default;
use std::num::IntErrorKind;
use std::sync::Arc;

use proc_macro::{token_stream, Ident, Span};
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens, format_ident};
use syn::spanned::Spanned;
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
    let data = input.data;

    match data {
        Data::Struct(the_struct) => match the_struct.fields {
            Fields::Named(ref fields) => {
                let span = fields.named.iter().next();
                let span = span.unwrap();
                let span = span.ident.span();

                let mut span = proc_macro2::Span::call_site();
                let mut i = 1;
                let recurse = fields.named.iter().map(|f| {
                    let func_name = format_ident! ("randfn_{}", f.ty.to_token_stream().to_string());
                    
                    quote_spanned! {f.ty.span() =>
                        fn #func_name(id: i32) {
                            println!("{}", BitSerdeSerialization::serialize(&id));
                        }   
                    }
                });

                result = quote! {

                    #(#recurse)*
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
        "a"
    };

    proc_macro::TokenStream::from(a)
}
