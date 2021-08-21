use std::fmt::{Display, Formatter};

/// A tuple is a collection of values of differing types
/// Constructed using `()`, each tuple itself is a value with type signature `(T1, T2...)`
/// where `T1`, `T2` are the types of its members.
/// Functions can use tuples to return multiple values, as tuples can hold any number of values

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;

    // return (boolean, integer);
    (boolean, integer)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

pub fn tuple_main() {
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                                                -1i8, -2i16, -3i32, -4i64,
                                                0.1f32, 0.2f64,
                                                'a', "true", true);

    // Values can be extracted using tuple indexing
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // Tuples can be members of tuples
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // tuples are printable
    println!("tuple of tuples: {:#?}", tuple_of_tuples);

    // Debug not implemented for long tuples
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:#?}", too_long_tuple);

    let pair = (1, true);
    println!("pair is {:#?}", pair);

    // println!("Printing *pair: {:#?}", *pair);
    println!("Printing &pair: {:#?}", &pair);

    println!("The reversed pair is: {:#?}", reverse(pair));

    /// To create one element tuples a `,` is required to differentiate them from a literal
    println!("One element tuple: {:#?}", (5u32,));
    println!("Just an integer: {:#?}", (5u32));

    let tuple = (1, "hello", 4.5, true);

    let (int1, str1, float1, bool1) = tuple;
    println!("{:#?}, {:#?}, {:#?}, {:#?}", int1, str1, float1, bool1);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);

    println!("{}", matrix);

}