use std::io::BufRead;

fn main () {
  let stdin = std::io::stdin();
  for line in stdin.lock().lines() {
    let input = line.unwrap();
    if input == "-1 -1 -1" { break }
    let mut iter = input.split_whitespace();
    let m: i64 = iter.next().unwrap().parse().unwrap();
    let f: i64 = iter.next().unwrap().parse().unwrap();
    let r: i64 = iter.next().unwrap().parse().unwrap();
    let result = match (m, f, r) {
      (-1, _, _) | (_, -1, _) => "F",
      (m, f, _) if m + f >= 80 => "A",
      (m, f, _) if m + f >= 65 => "B",
      (m, f, _) if m + f >= 50 => "C",
      (m, f, r) if m + f >= 30 => if r >= 50 { "C" } else { "D" },
      _ => "F"
    };
    println!("{}", result);
  }
}
