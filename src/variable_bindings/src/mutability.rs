/// Variable bindings are immutable by default, but this can be overwritten using the `mut`
/// modifier
pub fn mut_main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 2;

    println!("Before mutation: {}", mutable_binding);

    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // Error
    // _immutable_binding += 1;
}