use std::fmt::Display;

struct A {
    i: i32,
}

impl Display for A {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.i)
    }
}

fn main() {
    println!("Hello, world!");
    let a = A { i: 4};
    println!("{}", a);
}
