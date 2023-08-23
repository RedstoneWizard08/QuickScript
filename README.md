# qsc: The QuickScript compiler.

QuickScript is a language that I am creating to learn how to, well, create a language.
The syntax is very similar to Rust, and it is a semicolon-based language, or at least it will be (I think). The compiler is written in Rust, and will translate into Assembly, which will then be compiled with either nasm or GNU as (probably the latter), and then linked with ld (I think I might use mold for it actually) internally. The standard library should hopefully be pretty good, but I haven't gotten around to that yet.
