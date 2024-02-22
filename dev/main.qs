fn get_argc(a: i32, b: i32) -> i32 {
    let v: i32 = a - b;

    return v;
}

fn main() -> i32 {
    let argc: i32 = get_argc(4, 2);

    puts("Hello, world!");
    puts("Another test!");
    printf("Args: %i\n", argc);

    return 0;
}
