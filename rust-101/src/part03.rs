use std::io::prelude::*;
use std::io;

use crate::part02::vec_min;

fn read_vec() -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::<i32>::new();

    let stdin = io::stdin();
    println!("Enter a list of numbers; one per line"); 

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        match line.trim().parse::<i32>() {
            Ok(num) => {
                vec.push(num)
            },
            Err(_) => {
                println!("What did I say about numbers?")
            },
        }
    }
    vec
}


pub fn main() {
    let vec = read_vec();
    let min = vec_min(vec);
    min.print();
}
