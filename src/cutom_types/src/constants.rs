/// Rust has two different types of constants which can be declared in any scope...including global.
/// Both require explicit type annotation:
/// * `const`: an unchangeable value (the common case)
/// * `static`: a possibly mutable (`mut`) variable with a `'static` lifetime. The `'static` lifetime
/// is inferred and does not have to be specified. Accessing or modifying a mutable static variable
/// is `unsafe`
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Access the constant in some function
    n > THRESHOLD
}

pub fn constant_main() {
    let n = 16;

    // Access constants in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}",
             n,
             if is_big(n) {
                 "big"
             }
             else {
                 "small"
             }
    );

    // CANNOT MODIFY A CONST
    // THRESHOLD = 5;
}