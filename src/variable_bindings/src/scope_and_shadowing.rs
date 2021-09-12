/// Variable bindings have a scope and are constrained to live in a *block*.
/// A block is a collection of statements enclosed by `{}`

pub fn scope_main() {
    // This binding lives in the main function
    let long_lived_binding = 1;

    // This is a block and has a smaller scope than the main function
    {
        // This binding only exists in this block
        let short_lived_binding = 2;

        println!("Inner short binding: {}", short_lived_binding);
    }
    // End of block


    // This is an error. As the code is not in the same scope as the `short_lived_binding`
    // println!("Outer short binding: {}", short_lived_binding);

    println!("Outer long binding: {}", long_lived_binding);
}

pub fn shadowing_main() {
    let shadowed_binding = 1;

    {
        println!("Before being shadowed....{}", shadowed_binding);

        // This binding *shadows* the outer one
        let shadowed_binding = "abd";

        println!("After being shadowed....{}", shadowed_binding);
    }
    println!("Outside inner block: {}", shadowed_binding);

    // this shadows the previous binding
    let shadowed_binding = "abc";
    println!("Shadowed the outer block: {}", shadowed_binding);

}