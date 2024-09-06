#include <curses.h>
#include <stdio.h>

char* get_name() {
    return "world";
}

int do_math(int a, int b) {
    int v = a - b;

    return v;
}

bool get_hi() {
    initscr();

    char h = getch();
    char i = getch();
    char e = getch();

    char rh = 'h';
    char ri = 'i';
    char re = '!';

    endwin();

    if (h == rh) {
        if (i == ri) {
            if (e == re) {
                return true;
            }
        }
    }

    return false;
}

int main() {
    int a = 4;
    int b = 2;
    int val = do_math(a, b);

    if (get_hi()) {
        printf("Well, hello to you too!\n");
    } else {
        printf("So rude! You didn't say hi!\n");
    }

    if (val == 3) {
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
