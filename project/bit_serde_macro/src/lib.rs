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
use syn::FieldsNamed;
use syn::ext::IdentExt;
use syn::spanned::Spanned;
use syn::token::Type;
use syn::DataStruct;
use syn::Field;
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

    let mut input = parse_macro_input!(input as DeriveInput);
    let input_tokens = input.to_token_stream();
    let data = &input.data;

    let mut result: TokenStream = quote! { 
    };

    if ser_flag || de_flag == false {}



    match data {
        Data::Struct(_) => {
            
            let tokens = struct_implementation(&input,ser_flag,de_flag);
            
            result = quote ! {
                #result
                #tokens
            }
        }
        _ => {
            unimplemented!();
        }
    }
    strip_attributes(&mut input);

    let input_tokens = input.to_token_stream();

    result = quote ! {
        #input_tokens
        #result 
    };

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

fn check_max_constraint(field: &Field) -> Option<usize> {
    
    for attr in field.attrs.iter() {
        if let Ok(meta) = attr.parse_meta() {
            match meta {
                syn::Meta::NameValue(nv) => {
                    if let Some(id) = nv.path.get_ident() {
                        let lit = nv.lit;
                        if let syn::Lit::Int(val) = lit {
                            let number_str = val.base10_digits();

                            println!("{} = {}", id.to_string(), val.base10_digits());

                            if  id.to_string() == "max" {
                                match number_str.parse::<usize>() {
                                    Ok(number) => return Some(number),
                                    Err(_) => {
                                        println!("Failed to parse the string as an i32");
                                        return None;
                                    }
                                }
                            }
                            
                            
                        } else {
                            println!("not a int val {}", lit.into_token_stream().to_string());
                        }
                    } else {
                        println!(
                            "not a ident path {}",
                            nv.path.into_token_stream().to_string()
                        );
                    }
                }
                _ => {
                    println!(
                        "not a namevalue attirbute {}",
                        meta.into_token_stream().to_string()
                    );
                }
            }
        } else {
            println!("not meta attirbutes {}", attr.tokens.to_string());
        }
    }
    None
}

fn struct_implementation(input: &DeriveInput, ser_flag : bool, deser_flag : bool) -> TokenStream {

    let mut result: TokenStream = quote! {

    };
    let struct_name = &input.ident;    
    if let Data::Struct(the_struct) = &input.data{

        match the_struct.fields {
            Fields::Named(ref fields) => {
                let fields = &fields.named;

                if ser_flag{
                    let field_write_bits_to = fields.iter().map(|f| {
                        let field_name = format_ident!("{}", f.ident.clone().unwrap().to_string());
                        
                        let constraint = check_max_constraint(f);

                        if let Some(max) = constraint {
                            quote_spanned! {f.ty.span() =>
                                self.#field_name.write_bits_to_with_max(destination,#max)?;
                            }
                        }
                        else {
                            quote_spanned! {f.ty.span() =>
                                self.#field_name.write_bits_to(destination)?;
                            }
                        }
                    }); 
                    result = quote ! {
                        #result 

                        impl BitSerdeSerialization for #struct_name {
                            fn write_bits_to(&self, destination: &mut BitVec<u8, Lsb0>) -> std::io::Result<()> {
    
                                #(#field_write_bits_to)*
                                Ok (())
                            }    
                        }
                    }           
                }
                if deser_flag {
                    let field_deserialize_from = fields.iter().map(|f| {
                        let temp = format_ident!("{}_temp", f.ident.clone().unwrap().to_string());
                        let type_name = f.ty.to_token_stream();

                        let constraint = check_max_constraint(f);
                        
                        if let Some(max) = constraint {
                            quote_spanned! {f.ty.span() =>       
                                let parts:(&BitSlice<u8,Lsb0>, #type_name )  = BitSerdeDeserializationMax::deserialize_from_with_max(&data,#max);
                                data = parts.0;
                                let #temp = parts.1;
                            }
                        }
                        else {
                            quote_spanned! {f.ty.span() =>       
                                let parts:(&BitSlice<u8,Lsb0>, #type_name )  = BitSerdeDeserialization::deserialize_from(&data);
                                data = parts.0;
                                let #temp = parts.1;
                            }
                        }
                    });
                    let struct_initializtion = fields.iter().map(|f| {
                        let field_name = format_ident!("{}", f.ident.clone().unwrap().to_string());
                        let temp = format_ident!("{}_temp", f.ident.clone().unwrap().to_string()); 
                            
                        quote_spanned! {f.ty.span() =>       
                            #field_name : #temp
                        }
                    }); 
                    result = quote ! {
                                            
                    #result

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
                    }
                }
            }
            _ => {
                unimplemented!()
            }
        }
    }
    else {

    }
result
}


fn strip_attributes(input: &mut DeriveInput ) {
    
    if let Data::Struct(the_struct) = &mut input.data{

        if let Fields::Named(ref mut fields) = the_struct.fields {

            let x  = &mut fields.named;

            for f in x.iter_mut() {            

                f.attrs.retain(|attr| {

                    let mut ok = true;

                    if let Ok(meta) = attr.parse_meta() {

                        if let syn::Meta::NameValue(nv ) = meta{

                            if let Some(id) = nv.path.get_ident() {
        
                                if id.to_string() == "max" {
                                    ok = false;
                                }
                            }
                        }
                    }             
                    ok                
                });                        
            }                        
        }
    }
    
} 