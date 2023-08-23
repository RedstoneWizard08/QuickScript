# qsc: The QuickScript compiler.

QuickScript is a language that I am creating to learn how to, well, create a language.
The syntax is very similar to Rust, and it is a semicolon-based language, or at least it will be (I think). The compiler is written in Rust, and will translate into Assembly, which will then be compiled with either nasm or GNU as (probably the latter), and then linked with ld (I think I might use mold for it actually) internally. The standard library should hopefully be pretty good, but I haven't gotten around to that yet.

## License

This project is licensed under the MIT license. Feel free to use the code for whatever you want.

## Usage

Currently you just run `cargo run [file]`, where `[file]` represents the path to the input file. I know, very complex.

If you want to use the code as a library, feel free, I tried to document most of it. I may have forgotten some though, or I may just stop out of laziness later. :p
