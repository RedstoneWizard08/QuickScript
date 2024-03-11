fn get_name() -> str {
    return "world";
}

fn do_math(a: i32, b: i32) -> i32 {
    let v = a - b;

    return v;
}

fn get_hi() -> bool {
    let h = getch();
    let i = getch();
    let e = getchar();

    printf("Input: H: %c | I: %c | E: %c\n", h, i, e);

    if h == 'h' {
        if i == 'i' {
            if e == '!' {
                return true;
            }
        }
    }
    
    return false;
}

fn main() -> i32 {
    let a = 4;
    let b = 2;
    let val = do_math(a, b);

    if get_hi() {
        printf("Well, hello to you too!\n");
    } else {
        printf("So rude! You didn't say hi!\n");
    }

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
