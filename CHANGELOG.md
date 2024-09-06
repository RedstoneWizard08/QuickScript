# v0.6.0

Entirely rewrote the entire compilation backend!
Added JIT support!
Made a new CLI!
Many debugging functions!
Completely new lexer!
More stable syntax!

# v0.5.5

Fixed `cranelift_object::backend` being shown in logs.

# v0.5.4

Fixed commit hash build errors.

# v0.5.3

Updated all dependencies.

# v0.5.2

Fixed Android (Termux) support.

# v0.5.1

This release just fixes a few bugs.

- Watch mode now works.
- Fixed default verbosity level.

# v0.5.0

This release brings a ton of new features!

- A new CLI design.
- A new compile subcommand.
- A JIT run mode.
- Complete refactor of all CLI logic.
- Verbose mode.
- Integration with the `log` crate.
- A new watch mode (currently broken, will be fixed in 0.5.1)!

# v0.4.1

A quick patch that doesn't merit a full release:

- Fixed `--strip` defaulting to `true`.
- Fixed compiler not recognizing variables.
- Fixed issues with the linker.
- Fixed some compiler finnicky-ness.
- Hopefully fixed android support?

# v0.4.0

This release brings many new features, from JIT to targeting, and more!

- Added JIT mode
- Worked on basic strip functionality
- Added VCode and ASM dumping
- Fixed some utilities and arguments
- Fixed linker problems

Known issues:

- Compiler can be very finnicky.

# v0.3.0

Version 0.3.0! This release brings many changes, including a new codegen backend, a new tokenizer, parser, and general stability improvements.

- Switched from manual ASM to Cranelift codegen
- Created a new character-based tokenizer
- Updated parser to use a more modular design
- Performance upgrades
- Removed unnecessary dependencies

# v0.2.0

Changes:

- Optimize binary for size
- Add print function
- Improve parsing method
- Improve the linking method
- Improve assembling method
- Reduce duplicated code
- Work on multi-platform support

# v0.1.0

The first working release!
