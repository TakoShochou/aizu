use std::io::BufRead;

fn main () {
  let stdin = std::io::stdin();
  for line in stdin.lock().lines() {
    let input = line.unwrap();
    if input == "0 0" { break }
    let mut iter = input.split_whitespace();
    let n: u16 = iter.next().unwrap().parse().unwrap();
    let x: u16 = iter.next().unwrap().parse().unwrap();
    let mut result = 0;
    for a in 1 .. n + 1 {
      for b in a + 1 .. n + 1 {
        for c in b + 1 .. n + 1 {
          if a + b + c == x { result += 1 }
        }
      }
    }
    println!("{}", result);
  }
}
