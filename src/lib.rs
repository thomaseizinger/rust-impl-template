extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::Type;
use syn::TypeTuple;
use syn::{parse_macro_input, ItemImpl};
use syn::{Item, TypeParen};

#[proc_macro_attribute]
pub fn impl_template(_: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as Item);

    let item_impl = match item {
        Item::Impl(item_impl) => item_impl,
        _ => panic!("impl-template can only be used on impl items"),
    };

    let impl_blocks = make_impl_blocks(&item_impl);

    let expanded = quote! {
        #(#impl_blocks)*
    };

    TokenStream::from(expanded)
}

fn make_impl_blocks(item_impl: &ItemImpl) -> Vec<ItemImpl> {
    if let Some(types) = parse_double_tuple(&item_impl.self_ty) {
        return types
            .into_iter()
            .map(|ty| {
                let mut new_impl_block = item_impl.clone();

                new_impl_block.self_ty = Box::new(ty.clone());
                new_impl_block
            })
            .collect();
    }

    vec![item_impl.clone()]
}

fn parse_double_tuple(ty: &Type) -> Option<Vec<&Type>> {
    let inner = match ty {
        Type::Paren(TypeParen { elem, .. }) => elem.as_ref(),
        _ => return None,
    };

    match inner {
        Type::Tuple(TypeTuple { elems, .. }) => {
            Some(elems.pairs().map(|pair| pair.into_value()).collect())
        }
        _ => None,
    }
}
