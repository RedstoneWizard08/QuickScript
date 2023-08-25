use crate::arch::Architecture;

pub trait Function {
    fn new(value: String) -> Self;
    fn name() -> String;
    fn compile(&self, arch: Architecture) -> (String, String);
}
