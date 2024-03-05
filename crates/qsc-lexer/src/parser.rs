use pest_derive::Parser;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Parser)]
#[grammar = "quickscript.pest"]
pub struct CodeParser;
