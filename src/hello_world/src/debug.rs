use std::fmt;

#[doc(inline)]
/// This structure cannot be printed with `fmt::Display` or `fmt::Debug`
struct UnPrintable(i32);

/// The `derive` attribute automatically creates the implementation required
/// to make this `struct` printable with `fmt::Debug`
#[derive(Debug)]
struct DebugPrintable(i32);

/// All `std` library types are automatically printable with `{:?}` too.

/// Derive the `fmt::Debug` implementation for `Structure`
/// `Structure` is a structure which contains a single `i32`
#[derive(Debug)]
struct Structure(i32);

/// Put `Structure` inside of another struct `Deep`. Make sure it derives `Debug` as well
#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

impl<'a> fmt::Display for Person<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "My name is {} and I am {} years old.", self.name, self.age)
    }
}

pub fn debug_main() {
    // Printing in {:?} is similar to printing with {}
    println!("{:?} months in a year", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
            "Slater",
            "Christian",
            actor="actor's");

    // `Structure` is printable in the same manner cause we derived the Debug implementation
    println!("Now {:?} will print as well!", Structure(3));

    // Problem with `derive` is, there is no control over how it will look at the end.
    // What if I wanted this to simply print `9`?
    println!("Now {:?} will print as well", Deep(Structure(8)));

    let name = "Peter";
    let age: u8 = 30;
    let peter = Person{ name, age };

    // Pretty printing
    println!("{:#?}", peter);

    println!("{}", peter);
}