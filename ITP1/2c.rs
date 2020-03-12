fn main () {
  let mut a = String::new();
  std::io::stdin().read_line(&mut a).ok();
  let mut b:Vec<i32> = a.split_whitespace().map(|a| a.parse().unwrap()).collect();
  b.sort();
  println!("{} {} {}", b[0], b[1], b[2])
}