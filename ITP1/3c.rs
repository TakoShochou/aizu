use std::io::prelude::*;
use std::cmp;

fn main () {
  let stdin = std::io::stdin();
  for line in stdin.lock().lines() {
    let xs = line.unwrap();
    if xs == "0 0" { break }
    let mut iter = xs.split_whitespace();
    let a:i32 = iter.next().unwrap().parse().unwrap();
    let b:i32 = iter.next().unwrap().parse().unwrap();
    println!("{} {}", cmp::min(a, b), cmp::max(a, b));
  }
}