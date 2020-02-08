extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, GenericArgument, ItemImpl};
use syn::{Item, TypeParen};
use syn::{PathArguments, TypeTuple};
use syn::{PathSegment, Type};

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

    if let Some((never_token, path, for_token)) = &item_impl.trait_ {
        if let Some(PathSegment { arguments, .. }) = path.segments.single_or_none() {
            if let PathArguments::AngleBracketed(generic_args) = arguments {
                if let Some(GenericArgument::Type(ty)) = generic_args.args.single_or_none() {
                    if let Some(types) = parse_double_tuple(ty) {
                        return types
                            .into_iter()
                            .map(|ty| {
                                let mut new_impl_block = item_impl.clone();

                                let mut new_path = path.clone();

                                let mut new_generic_args = generic_args.clone();
                                *new_generic_args.args.last_mut().unwrap() =
                                    GenericArgument::Type(ty.clone());

                                new_path.segments.last_mut().unwrap().arguments =
                                    PathArguments::AngleBracketed(new_generic_args);

                                new_impl_block.trait_ = Some((*never_token, new_path, *for_token));

                                new_impl_block
                            })
                            .collect();
                    }
                }
            }
        }
    }

    vec![item_impl.clone()]
}

trait SingleOrNone<T> {
    fn single_or_none(&self) -> Option<&T>;
}

impl<T, P> SingleOrNone<T> for Punctuated<T, P> {
    fn single_or_none(&self) -> Option<&T> {
        if self.len() != 1 {
            return None;
        }

        self.first()
    }
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
