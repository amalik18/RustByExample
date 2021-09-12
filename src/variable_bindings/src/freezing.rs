/// When data is bound by the same name immutably, it also `freezes`. Frozen data can't be modified
/// until the immutable binding goes out of scope.
pub fn freeze_main() {
    let mut _mutable_integer = 7u8;

    {
        // Shadowing the mutable binding with an immutable `_mutable_integer`
        let _mutable_integer = _mutable_integer;

        // Error! `_mutable_integer` is frozen in this scope
        // Exact error: error[E0384]: cannot assign twice to immutable variable `_mutable_integer`
        // _mutable_integer = 50;

        // the immutable `_mutable_integer` goes out of scope
    }
    _mutable_integer = 50;
    println!("Mutable integer: {}", _mutable_integer);
}