#[doc(inline)]
// This is a comment, and is ignored by the compiler
// You can test this code by clicking the "Run" button over there ->
// or if you prefer to use your keyboard, you can use the "Ctrl + Enter" shortcut
/*
 * This is a block comment
 */
// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->

// This is the main function
/// Hello, this is the first project I've worked on
/// This is the entry to the Rust programming language.
/// These comments were generated using three slashes.
use std::f64::consts::PI;
use hello_world::debug::debug_main;
use hello_world::display::main_display;
use hello_world::display_list::list_main;
use hello_world::formatting::format_main;
fn main() {
    // Using `println` to print to the console (io::stdout) and `eprintln` to print to
    // the standard error (io::stderr)
    println!("Hello World!");
    eprintln!("Hello World!");

    // In general the `{}` will be replaced with stringified versions of the arguments
    // Without a type identified 22 is assumed to be an i32, you can change this by providing the
    // type as such 22i64, which would now mean that 22 is an i64.
    println!("I am {} years old", 22);
    eprintln!("I am {} years old", 22);

    // This stringification works with many patterns, such as positional argument.
    println!(
        "This is position 0: {0}, this is position 1: {1}, in the arguments passed",
        10, 20
    );
    eprintln!(
        "This is position 0: {0}, this is position 1: {1}, in the arguments passed",
        10, 20
    );

    // It also works with named arguments
    println!(
        "My name is {name}, I'm {age} years old!",
        name = "Ali",
        age = 24
    );
    eprintln!(
        "My name is {name}, I'm {age} years old!",
        name = "Ali",
        age = 24
    );

    // Special formatting can be done with `:`
    println!("{} of {:b} people know binary, the other half don't", 1, 5);
    eprintln!("{} of {:b} people know binary, the other half don't", 1, 5);

    // You can right-align text with a specified width. This will output "     1".
    // 5 white spaces and a `1`. A 6 character width string, 5 spaces and the number 1
    println!("{number:>width$}", number = 1, width = 6);
    eprintln!("{number:>width$}", number = 1, width = 6);

    println!("My name is {0}, {1} {0}", "Bond", "James");

    println!("{name} is roughly {pi:.6}", name="Pi", pi=PI);

    println!("----------main method from debug.rs----------");
    debug_main();

    println!("----------main method from display.rs----------");
    main_display();

    println!("----------main method from display_list.rs----------");
    list_main();

    println!("----------main method from formatting.rs----------");
    format_main();
}
