// This code is an enhancement of part00.rs to make it more prettier. Because Rust is an
// "expression-based" language, which means that most of the terms you write down are not just
// statements(executing code), but expressions(returning a value). This applies even to the body of
// entire functions!.
enum NumberOrNothing {
    Number(i32),
    Nothing 
}

impl NumberOrNothing {
    // This is called an Inherent Implementation in Rust. This is because Rust separates code from
    // data, so the definition of the methods on a `enum` (and also on `struct`) is independent of
    // the definition of the type.
    fn print(self) {
        match self {
            Nothing => println!("The number is: <nothing>"),
            Number(n) => println!("The number is: {}", n),
        }
    }
}

use self::NumberOrNothing::{Number, Nothing};
fn vec_min(v: Vec<i32>) -> NumberOrNothing {
    // Rust allows to define helper functions inside other functions.
    // In the context of namespacing, the inner function has no access to the data of the outer
    // one. 
    fn min_32(a: i32, b: i32) -> i32 {
        if a < b { a } else { b }
    }

    let mut min = Nothing;
    for e in v {
        min = Number(match min {
            Nothing => e,
            Number(n) => min_32(n, e)
        });
    }

    min
}

fn read_vec() -> Vec<i32> {
    vec![18, 5, 7, 2, 9, 27]
}

pub fn main() {
    let vec = read_vec();
    let min = vec_min(vec);
    min.print();
}
