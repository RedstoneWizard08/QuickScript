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
    let val = do_math(a, b);

    if val == 3 {
        printf("It works!\n");

        return 0;
    } else {
        printf("It wasn't 3...\n");
    }

    printf("Hello, %s!\n", get_name());
    puts("Another test!");
    printf("Math: %i - %i = %i\n", a, b, do_math(a, b));

    return 0;
}
