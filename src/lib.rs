extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn impl_template(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = args;
    let _ = input;

    let expanded = quote! {

    };

    TokenStream::from(expanded)
}
