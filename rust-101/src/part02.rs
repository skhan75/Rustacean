/**
 * Generics or Polymorphism
 * In the previous examples, we had to hard code the type i32, what if we want CharOrNothing or
 * FloatOrNothing
 * In this example, we are going to define a generic type SomethingOrNothing
 */

pub enum SomethingOrNothing<T> {
    Something(T),
    Nothing,
}

pub use self::SomethingOrNothing::*;
type NumberOrNothing = SomethingOrNothing<i32>;

// ## Generic `impl`, Static functions 
// Inside an `impl`, `Self` refers to the type we are implmenting things for. Here, it is an alias
// for `SomethingOrNothing`
impl<T> SomethingOrNothing<T> {
    fn new(o: Option<T>) -> Self {
        match o {
            None => Nothing,
            Some(t) => Something(t)
        }
    }

    fn to_option(self) -> Option<T> {
        match self {
            Nothing => None,
            Something(t) => Some(t)
        }
    }
}

// You can call static functions, and in particular constructors, as demonstrated below
// new is the rust convention for defining constructors
fn call_constructor(x: i32) -> SomethingOrNothing<i32> {
    SomethingOrNothing::new(Some(x))
}

// ## Traits
pub trait Minimum: Copy {
    fn min(self, b: Self) -> Self;
}

// We write vec_min as a generic function over a type T that we demand to satisfy the Minimum
// trait. This requirement is called `trait bound`. Rust automatically figures out that e is of
// type T, which implements the Minimum trait and hence we can call the function
pub fn vec_min<T: Minimum>(v: Vec<T>) -> SomethingOrNothing<T> {
    let mut min = Nothing;
    for e in v {
        min = Something(match min {
            Nothing => e,
            Something(n) => {
                e.min(n)
            }
        });
    }

    min
}

// TO make `vec_min` usable with `Vec<i32>`, we implement the Minimum trait for i32.
impl Minimum for i32 {
    fn min(self, b: Self) -> Self {
        if self < b { self } else { b }
    }
}

// We again provide the print function. This also shows that we can have multiple `impl` blocks for
// the same type 
// `NumberOrNothing` is just a type alias for `SomethingOrNothing` and we can provide some methods
// only for certain instances of generic type.
impl NumberOrNothing {
    pub fn print(self) {
        match self {
            Nothing => println!("The number is: <nothing>"),
            Something(n) => println!("The number is: {}", n),
        }
    }
}

fn read_vec() -> Vec<i32> {
    vec![18, 5, 7, 1, 9, 27]
}

pub fn main() {
    let vec = read_vec();
    let min = vec_min(vec);
    min.print();
}
