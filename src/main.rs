#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufWriter, ErrorKind, Write};

fn main() {
    let mut input: String = String::new();
    // io::stdin().read_line(&mut input).expect("failed to read");
    // println!("Entered Sentence is {}", input);
    let x = 2;
    let y = x;
    println!("{}", x);
    let random_num: i32 = rand::thread_rng().gen_range(1..101);
    println!("Entered Sentence is {}", random_num);

    let mut hell = String::from("value");
    println!("Entered Sentence is {}", hell);
    for val in hell.as_bytes() {
        print!("{} ", val);
    }
    println!();
}
