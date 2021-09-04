/// Rust custom data types are formed mainly through the two keywords:
/// `struct`: define a structure
/// `enum`: define an enumeration
/// Constants can also be created via the `const` and `static`
use cutom_types::structures::struct_main;
use cutom_types::enums::enum_main;
use cutom_types::constants::constant_main;
fn main() {
    println!("Hello, world!");

    println!("----------main method from structures.rs----------");
    struct_main();

    println!("----------main method from enums.rs----------");
    enum_main();

    println!("----------main method from constants.rs----------");
    constant_main();
}
