// src/lib.rs
extern crate proc_macro;
use std::sync::LazyLock;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{bracketed, parse::Parse, parse_macro_input, parse_str, AngleBracketedGenericArguments, DeriveInput, GenericParam, ItemFn, Path, TypeParam};

pub(crate) static M:LazyTokenStream= LazyTokenStream::new(||{"a".to_string()});
pub(crate) struct LazyTokenStream{
    t:LazyLock<String>
}
impl LazyTokenStream { pub const fn new(f:fn()->String) -> Self{ Self { t: LazyLock::new(f) } } }
impl std::ops::Deref for LazyTokenStream{
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.t }
}
unsafe impl Sync for LazyTokenStream{ }
unsafe impl Send for LazyTokenStream{ }
impl ToTokens for LazyTokenStream {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        Path::to_tokens(&parse_str(&self).unwrap(), tokens);
    }
}


#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let _ts = proc_macro2::token_stream::TokenStream::from(input.clone());
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let expanded = quote! {
        impl #name {
            fn hello(&self) {
                println!("Hello, Macro! My name is {}!", stringify!(#M));
            }
        }
    };
    expanded.into()
}
#[proc_macro_attribute]
pub fn show_para(attr:proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream{
    let input_fn = parse_macro_input!(item as ItemFn);
    
    
    
    // Get the function signature
    let mut sig = input_fn.sig;
    let block = input_fn.block;
    
    // Create generic parameter "T: Tx"
    let type_param: GenericParam = syn::parse_str::<TypeParam>("T:  LetSmt").unwrap().into();
    // let v = sig.generics.type_params().map(|x| x.to_token_stream().to_string()).collect::<Vec<_>>();
    // panic!("{:?} {:?} ",v, type_param.to_token_stream().to_string());
    sig.generics.params.push(type_param);

    let mut s=  String::new();
    // Transform the return type to use the generic parameter
    if let syn::ReturnType::Type(arrow, ty) = &mut sig.output {
        if let syn::Type::Tuple(tuple) = ty.as_mut() {
            for elem in &mut tuple.elems {
                if let syn::Type::Path(type_path) = elem {
                    // type_path.path;
                    s += format!("{}", type_path.path.to_token_stream().to_string()).as_str();

                    if let Some(ident) = type_path.path.get_ident() {
                        // s+= ident.to_token_stream().to_string().as_str();
                        // s+= "\n";
                        // Add <T> to the type
                        let generic_arg: syn::GenericArgument = syn::parse_str("T").unwrap();
                        
                        // type_path.path.segments.push(syn::parse_str::<syn::PathSegment>("T").unwrap());
                        let brackted_t :AngleBracketedGenericArguments  = parse_str("<T>").unwrap();
                        s += &brackted_t.to_token_stream().to_string();
                        type_path.path.segments.last_mut().unwrap().arguments = syn::PathArguments::AngleBracketed(brackted_t);
                        // type_path
                    }
                }
            }
        }
        
    };
    // panic!("{}", s);
    quote! { #sig 
        #block
    }.into()
}