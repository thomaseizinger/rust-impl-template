use crate::as_types::AsTypes;
use crate::contains_placeholder::ContainsPlacerholder;
use crate::extract_path::ExtractPath;
use syn::visit_mut::VisitMut;
use syn::{ItemImpl, Path, Type};

pub trait RenderWithType {
    fn render_with_type(&self, ty: Type) -> Self;
}

impl RenderWithType for ItemImpl {
    fn render_with_type(&self, ty: Type) -> Self {
        let path = ty.extract_path();
        let mut new_impl_block = self.clone();

        let mut visitor = ReplacePlaceholderVisitor { ty, path };
        visitor.visit_item_impl_mut(&mut new_impl_block);

        new_impl_block
    }
}

struct ReplacePlaceholderVisitor {
    ty: Type,
    path: Path,
}

impl VisitMut for ReplacePlaceholderVisitor {
    fn visit_path_mut(&mut self, node: &mut Path) {
        if node.contains_placeholder() {
            let segments = node
                .segments
                .iter()
                .map(|segment| {
                    if segment.ident == "__TYPE__" {
                        self.path.segments.iter().collect()
                    } else {
                        vec![segment]
                    }
                })
                .flatten()
                .cloned()
                .collect();

            node.segments = segments;
        }

        syn::visit_mut::visit_path_mut(self, node);
    }

    fn visit_type_mut(&mut self, node: &mut Type) {
        if node.as_types().is_some() || node.contains_placeholder() {
            *node = self.ty.clone();
        }

        syn::visit_mut::visit_type_mut(self, node)
    }
}
