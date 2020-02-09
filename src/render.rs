use syn::export::Span;
use syn::visit_mut::VisitMut;
use syn::{ItemImpl, Path, Type, TypePath};

pub trait Render {
    fn render(&self, replacements: Vec<Replacement>) -> Self;
}

impl Render for ItemImpl {
    fn render(&self, replacements: Vec<Replacement>) -> Self {
        let mut new_impl_block = self.clone();

        for r in replacements {
            let mut visitor = ReplacePlaceholderVisitor {
                type_path: r.type_path,
                declaration_site: r.declaration,
                placeholder: format!("__TYPE{}__", r.pattern_index),
            };

            visitor.visit_item_impl_mut(&mut new_impl_block);
        }

        new_impl_block
    }
}

#[derive(Clone)]
pub struct Replacement {
    pub pattern_index: usize,
    pub type_path: TypePath,
    pub declaration: Span,
}

impl Eq for Replacement {}

impl PartialEq for Replacement {
    fn eq(&self, other: &Self) -> bool {
        self.type_path == other.type_path && self.pattern_index == other.pattern_index
    }
}

struct ReplacePlaceholderVisitor {
    type_path: TypePath,
    declaration_site: Span,
    placeholder: String,
}

impl VisitMut for ReplacePlaceholderVisitor {
    fn visit_path_mut(&mut self, node: &mut Path) {
        let contains_placeholder = node
            .segments
            .iter()
            .any(|segment| segment.ident == self.placeholder);

        if contains_placeholder {
            let segments = node
                .segments
                .iter()
                .map(|segment| {
                    if segment.ident == self.placeholder {
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
        match node {
            Type::Paren(inner) => {
                let current_span = inner.paren_token.span;
                let declaration_site = self.declaration_site;

                if format!("{:?}", current_span) == format!("{:?}", declaration_site) {
                    *node = Type::Path(self.type_path.clone());
                }
            }
            _ => {
                syn::visit_mut::visit_type_mut(self, node);
            }
        }
    }
}
