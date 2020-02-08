use syn::export::Span;
use syn::visit::Visit;
use syn::{ItemImpl, TypeParen, TypePath};

pub trait FindTypes {
    fn find_types(&self) -> Option<(Vec<TypePath>, Span)>;
}

#[derive(Default)]
struct FindTypesVisitor {
    type_paths: Vec<TypePath>,
    declaration_span: Option<Span>,

    in_paren: bool,
}

impl Visit<'_> for FindTypesVisitor {
    fn visit_type_paren(&mut self, node: &TypeParen) {
        if self.declaration_span.is_some() {
            return; // early return, we don't support more than one declaration site for now
        }

        self.in_paren = true;

        syn::visit::visit_type_paren(self, node);

        if !self.type_paths.is_empty() {
            self.declaration_span = Some(node.paren_token.span)
        }

        self.in_paren = false;
    }

    fn visit_type_path(&mut self, node: &TypePath) {
        if self.in_paren {
            self.type_paths.push(node.clone())
        }

        syn::visit::visit_type_path(self, node);
    }
}

impl FindTypes for ItemImpl {
    fn find_types(&self) -> Option<(Vec<TypePath>, Span)> {
        let mut visitor = FindTypesVisitor::default();
        visitor.visit_item_impl(self);

        visitor
            .declaration_span
            .map(|declaration_span| (visitor.type_paths, declaration_span))
    }
}
