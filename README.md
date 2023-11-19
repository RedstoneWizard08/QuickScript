# qsc: The QuickScript compiler.

![Crates.io](https://img.shields.io/crates/v/quickscript?style=flat-square)
![Crates.io Downloads](https://img.shields.io/crates/d/quickscript?style=flat-square&label=downloads%20(crates.io))
![GitHub Downloads](https://img.shields.io/github/downloads/RedstoneWizard08/QuickScript/total?style=flat-square&label=downloads%20(GitHub)&color=red)
![Build status](https://img.shields.io/github/actions/workflow/status/RedstoneWizard08/QuickScript/build.yml?style=flat-square)

[![build](https://github.com/RedstoneWizard08/QuickScript/actions/workflows/build.yml/badge.svg)](https://nightly.link/RedstoneWizard08/QuickScript/workflows/build/main/binaries)

QuickScript is a language that I am creating to learn how to, well, create a language.
The syntax is very similar to Rust, and it is a semicolon-based language, or at least it will be (I think). The compiler is written in Rust, and will translate into Assembly, which will then be compiled with either nasm or GNU as (probably the latter), and then linked with ld (I think I might use mold for it actually) internally. The standard library should hopefully be pretty good, but I haven't gotten around to that yet.

## License

This project is licensed under the MIT license. Feel free to use the code for whatever you want.

## Installation

Installing QuickScript is incredibly easy!
Here's how you can do it:

**With Cargo:**

```sh
cargo install quickscript
```

**Via GitHub Releases:**

1. Head to https://github.com/RedstoneWizard08/QuickScript/releases
2. Download the correct binary for your platform
3. Run `chmod +x [path to binary here]`
4. Run the binary!

## Usage

For usage details, run `qsc --help`

## Support

Currently, here is the support matrix:

| Version | i686 | x86_64 | armv7l | armhf | arm64 |
| ------- | ---- | ------ | ------ | ----- | ----- |
|  0.0.0  | 🔴 | 🔴 | 🔴 | 🔴 | 🟢 |
|  0.1.0  | 🔴 | 🟡 | 🔴 | 🔴 | 🟢 |
|  0.3.0  | 🟢 | 🟢 | 🔴 | 🔴 | 🟢 |
|  0.4.0  | 🟢 | 🟢 | 🔴 | 🔴 | 🟢 |
|  0.5.x  | 🟢 | 🟢 | 🔴 | 🔴 | 🟢 |

Note that armv7l and armhf support is dependent on Cranelift's support for it.
See [cranelift#1173](https://github.com/bytecodealliance/wasmtime/issues/1173)
for more details.

Musl builds for i686 are also not supported due to a big in cargo zigbuild.
See [cargo-zigbuild#96](https://github.com/rust-cross/cargo-zigbuild/issues/96)
for more details.

# Credits

Huge thanks to [Pixeled (@orosmatthew)](https://github.com/orosmatthew) and his
[Creating a Compiler](https://www.youtube.com/playlist?list=PLUDlas_Zy_qC7c5tCgTMYq2idyyT241qs)
series, as it helped me think about how to do this best and it gave me the
inspiration for this project.

Another huge thanks for the [cranelift-jit-demo](https://github.com/bytecodealliance/cranelift-jit-demo),
as it helped me implement the code generator with Cranelift.
