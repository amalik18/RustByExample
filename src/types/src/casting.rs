#![allow(overflowing_literals)]
/// Rust provides no implicit type conversion (coercion) between primitive types.
/// But, explicit type conversion (casting) can be performed using the `as` keyword
///
/// Rules for converting between integral types, generally, follow C conventions, except in cases
/// where C has undefined behavior. The behavior of all casts between integral types is well
/// defined in Rust.
pub fn casting_main() {
    let decimal = 65.4321_f32;

    // Error: No implicit conversion
    // Exact error: error[E0308]: mismatched types
    // let integer: u8 = decimal;

    // Explicit conversion
    let integer = decimal as u8;
    let character = integer as char;

    // Error!
    // There are limits to explicit conversion...a `float` cannot be directly converted into a `char`
    // Exact error: error[E0604]: only `u8` can be cast as `char`, not `f32`
    // let character = decimal as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // When casting any value to an unsigned type, T,
    // T::MAX + 1 is added or subtracted until the value fits into the new type

    // 1000 already fits in a u16
    println!("1000 as a u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    // Under the hood, the first 8 LSBs (Least Significant Bits) are kept
    // and the rest towards the MSB (Most Significant Bit) get truncated
    println!("1000 as a u8 is: {}", 1000 as u8);

    // -1 + 256 = 255
    println!("-1 as a u8 is: {}", (-1i8) as u8);

    // For positive numbers, this is the same as modulus `%`
    println!("1000 mod 256 is: {}", 1000 % 256);

    // When casting to a signed type, the bitwise result is the same as first casting to the
    // corresponding unsigned type. If the most significant bit of that value is 1, then the
    // value is negative.

    // Unless it already fits of course.
    println!("128 as a i16 is: {}", 128 as i16);

    // 128 as u8 -> 128 whose 2's complement in 8 bits is:
    println!("128 as an i8 is: {}", 128 as i8);

    // Repeating the example above
    // 1000 as u8 -> 232
    println!("1000 as u8: {}", 1000 as u8);
    // and the 2's complement of 232 is -24
    println!("232 as i8 is: {}", 232 as i8);

    // Since Rust 1.45, the `as` keyword performs a *saturating cast* when casting from a `float` to
    // an `int`. If the floating point value exceeds the upper bound or is less than the lower bound,
    // the returned value will be equal to the bound crossed

    // 300.0 is as u8 is 255
    println!("300.0 as a u8: {}", 300.0_f32 as u8);

    // -100.0 as u8 is 0
    println!("-100.0 as a u8: {}", -100.0_f32 as u8);

    // NaN as a u8 is 0
    println!("NaN as a u8: {}", f32::NAN as u8);

    // This behavior incurs a small runtime cost and can be avoided with `unsafe` methods, however
    // the results might overflow and return **unsound values**. Use wisely
    unsafe {
        // 300.0 is 44
        println!("300.0 as u8: {}", 300.0_f32.to_int_unchecked::<u8>());

        // -100.0 as u8 is 156
        println!("-100.0 as u8 is: {}", (-100.0_f32).to_int_unchecked::<u8>());

        // NaN as a u8 is 0
        println!("NaN as a u8 is: {}", f32::NAN.to_int_unchecked::<u8>());
    }
}