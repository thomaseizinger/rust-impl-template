extern crate proc_macro;

mod as_types;
mod contains_placeholder;
mod extract_path;
mod find_types;
mod render_with_type;

use crate::find_types::FindTypes;
use crate::render_with_type::RenderWithType;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::Item;

#[proc_macro_attribute]
pub fn impl_template(_: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as Item);

    let template = match item {
        Item::Impl(item_impl) => item_impl,
        _ => panic!("impl-template can only be used on impl items"),
    };

    let types = template.find_types();

    let impl_blocks = types
        .into_iter()
        .map(|ty| template.render_with_type(ty))
        .collect::<Vec<_>>();

    let expanded = if impl_blocks.is_empty() {
        quote! {
            #template
        }
    } else {
        quote! {
            #(#impl_blocks)*
        }
    };

    TokenStream::from(expanded)
}
