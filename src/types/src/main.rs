/// Rust provides several mechanisms to change or define the type of primitives and user defined
/// types.
use types::*;
fn main() {
    println!("Hello, world!");

    println!("----------main method from casting.rs----------");
    casting::casting_main();

    println!("----------main method from literals.rs----------");
    literals::literal_main();

    println!("----------main method from inference.rs----------");
    inference::inference_main();

    println!("----------main method from aliasing.rs----------");
    aliasing::aliasing_main();
}
