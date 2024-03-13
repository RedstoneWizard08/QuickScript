### v0.6.0 (SUBJECT TO CHANGE!)

- [x] Switch to [pest](https://pest.rs) instead of a custom tokenizer/lexer (this was before I decided to use [chumsky](https://github.com/zesterer/chumsky))
- [x] Overhaul the AST and lexer
- [x] Implement a basic processor (type checker & validator)
- [ ] Switch to using [chumsky](https://github.com/zesterer/chumsky) instead of [pest](https://pest.rs) to simplify the lexer and provide more useful errors
- [ ] Eliminate as many `clone` calls as possible
- [ ] Implement `for` and `while` loops
- [ ] Overhaul the processor to provide better errors
- [ ] Overhaul the codegen
    - [ ] Clean it up
    - [ ] Move local variables from the heap to the stack (using dynamic stack slots)
- [ ] Create a basic allocator
- [ ] Partial `libc` code (using `extern`s)
- [ ] Fix the LSP
- [ ] Make the VSCode extension work
- [ ] NeoVim support?
- [ ] Fix actions artifact uploads
- [ ] Self-updater? (Undecided)

### v0.7.0 (SUBJECT TO CHANGE!)

- [ ] Create a proper allocator
- [ ] Partial standard library
- [ ] Package manager
- [ ] Partial `libc` package
- [ ] Add attributes (ex: `@Attribute`)
- [ ] Add macros & `comptime`
- [ ] More to come!

### v0.8.0 (SUBJECT TO CHANGE!)

- [ ] Complete standard library
- [ ] Implement some useful tools
- [ ] Make better examples
- [ ] Add proper tests
