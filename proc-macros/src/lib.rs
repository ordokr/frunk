#![doc(html_playground_url = "https://play.rust-lang.org/")]
//! OrdoFP Proc Macros
//!
//! This library holds procedural macros for ordofp
//!
//! Links:
//!   1. [Source on Github](https://github.com/lloydmeta/ordofp)
//!   2. [Crates.io page](https://crates.io/crates/ordofp)

extern crate ordofp_core;
extern crate ordofp_proc_macro_helpers;
extern crate proc_macro;

extern crate quote;
extern crate syn;

use ordofp_proc_macro_helpers::*;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr};

/// Build a generic path that can be used for traversals
#[proc_macro]
pub fn path(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as Expr);
    let path_type = build_path_type(expr);
    let ast = quote! {
        {
            let p: #path_type = ::ordofp_core::path::Path::new();
            p
        }
    };
    //    println!("ast: [{}]", ast);
    TokenStream::from(ast)
}

/// Build a generic path that can be used for traversals
#[proc_macro]
#[allow(non_snake_case)]
pub fn Path(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as Expr);
    let path_type = build_path_type(expr);
    let ast = quote! {
        #path_type
    };
    //    println!("ast: [{}]", ast);
    TokenStream::from(ast)
}
