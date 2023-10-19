use crate::arch::Architecture;

pub trait Function<T> {
    fn new(value: T) -> Self;
    fn name() -> String;
    fn compile(&self, arch: Architecture) -> (String, String);
}
