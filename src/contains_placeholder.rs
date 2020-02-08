use syn::visit::Visit;
use syn::Ident;
use syn::{Path, Type};

pub trait ContainsPlacerholder {
    fn contains_placeholder(&self) -> bool;
}

#[derive(Default)]
struct ContainsPlaceholderVisitor {
    is_placeholder: bool,
}

impl ContainsPlacerholder for Type {
    fn contains_placeholder(&self) -> bool {
        let mut visitor = ContainsPlaceholderVisitor::default();
        visitor.visit_type(self);

        visitor.is_placeholder
    }
}

impl ContainsPlacerholder for Path {
    fn contains_placeholder(&self) -> bool {
        let mut visitor = ContainsPlaceholderVisitor::default();
        visitor.visit_path(self);

        visitor.is_placeholder
    }
}

impl Visit<'_> for ContainsPlaceholderVisitor {
    fn visit_ident(&mut self, ident: &'_ Ident) {
        if self.is_placeholder {
            // early abort if we already determined that we have a placeholder type
            return;
        }

        self.is_placeholder = *ident == "__TYPE__"
    }
}
