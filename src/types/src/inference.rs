/// The type inference engine is pretty smart. It does more than looking at the type of the value
/// expression during an initialization. It also looks at how the variable is used afterwards to
/// infer its type. Here's an advanced example of type inference:
pub fn inference_main() {
    // Because of the annotation, the compiler knows that `elem` has the type of `u8`
    let elem = 5_u8;

    // Create an empty vector (a growable array)
    let mut vec = Vec::new();
    // At this point the compiler doesn't know the exact type of `Vec`, it just know that it's a
    // vector of something (`Vec<_>`)

    // Insert `elem` in the vector
    vec.push(elem);
    // Now the compiler knows that `vec` is a vector of `u8s` (`Vec<u8>`)

    println!("{:?}", vec);
}