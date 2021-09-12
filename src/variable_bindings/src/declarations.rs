/// It's possible to declare a variable binding first and then initialize later.
/// However, this is seldom used as it may lead to the use of an uninitialized variable
/// The compiler forbids use of uninitialized variables as this would lead to undefined behavior.
pub fn declare_main() {
    // declare a variable binding
    let a_binding;

    {
        let x = 2;

        // initialize the binding
        a_binding = x * x;
    }

    println!("a_binding: {}", a_binding);

    let another_binding;

    // Error! Use of uninitialized variable
    // Exact error: error[E0381]: borrow of possibly-uninitialized variable: `another_binding`
    // println!("another_binding: {}", another_binding);

    another_binding = 1;

    println!("another_binding: {}", another_binding);
}