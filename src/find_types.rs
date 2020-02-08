use crate::as_types::AsTypes;
use syn::visit::Visit;
use syn::{ItemImpl, Type};

pub trait FindTypes {
    fn find_types(&self) -> Vec<Type>;
}

#[derive(Default)]
struct FindTypesVisitor {
    types: Vec<Type>,
}

impl Visit<'_> for FindTypesVisitor {
    fn visit_type(&mut self, node: &'_ Type) {
        if let Some(types) = node.as_types() {
            self.types = types.into_iter().cloned().collect()
        }

        syn::visit::visit_type(self, node)
    }
}

impl FindTypes for ItemImpl {
    fn find_types(&self) -> Vec<Type> {
        let mut visitor = FindTypesVisitor::default();
        visitor.visit_item_impl(self);

        visitor.types
    }
}
