fn main(args: Array<str>) {
    print("Hello, world!");
    print("Another test!");

    if (args.len() >= 2) {
        let name = args[1];

        print("Hello to {} in particular!", name);
    }

    exit 0;
}
