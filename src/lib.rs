extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::visit::Visit;
use syn::visit_mut::VisitMut;
use syn::Type;
use syn::TypeTuple;
use syn::{Item, TypeParen};

#[proc_macro_attribute]
pub fn impl_template(_: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as Item);

    let item_impl = match item {
        Item::Impl(item_impl) => item_impl,
        _ => panic!("impl-template can only be used on impl items"),
    };

    let mut visitor = FindTypesVisitor::default();
    visitor.visit_item_impl(&item_impl);

    let impl_blocks = visitor
        .types
        .into_iter()
        .map(|ty| {
            let mut new_item_impl = item_impl.clone();

            ReplaceTypeVisitor { ty }.visit_item_impl_mut(&mut new_item_impl);

            new_item_impl
        })
        .collect::<Vec<_>>();

    let expanded = if impl_blocks.is_empty() {
        quote! {
            #item_impl
        }
    } else {
        quote! {
            #(#impl_blocks)*
        }
    };

    TokenStream::from(expanded)
}

#[derive(Default)]
struct FindTypesVisitor {
    types: Vec<Type>,
}

impl Visit<'_> for FindTypesVisitor {
    fn visit_type(&mut self, node: &'_ Type) {
        if let Some(types) = parse_double_tuple(node) {
            self.types = types.into_iter().cloned().collect()
        }

        syn::visit::visit_type(self, node)
    }
}

struct ReplaceTypeVisitor {
    ty: Type,
}

impl VisitMut for ReplaceTypeVisitor {
    fn visit_type_mut(&mut self, node: &mut Type) {
        if parse_double_tuple(&node).is_some() {
            *node = self.ty.clone();
        }

        syn::visit_mut::visit_type_mut(self, node)
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
