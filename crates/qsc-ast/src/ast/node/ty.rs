use crate::span::StaticSpan;

use super::sym::SymbolNode;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TypeNode {
    #[serde(skip)]
    pub span: StaticSpan,
    pub name: String,
    pub generics: Vec<SymbolNode>,
}

impl TypeNode {
    pub fn as_str(&self) -> String {
        if self.generics.len() > 0 {
            format!(
                "{}<{}>",
                self.name,
                self.generics
                    .iter()
                    .map(|v| v.value.clone())
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        } else {
            format!("{}", self.name)
        }
    }
}
