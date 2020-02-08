use syn::export::Span;
use syn::visit_mut::VisitMut;
use syn::{ItemImpl, Path, Type, TypeParen, TypePath};

pub trait RenderWithType {
    fn render_with_type(&self, type_path: TypePath, declaration_site: Span) -> Self;
}

impl RenderWithType for ItemImpl {
    fn render_with_type(&self, type_path: TypePath, declaration_site: Span) -> Self {
        let mut new_impl_block = self.clone();

        let mut visitor = ReplacePlaceholderVisitor {
            type_path,
            declaration_site,
            replace_current_type: false,
        };
        visitor.visit_item_impl_mut(&mut new_impl_block);

        new_impl_block
    }
}

struct ReplacePlaceholderVisitor {
    type_path: TypePath,
    declaration_site: Span,

    replace_current_type: bool,
}

impl VisitMut for ReplacePlaceholderVisitor {
    fn visit_path_mut(&mut self, node: &mut Path) {
        let contains_placeholder = node
            .segments
            .iter()
            .any(|segment| segment.ident == "__TYPE__");

        if contains_placeholder {
            let segments = node
                .segments
                .iter()
                .map(|segment| {
                    if segment.ident == "__TYPE__" {
                        self.type_path.path.segments.iter().collect()
                    } else {
                        vec![segment]
                    }
                })
                .flatten()
                .cloned()
                .collect();

            node.segments = segments;
        } else {
            syn::visit_mut::visit_path_mut(self, node);
        }
    }

    fn visit_type_mut(&mut self, node: &mut Type) {
        syn::visit_mut::visit_type_mut(self, node);

        if self.replace_current_type {
            *node = Type::Path(self.type_path.clone());
            self.replace_current_type = false;
        }
    }

    fn visit_type_paren_mut(&mut self, node: &mut TypeParen) {
        let current_span = node.paren_token.span;
        let declaration_site = self.declaration_site;
        if current_span.start() == declaration_site.start()
            && current_span.end() == declaration_site.end()
        {
            self.replace_current_type = true;
        }
    }
}
