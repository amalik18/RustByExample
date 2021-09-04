use std::mem;

/// ***Arrays***
/// An array is a collection of objects of the same type `T`, stored in contiguous memory.
/// Arrays are created using brackets `[]`, and their length, which is known at compile, is part of
/// their type signature `[T; length]`

/// ***Slices***
/// Slices are similar to arrays, but their length is NOT known at compile time. Instead, a slice is
/// a two-word object, the first word is a pointer to the data and the second word is the length of
/// the slice. The word size is the same as `usize`, determined by the processor architecture: e.g.
/// 64 bits on an x86_64. Slices can be used to borrow a section of an array, and have the type
/// signature `&[T]`

fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

pub fn array_main() {
    // Fixed-size array, the type signature is not necessary
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be instantiated to the same value
    let ys: [i32; 5] = [10; 5];

    // Indexing starts at 0
    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);

    // `len` returns the count of elements in the array
    println!("Number of elements in the array: {}", xs.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices
    println!("Borrow the whole array as a slice");
    analyze_slice(&xs);

    // Slices can point to a section of an array
    // They are of the form [starting_index..ending_index]
    // starting_index is the first position in the slice
    // ending_index is one more than the last position in the slice
    println!("Borrow a section of the array as a slice");
    analyze_slice(&ys[1..5]);

    // Out of bound indexing causes compile error
    println!("{}", xs[4]);
}