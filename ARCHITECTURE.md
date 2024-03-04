# Architecture

Here's how the QuickScript compiler works*:

\* Most of this is how I want it to work, not how it works right now. This is still very WIP and subject to change!

## 1. Parser

***Crate: part of `qsc-lexer`***

`qsc` uses [pest](https://pest.rs/) to parse the input code, which returns a tree that can be processed by the lexer.

## 2. Lexer

***Crate: `qsc-lexer`***

The lexer will transform `pest`'s tree into our `AbstractTree<'t>` type, which represents the raw abstract syntax tree. This tree has extra information in it, such as the `Span<'t>`s that `pest` will emit, allowing the processor to throw helpful errors if something is wrong.

## 3. Pre-processor

***Crate: `qsc-preprocessor`***

This will go through the `AbstractTree<'t>` and gather a list of defined functions and variables in the module's scope. This will immediately return to the invoker after this is done. This information is then used by the processor in its information collection stage.

## 4. Processor

***Crate: `qsc-processor`***

The processor works in three phases:

1. Walk through the AST and make sure that code is valid.
2. Collect information.
3. Make type inferences and fill missing data.

I like to call this the CCI architecture, which stands for "Check, Collect, Infer".

### 4-1. Walk through the AST

***Crate: part of `qsc-processor`***

This stage of the processor will walk through the `AbstractTree<'t>` and make sure that statements are complete, calls aren't missing required arguments, references exist, and that variables' static types match their values.

This stage does this using only static analysis, and will return the same `AbstractTree<'t>` and will throw any errors it finds.

### 4-2. Collect information

***Crate: part of `qsc-processor`***

This stage will walk through the `AbstractTree<'t>` again, but will construct a new `ProcessedTree<'t>`. This new processed tree will contain information about variable usages, function usages, and variable assignments.

This new `ProcessedTree<'t>` is also easily navigated with functions to manipulate its contents, as well as a way to view statements and determine their return types.

This stage will also include information about functions not defined in the module, and mark them as imported. This stage gets information from the invoker about other modules that are being processed, and will receive a list of functions and their argument/return types to validate that they are being used correctly.

### 4-3. Infer types & fill missing data

***Crate: part of `qsc-processor`***

This stage will walk through the new `ProcessedTree<'t>` and create a new `FinalizedTree<'t>` type, which contains the abstract syntax tree in its final form.

This stage will find any empty `Option`s and fill them with the correct values, and will infer variable types based on statement contents and return types. It will also fill any missing types with `void`.

## 5. Mangler

***Crate: `qsc-mangler`***

The mangler will go through the `FinalizedTree<'t>` and rename functions and variable names to avoid conflicts, using a random string generator and a basic renaming scheme. For example, `path::to::function()` will become `__qsc_path_to__function__[random string]()`.

This returns the `FinalizedTree<'t>` with modified names.

## 6. Code generator

***Crate: `qsc-codegen`***

The code generator will use [cranelift](https://cranelift.dev/) to generate machine code based on the `FinalizedTree<'t>`.

This has two backends:

- JIT: Just-in-time compiler, returns bytecode that is converted into a function and executed at runtime.
- AOT: Ahead-of-time compiler, emits and object file.

Both of these use the same code generation functions, but with different `struct`s and `trait` implementations.

## 7. Linker

***Crate: `qsc-linker`***

**NOTE:** This will only be triggered if the compiler is in AOT mode and the user has not told the compiler to emit a raw object file.

This will take the code generator's object file and write it to a temporary file, and emit the linked file using the system's linker. This can either statically* or dynamically link.

\* Static linking is currently not functional.
