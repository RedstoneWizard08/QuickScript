fn do_math(a: i32, b: i32) -> i32 {
    let v: i32 = a - b;

    return v;
}

fn main() -> i32 {
    let a: i32 = 4;
    let b: i32 = 2;
    let val: i32 = do_math(a, b);

    puts("Hello, world!");
    puts("Another test!");
    printf("Math: %i - %i = %i\n", a, b, val);

    return 0;
}
