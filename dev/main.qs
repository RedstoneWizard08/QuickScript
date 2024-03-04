fn do_math(a: i32, b: i32) -> i32 {
    let v: i32 = a - b;

    return v;
}

fn main() -> i32 {
    let val: i32 = do_math(4, 2);

    puts("Hello, world!");
    puts("Another test!");
    printf("Math: %i\n", val);

    return 0;
}
