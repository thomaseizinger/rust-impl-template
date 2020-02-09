extern crate proc_macro;

mod find_patterns;
mod render;

use crate::find_patterns::{FindPatterns, Pattern};
use crate::render::Render;
use crate::render::Replacement;
use itertools::Itertools;
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

    let patterns = template.find_patterns();

    if patterns.is_empty() {
        return TokenStream::from(quote! {
            #template
        });
    }

    let replacements = patterns
        .into_iter()
        .enumerate()
        .map(
            |(
                pattern_index,
                Pattern {
                    type_paths,
                    declaration,
                },
            )| {
                type_paths.into_iter().map(move |type_path| Replacement {
                    pattern_index,
                    type_path,
                    declaration,
                })
            },
        )
        .multi_cartesian_product()
        .collect::<Vec<_>>();

    let impl_blocks = replacements
        .into_iter()
        .map(|r| {
            let impl_block = template.render(r);

            quote! {
                #impl_block
            }
        })
        .collect::<Vec<_>>();

    let expanded = quote! {
        #(#impl_blocks)*
    };

    TokenStream::from(expanded)
}
