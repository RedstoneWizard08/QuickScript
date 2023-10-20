fn main(args: Array<str>) -> i32 {
    print("Hello, world!");
    print("Another test!");

    if (args.len() >= 2) {
        let name = args[1];

        print("Hello to {} in particular!", name);
    }

    return 8;
}
