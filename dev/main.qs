fn get_name() -> str {
    return "world";
}

fn do_math(a: i32, b: i32) -> i32 {
    let v = a - b;

    return v;
}

fn main() -> i32 {
    let a = 4;
    let b = 2;
    let val: i32 = do_math(a, b);

    printf("Hello, %s!\n", get_name());
    puts("Another test!");
    printf("Math: %i - %i = %i\n", a, b, do_math(a, b));

    return 0;
}
