//! !! IMPORTANT !!
//!
//! This program requires ncurses to run!
//!
//! During compilation, add `-a=-lncurses` to your compiler command
//! and make sure that ncurses is installed on your system for this
//! to work!

extern fn initscr();
extern fn endwin();

extern fn getch() -> char;
extern fn getchar() -> char;

fn get_name() -> str {
    return "world";
}

fn do_math(a: i32, b: i32) -> i32 {
    let v = a - b;

    return v;
}

fn get_hi() -> bool {
    initscr();

    let h = getch();
    let i = getch();
    let e = getch();

    let rh = 'h';
    let ri = 'i';
    let re = '!';

    endwin();

    if h == rh {
        if i == ri {
            if e == re {
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
