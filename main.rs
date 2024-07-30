use std::io;
use std::io::prelude::*;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)
    ap();
    let n: i32 = buffer.trim().parse().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut a: Vec<i32> = buffer
}