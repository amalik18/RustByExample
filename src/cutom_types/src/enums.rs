use crate::enums::List::{Cons, Nil};

/// The `enum` keyword allows the creation of a type which may be one of a few different variants.
/// Any variant which is valid as a `struct` is also valid as an `enum`

// Create an `enum` to classify web events. Note how both names and type information together
// specify the variant: `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.
enum WebEvent {
    // An `enum` may either be a `unit` struct
    PageLoad,
    PageUnload,

    // Like a tuple struct
    KeyPress(char),
    Paste(String),

    // Or like a C-like struct
    Click { x: i64, y: i64 },
}

// A function which takes a `WebEvent` enum as an argument and returns nothing
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page Loaded"),
        WebEvent::PageUnload => println!("Page unloaded"),
        WebEvent::KeyPress(c) => println!("Pressed '{}'.", c),
        WebEvent::Paste(s) => println!("Pasted \"{}\".", s),
        WebEvent::Click { x, y } => {
            println!("Clicked at x={}, y={}", x, y);
        },
    }
}

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

/// Type aliases allow you to alias an `enum` and thus allowing you to refer to the `enum` using
/// alias.
// Creates a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

/// `enum` can also be used as the classic C-like enums

// `enum` with implicit discriminator (starts at 0)
enum Number {
    Zero,
    One,
    Two,
}

// `enum` with explicit discriminator
enum Color {
    Red = 0xff0000,
    Blue = 0x0000ff,
    Green = 0x00ff00,
}

enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: a node that signifies the end of the linked list
    Nil,
}

impl List {
    // Create an empty list
    fn new() -> List {
        // `Nil` has type `List`
        Nil
    }

    // Consume a list, and return the same list with a new element at its front
    fn prepend(self, elem: u32) -> List {
        // `Cons` also has type List
        Cons(elem, Box::new(self))
    }

    // Return the length of the list
    fn len(&self) -> u32 {
        // `self` has to matched, because the behavior of this method depends on the variant of `self`
        // `self` has type `&List` and `*self` has type `List`, matching on a concrete type `T` is
        // preferred over a match on a reference `&T`

        // After Rust 2018 you can use `self` and `tail` here (with no `ref`) below as well,
        // rust will infer `&self` and `ref tail`
        match *self {
            // Can't take ownership of the tail, because `self` is borrowed
            // instead take a reference to `tail`
            Cons(_, ref tail) => 1 + tail.len(),
            // Base case: An empty list has zero length
            Nil => 0
        }
    }

    // Return representation of the list as a (heap allocated) string
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` is similar to `print!`, but returns a heap allocated string
                // instead of printing to the console
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            }
        }
    }
}

pub fn enum_main() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a String slice
    let pasted = WebEvent::Paste("Sample Test".to_owned());
    let click = WebEvent::Click { x: 21, y: 82 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    let x = Operations::Add;

    println!("{}", x.run(3, 4));

    // Explicitly `use` each name so they are available without manual scoping
    use crate::enums::Status::{ Poor, Rich };
    // Automatically `use` each name inside `Work`
    use crate::enums::Work::*;

    // Equivalent to `Status::Poor`
    let status = Poor;
    // Equivalent to `Work::Civilian`
    let work = Civilian;

    match status {
        // Note the lack of scoping because of the explicit `use` above
        Rich => println!("The rich have lots of moeny!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        // Note again the lack of scoping
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }

    // `enums` can be cast as integers
    println!("Zero is {}", Number::Zero as i32);
    println!("One is {}", Number::One as i32);

    println!("Roses are #{:06x}", Color::Red as i32);
    println!("Violets are #{:06x}", Color::Blue as i32);

    // create an empty linked list
    let mut linked_list = List::new();

    // prepend some elements
    linked_list = linked_list.prepend(1);
    linked_list = linked_list.prepend(2);
    linked_list = linked_list.prepend(3);

    // Show the final state of the list
    println!("Linked list has length: {}", linked_list.len());
    println!("{}", linked_list.stringify());
}