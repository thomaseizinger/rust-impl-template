extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::Item;

#[proc_macro_attribute]
pub fn impl_template(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = args;

    let item = parse_macro_input!(input as Item);

    let expanded = quote! {
        #item
    };

    TokenStream::from(expanded)
}
