This is a modified version of Cranelift's object backend, taken from [here](https://github.com/bytecodealliance/wasmtime/tree/bd2ea901d3fa4555ffb3b8dc73c45455fe8a4439/cranelift/object) at commit bd2ea901d3fa4555ffb3b8dc73c45455fe8a4439 under the [Apache 2.0 license](./LICENSE).

This crate contains a library that enables
[Cranelift](https://crates.io/crates/cranelift)
to emit native object (".o") files, using the
[object](https://crates.io/crates/object) library.
