use std::fmt::{ Display, Result, Formatter };

struct List(Vec<i32>);

impl Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let vec = &self.0;
        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", count, v)?;
        }
        write!(f, "]")
    }
}

pub fn list_main() {
    let v1 = List(vec![1, 2, 3, 4]);
    println!("{}", v1);
}