/// There are three types of structures (`structs`) that can be created using `struct`
/// * Tuple structs: basically, named tuples
/// * The classic C struct
/// * Unit structs: which are field-less, are useful for generics

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right corners are in space
    top_left: Point,
    bottom_right: Point
}

impl Rectangle {

    fn area(&mut self) -> f32 {
        let Point{ x: x1, y: y1} = self.top_left;
        let Point{ x: x2, y: y2 } = self.bottom_right;

        let difference_x = x2 - x1;
        let difference_y  = y1- y2;

        difference_x * difference_y
    }
}

fn square(point: Point, dimension: f32) -> Rectangle {
    let x1 = point.x;
    let y1 = point.y;

    Rectangle {
        top_left: Point { x: x1, y: y1 + dimension },
        bottom_right: Point { x: x1 + dimension, y: y1 }
    }
}

pub fn struct_main() {
    // Create a struct with field init shorthand
    let name = String::from("Peter");
    let age = 25;
    let peter = Person { name, age };

    // Print the struct via Debug
    println!("{:#?}", peter);

    // Instantiate a `Point`
    let point: Point = Point{ x: 10.3, y: 0.5 };

    // Access the fields of the point
    println!("Point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our other point
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right` will be the same as `point.y` because we used that field from `point`
    println!("Second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge} = point;

    let _rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("Pair contents: {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("Pair contents: {:?} and {:?}", integer, decimal);

    let mut rect1 = Rectangle {
        top_left: Point { x: 0.0, y: 5.0 },
        bottom_right: Point { x: 5.0, y: 0.0 }
    };

    println!("Rectangle coordinates: {:#?}", rect1);

    println!("Rectangle Area: {}", rect1.area());

    let mut rect_out = square(
        Point {x: 1.0, y: 1.0 },
        2.0
    );
}
