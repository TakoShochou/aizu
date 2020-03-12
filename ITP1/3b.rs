use std::io::prelude::*;

fn main () {
  let stdin = std::io::stdin();
  for (i, xs) in stdin.lock().lines().enumerate() {
    let x = xs.unwrap();
    if x == "0" { break }
    println!("Case {}: {}", i + 1, x)
  }
}

/*
fn main () {
  let mut xs:Vec<i32> = vec!();
  let stdin = std::io::stdin();
  for line in stdin.lock().lines() {
    let a = line.unwrap();
    if a == "0" { break }
    xs.push(a.parse().unwrap());
  }
  let mut i = 1;
  for x in xs {
    println!("Case {}: {}", i, x);
    i += 1;
  }
}
*/