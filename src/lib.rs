extern crate proc_macro;
use frunk_proc_macro_helpers::build_label_type;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident};

#[proc_macro]
pub fn label(input: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(input as Ident);
    let label_type = build_label_type(&ident);
    let ast = quote! {
        #label_type
    };
    TokenStream::from(ast)
}
