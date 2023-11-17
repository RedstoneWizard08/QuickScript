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
