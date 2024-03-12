This is a modified version of Cranelift's frontend, taken from [here](https://github.com/bytecodealliance/wasmtime/tree/bd2ea901d3fa4555ffb3b8dc73c45455fe8a4439/cranelift/frontend) at commit bd2ea901d3fa4555ffb3b8dc73c45455fe8a4439 under the [Apache 2.0 license](./LICENSE).

This crate provides a straightforward way to create a
[Cranelift](https://crates.io/crates/cranelift) IR function and fill it with
instructions translated from another language. It contains an SSA construction
module that provides convenient methods for translating non-SSA variables into
SSA Cranelift IR values via `use_var` and `def_var` calls.
