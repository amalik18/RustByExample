/// `()` is the Unit type whose only valuable is an empty tuple
use primitives::literals_operators::literal_main;
use primitives::tuples::tuple_main;
use primitives::arrays_and_slices::array_main;
fn main() {

    // Variables can be type annotated
    let logical: bool = true;

    let a_float: f32 = 1.0; // regular annotation
    let an_int = 32u32; // suffix annotation

    // if annotation is not provided, default will be used
    let default_float = 1.3; // `f64`
    let default_int = 7; // `i32`

    // a type can also be inferred from context
    let mut inferred_type = 12; // i64 is inferred from another line
    inferred_type = 42949683242324i64;

    // a mutable `mut` variables value can be changed.
    let mut mutable = 12;
    mutable = 21;

    // Error! The type cannot be changed.
    // mutable = true;

    // variables can be overwritten with shadowing
    let mutable = true;

    println!("----------main method from literals_operators.rs----------");
    literal_main();

    println!("----------main method from tuples.rs----------");
    tuple_main();

    println!("----------main method from arrays_and_slices.rs----------");
    array_main();

}
