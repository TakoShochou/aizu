fn main () {
  let mut input = String::new();
  std::io::stdin().read_line(&mut input).unwrap();
  let S :u32 = input.trim().parse().unwrap();
  println!("{}:{}:{}", S / (60 * 60), S / 60 % 60, S % 60);
}
