extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::visit::Visit;
use syn::visit_mut::VisitMut;
use syn::Ident;
use syn::TypeTuple;
use syn::{Item, TypeParen};
use syn::{ItemImpl, Type};

#[proc_macro_attribute]
pub fn impl_template(_: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as Item);

    let template = match item {
        Item::Impl(item_impl) => item_impl,
        _ => panic!("impl-template can only be used on impl items"),
    };

    let types = FindTypesVisitor::find_types(&template);

    let impl_blocks = types
        .into_iter()
        .map(|ty| ImplBlockMaker::make_impl_block(&template, ty))
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

#[derive(Default)]
struct FindTypesVisitor {
    types: Vec<Type>,
}

impl FindTypesVisitor {
    fn find_types(template: &ItemImpl) -> Vec<Type> {
        let mut visitor = FindTypesVisitor::default();
        visitor.visit_item_impl(&template);

        visitor.types
    }
}

impl Visit<'_> for FindTypesVisitor {
    fn visit_type(&mut self, node: &'_ Type) {
        if let Some(types) = parse_double_tuple(node) {
            self.types = types.into_iter().cloned().collect()
        }

        syn::visit::visit_type(self, node)
    }
}

struct ImplBlockMaker {
    ty: Type,
}

impl ImplBlockMaker {
    fn make_impl_block(template: &ItemImpl, ty: Type) -> ItemImpl {
        let mut visitor = ImplBlockMaker { ty };

        let mut new_impl_block = template.clone();

        visitor.visit_item_impl_mut(&mut new_impl_block);

        new_impl_block
    }
}

impl VisitMut for ImplBlockMaker {
    fn visit_type_mut(&mut self, node: &mut Type) {
        if parse_double_tuple(&node).is_some() {
            *node = self.ty.clone();
        }

        if IsPlaceholderVisitor::is_placeholder(&node) {
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

#[derive(Default)]
struct IsPlaceholderVisitor {
    is_placeholder: bool,
}

impl IsPlaceholderVisitor {
    fn is_placeholder(ty: &Type) -> bool {
        let mut visitor = IsPlaceholderVisitor::default();

        visitor.visit_type(ty);

        visitor.is_placeholder
    }
}

impl Visit<'_> for IsPlaceholderVisitor {
    fn visit_ident(&mut self, ident: &'_ Ident) {
        if self.is_placeholder {
            // early abort if we already determined that we have a placeholder type
            return;
        }

        self.is_placeholder = *ident == "__TYPE__"
    }
}
