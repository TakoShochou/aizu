fn main () {
  let mut a = String::new();
  std::io::stdin().read_line(&mut a).ok();
  let b:Vec<i32> = a.split_whitespace().map(|x| x.parse().unwrap()).collect();
  println!("{}", if b[0] < b[1] && b[1] < b[2] { "Yes" } else { "No" });
}