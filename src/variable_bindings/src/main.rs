/// Rust provides type safety via static typing. Variable bindings can be type annoted when declared.
/// However, in most cases, the compiler will be able to infer the type of the variable from the
/// context, heavily reducing the annotation burden
///
/// Values, like literals, can be bound to variables using the `let` binding
use variable_bindings::*;
fn main() {
    println!("Hello, world!");
    let an_int = 1u8;
    let a_bool = true;
    let unit = ();

    // copy `an_int` into `copied_int`
    let copied_int = an_int;

    println!("An integer: {}", copied_int);
    println!("A boolean: {}", a_bool);
    println!("Meet the unit value: {:?}", unit);

    // The compiler warns about unused variable bindings; these warnings can be silenced by prefixing
    // the variable name with an underscor `_`
    let _unused_var = 3u32;

    let _noisy_unused_variable = 2u8;

    println!("----------main method from mutability.rs----------");
    mutability::mut_main();

    println!("----------main method from scope_and_shadowing.rs----------");
    scope_and_shadowing::scope_main();
    scope_and_shadowing::shadowing_main();

    println!("----------main method from declarations.rs----------");
    declarations::declare_main();

    println!("----------main method from freezing.rs----------");
    freezing::freeze_main();
}


