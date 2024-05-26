// Import proc-macro crate
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote; // syntax-tree-DS  ==> Rust code
use syn; // string of Rust-code ==> syntax-tree-DS

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of RUST code as a syntax tree that we can manipulate.
    let ast = syn::parse(input).unwrap();
    // BUild the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! The name is {}", stringify!(#name));
            }
        }
    };
    gen.into()
}
