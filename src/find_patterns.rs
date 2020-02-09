use syn::export::Span;
use syn::visit::Visit;
use syn::{ItemImpl, TypeParen, TypePath};

pub trait FindPatterns {
    fn find_patterns(&self) -> Vec<Pattern>;
}

#[derive(Default)]
struct FindPatternsVisitor {
    patterns: Vec<Pattern>,

    current_type_paths: Vec<TypePath>,
    currently_visiting_paren_type: bool,
}

pub struct Pattern {
    pub type_paths: Vec<TypePath>,
    pub declaration: Span,
}

//impl Pattern {
//    pub fn render_types(&self) -> String {
//        self.type_paths
//            .iter()
//            .map(|ty| {
//                ty.path
//                    .segments
//                    .iter()
//                    .map(|segment| segment.ident.to_string())
//                    .collect::<Vec<_>>()
//                    .as_slice()
//                    .join("::")
//            })
//            .collect::<Vec<_>>()
//            .as_slice()
//            .join(", ")
//    }
//}

impl Visit<'_> for FindPatternsVisitor {
    fn visit_type_paren(&mut self, node: &TypeParen) {
        self.currently_visiting_paren_type = true;

        syn::visit::visit_type_paren(self, node);

        if !self.current_type_paths.is_empty() {
            self.patterns.push(Pattern {
                type_paths: std::mem::replace(&mut self.current_type_paths, Vec::new()),
                declaration: node.paren_token.span,
            });
        }

        self.currently_visiting_paren_type = false;
    }

    fn visit_type_path(&mut self, node: &TypePath) {
        if self.currently_visiting_paren_type {
            self.current_type_paths.push(node.clone())
        }

        syn::visit::visit_type_path(self, node);
    }
}

impl FindPatterns for ItemImpl {
    fn find_patterns(&self) -> Vec<Pattern> {
        let mut visitor = FindPatternsVisitor::default();
        visitor.visit_item_impl(self);

        visitor.patterns
    }
}
