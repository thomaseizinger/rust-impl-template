use syn::Type;
use syn::TypeParen;
use syn::TypeTuple;

pub trait AsTypes {
    fn as_types(&self) -> Option<Vec<&Type>>;
}

impl AsTypes for Type {
    fn as_types(&self) -> Option<Vec<&Type>> {
        let inner = match self {
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
}
