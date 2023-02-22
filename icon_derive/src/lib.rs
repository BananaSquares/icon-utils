use darling::FromDeriveInput;
use proc_macro::{TokenStream};
use quote::quote;
use syn::{parse_macro_input,DeriveInput};
use syn::parse::Parser;
#[derive(FromDeriveInput, Default)]
#[darling(attributes(params))]
struct Params {
    e: String
}

#[proc_macro_derive(Transaction)]
pub fn transaction(input: TokenStream) -> TokenStream {
    let DeriveInput {ident, attrs, mut data,    ..} = parse_macro_input!(input as DeriveInput);
    let fields = match &mut data {
        syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(fields), ..}) => &fields.named,
        _ => panic!("Only structs with named fields are supported"),
        
    };
    let paramType = fields.iter().find(|f| f.ident.as_ref().unwrap() == "params").unwrap().ty.clone();
    let output = quote! {
        impl icon_utils::serializer::Transaction for &#ident {
            type Params = #paramType;
            fn params(&self) -> &Self::Params {
                &self.params
            }
            fn method(&self) -> &String {
                &self.method
            } 
        }
    };
    output.into()
}
#[proc_macro_attribute]
pub fn sort(_args: TokenStream, input: TokenStream) -> TokenStream{
    let mut ast = parse_macro_input!(input as syn::DeriveInput);
    let ident = &ast.ident;
    let data = &ast.data;
    let fields = match &data {
        syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(fields), ..}) => &fields.named,
        _ => panic!("Only structs with named fields are supported"),
    };
    let mut newFields: Vec<&syn::Field> = fields.iter().collect();
    newFields.sort_by(|a, b| a.ident.as_ref().unwrap().cmp(&b.ident.as_ref().unwrap()));
    let mut ast2 = ast.clone();
    let mut data2 = &mut ast2.data;
    match &mut data2 {
        syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(fields), ..}) => {
            fields.named.clear();
            for i in newFields {
                fields.named.push(i.clone());
            }
        },
        _ => panic!("Only structs with named fields are supported"),
        
    }
    return quote! {
        #ast2
    }.into()
}
