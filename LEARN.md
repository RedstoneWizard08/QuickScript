# How I built QuickScript

1. Create a basic parser.

Use a library like [pest](https://docs.rs/pest) or [peg](https://docs.rs/peg) to achieve this.

2. Learn how to use Cranelift.

A great way to learn is to look at their [jit demo](https://github.com/bytecodealliance/cranelift-jit-demo) and check out how they did it.

3. Implement your own.

Using a lot of `match` cases and enums, create your own compiler to turn functions into machine code.
