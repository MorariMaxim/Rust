use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::spanned::Spanned;use syn::Field;
use syn::{
    parse_macro_input, AttributeArgs, Data, DeriveInput, Fields,    
};
#[proc_macro_attribute]
pub fn bit_serde(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut ser_flag: bool = false;
    let mut de_flag: bool = false;

    check_attributes(args, &mut ser_flag, &mut de_flag);
    
    let mut input = parse_macro_input!(input as DeriveInput);
    let data = &input.data;

    let mut result: TokenStream = quote! { 
    };

    if ser_flag || de_flag == false {

    }

    match data {
        Data::Struct(_) => {            
            let tokens = struct_implementation(&input,ser_flag,de_flag);            

            result = quote ! {
                #result
                #tokens
            }
        }
        Data::Enum(_) =>{            
            let tokens = enum_implementation(&mut input,ser_flag,de_flag);            

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
                            *de_flag = true;
                        } else if id == "Serialize" {
                            *ser_flag = true;
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

                            //println!("{} = {}", id.to_string(), val.base10_digits());

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
                        let type_name = f.ty.to_token_stream().to_string();
                        if let Some(max) = constraint {
                            let part1 : String; 
                            let part3 = format!(", which exceeds constraint '{}'",max);
                            let value_to_compare  : TokenStream;
                            match get_field_type(&f.ty) {
                                FieldType::Other => {
                                    panic!("cant implement bitlevel serialization for this thype{}",type_name);
                                }
                                FieldType::Unsigned =>{
                                    value_to_compare = quote! {
                                        self.#field_name
                                    };
                                    part1  = format!("SerializationError: value of field '{}'",f.ident.clone().unwrap().to_string());
                                }   
                                FieldType::VecOrString => {
                                    value_to_compare = quote! {
                                        self.#field_name.len()
                                    };
                                    part1  = format!("SerializationError: '{}.len()'",f.ident.clone().unwrap().to_string());                                    
                                }
                            };
                            quote_spanned! {f.ty.span() =>
                                if (#value_to_compare as usize) > #max {
                                    let error_message = format!("{} = {} {}",#part1,#value_to_compare, #part3);
                                    return Err(std::io::Error::new(std::io::ErrorKind::Other, error_message ));
                                }
                                self.#field_name.write_bits_to_with_max(destination,#max)?;
                            }
                        }
                        else {
                            quote_spanned! {f.ty.span() =>

                                //BitSerdeSerialization::write_bits_to(&self.#field_name,destination)?;
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
                                let parts:(&BitSlice<u8,Lsb0>, #type_name )  = BitSerdeDeserializationMax::deserialize_from_with_max(&data,#max)?;
                                data = parts.0;
                                let #temp = parts.1;
                            }
                        }
                        else {
                            quote_spanned! {f.ty.span() =>       
                                let parts:(&BitSlice<u8,Lsb0>, #type_name )  = BitSerdeDeserialization::deserialize_from(&data)?;
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
                        fn deserialize_from(mut data: &BitSlice<u8,Lsb0>) -> std::io::Result<(&BitSlice<u8, Lsb0>,Self)> {

                            #(#field_deserialize_from)*
                                
                            let the_object  = Self {  #(#struct_initializtion),* };
                            Ok((data,the_object))
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

fn enum_implementation(input: &mut DeriveInput, ser_flag : bool, deser_flag : bool) -> TokenStream {
    let mut result: TokenStream = quote! {

    };
    add_clone_derive(input);
    
    let enum_name = &input.ident;  
    if let Data::Enum(en) = &input.data{
        
        let variants = &en.variants;

        let mut fields_counter = 0usize;

        for variant in variants.iter() {
            fields_counter +=1;
                        
            if let syn::Fields::Named(_) | syn::Fields::Unnamed(_) = &variant.fields {
                panic!("a variant of the enum has fields, the macro doesn't implement serialization for them");
            }                         
        }
        fields_counter-=1;
        if ser_flag { 
            result = quote ! {
                #result
                
                impl BitSerdeSerialization for #enum_name {
                    fn write_bits_to(&self, destination: &mut BitVec<u8, Lsb0>) -> std::io::Result<()> {
                                                
                        let val = ((*self).clone() as usize) as u128;
                        
                        val.write_bits_to_with_max(destination,#fields_counter)?;

                        Ok (())
                    }                    
                }
            }
        }

        if deser_flag{
             
            let mut counter = 0usize;
            let arms = variants.iter().map(|v|  {
                
                let variant_name = &v.ident;
                let tokens = quote! {
                    #counter => #enum_name::#variant_name,
                };
                counter+=1;
                
                tokens
            });

            result = quote! {

                #result

                impl BitSerdeDeserialization for #enum_name { 
                    fn deserialize_from(mut data: &BitSlice<u8, Lsb0>) -> std::io::Result<(&bitvec::slice::BitSlice<u8>, #enum_name)> {

                        let parts:(&BitSlice<u8,Lsb0>, u128 )  = BitSerdeDeserializationMax::deserialize_from_with_max(&data,#fields_counter)?;
                        data = parts.0;
    
                        let variant = match parts.1 as usize {
                            #(#arms)*
                            _ => {panic!("something went wrong");} 
                        };
    
                        Ok((data,variant))
                    }                    
                } 
            }
        }
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


fn add_clone_derive(input: &mut DeriveInput)  {
        
    let derive_segment = syn::PathSegment {
        ident: syn::Ident::new("derive", proc_macro2::Span::call_site()),
        arguments: Default::default(),
    };

    input.attrs.push(syn::Attribute {
        pound_token: Default::default(),
        style:syn::AttrStyle::Outer ,
        bracket_token: Default::default(),
        path: derive_segment.into(),
        tokens: quote! { (Clone) }.to_token_stream(),

    });    
}

enum FieldType {
    Unsigned,
    VecOrString,
    Other,
    
}
fn get_field_type(type_path: &syn::Type) -> FieldType {
    if let syn::Type::Path(type_path) = type_path {
        if let Some(segment) = type_path.path.segments.last() {

            match segment.ident.to_string().as_str() {
                
                "Vec" | "String" =>{
                    return  FieldType::VecOrString;
                }
                "u8" | "u16" | "u32" | "u64" | "u128" => {
                    return  FieldType::Unsigned;
                }
                _ => {
                    return  FieldType::Other;
                }
            }; 
        }
    } 
    return  FieldType::Other;
}