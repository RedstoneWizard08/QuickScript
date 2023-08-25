# qsc: The QuickScript compiler.

[![build](https://github.com/RedstoneWizard08/QuickScript/actions/workflows/build.yml/badge.svg)](https://nightly.link/RedstoneWizard08/QuickScript/workflows/build/main/binaries)

QuickScript is a language that I am creating to learn how to, well, create a language.
The syntax is very similar to Rust, and it is a semicolon-based language, or at least it will be (I think). The compiler is written in Rust, and will translate into Assembly, which will then be compiled with either nasm or GNU as (probably the latter), and then linked with ld (I think I might use mold for it actually) internally. The standard library should hopefully be pretty good, but I haven't gotten around to that yet.

## License

This project is licensed under the MIT license. Feel free to use the code for whatever you want.

## Usage

Currently you just run `cargo run [file]`, where `[file]` represents the path to the input file. I know, very complex. For any other options, use `--help` or `-h`, as the CLI is built with Clap.

If you want to use the code as a library, feel free, I tried to document most of it. I may have forgotten some though, or I may just stop out of laziness later. :p (I will likely get to documenting it later anyway.)

# Support

Currently, here is the support matrix:

| Version | x86 (32-bit) | x86_64 (64-bit) | arm (32-bit) | arm (64-bit) |
| ------- | ------------ | --------------- | ------------ | ------------ |
|  0.0.0  | 🔴 | 🔴 | 🔴 | 🟢 |
|  0.1.0  | 🔴 | 🟢 | 🔴 | 🟢 |

# Credits

Huge thanks to [Pixeled (@orosmatthew)](https://github.com/orosmatthew) and his [Creating a Compiler](https://www.youtube.com/playlist?list=PLUDlas_Zy_qC7c5tCgTMYq2idyyT241qs) series, as it helped me think about how to do this best and it gave me the inspiration for this project.
