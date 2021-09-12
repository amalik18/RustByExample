/// Numeric literals can be type annotated by adding the type as a suffix.
/// As an example, to specify that the literal `42` should have the type `i32`, we write `42i32`
/// The type of unsuffixed numeric literals will depend on how they are used. If no constraint exists
/// the compiler will use `i32` for integers and `f64` for floating point numbers
///
/// Concepts that haven't been explained:
/// `std::mem::size_of_val` is a function, but it's called with it's *full path*. Code can be split
/// in logical units called *modules*. In this case, the `size_of_val` function is defined in the
/// `mem` module which is defined in the `std` crate.

pub fn literal_main() {
    // suffixed literals, their types are known at initialization
    let x = 1_u8;
    let y = 2_u32;
    let z = 3_f32;

    // Unsuffixed literals, their types depend on how they are used
    let i = 1;
    let f = 1.0;

    // `std::mem::size_of_val(&x)` returns the size of the variable in bytes
    println!("Size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("Size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("Size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("Size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("Size of `f` in bytes: {}", std::mem::size_of_val(&f));
}