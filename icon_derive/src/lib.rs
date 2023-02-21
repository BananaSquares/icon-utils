use darling::FromDeriveInput;
use proc_macro::{self,TokenStream};
use quote::quote;
use syn::{parse_macro_input,DeriveInput};

#[derive(FromDeriveInput, Default)]
#[darling(attributes(params))]
struct Params {
    e: String
}

#[proc_macro_derive(Transaction)]
pub fn transaction(input: TokenStream) -> TokenStream {
    let DeriveInput {ident, attrs, data,    ..} = parse_macro_input!(input as DeriveInput);
    let fields = match &data {
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