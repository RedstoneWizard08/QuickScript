use pest::Span;

use super::sym::SymbolNode;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeNode<'i> {
    pub span: Span<'i>,
    pub name: &'i str,
    pub generics: Vec<SymbolNode<'i>>,
}

impl<'i> TypeNode<'i> {
    pub fn as_str(&self) -> String {
        if self.generics.len() > 0 {
            format!(
                "{}<{}>",
                self.name,
                self.generics
                    .iter()
                    .map(|v| v.value)
                    .collect::<Vec<&str>>()
                    .join(", ")
            )
        } else {
            format!("{}", self.name)
        }
    }
}
