use syn::visit::Visit;
use syn::{Path, Type};

pub trait ExtractPath {
    fn extract_path(&self) -> Path;
}

#[derive(Default)]
struct PathExtractorVisitor {
    path: Option<Path>,
}

impl ExtractPath for Type {
    fn extract_path(&self) -> Path {
        let mut visitor = PathExtractorVisitor::default();
        visitor.visit_type(self);

        visitor.path.expect("type did not contain a path")
    }
}

impl Visit<'_> for PathExtractorVisitor {
    fn visit_path(&mut self, node: &Path) {
        if self.path.is_some() {
            panic!("only types with a single path are supported")
        }

        self.path = Some(node.clone())
    }
}
