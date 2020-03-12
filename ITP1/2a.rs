fn main () {
  let mut input = String::new();
  std::io::stdin().read_line(&mut input).unwrap();

  let mut iter = input.split_whitespace();
  let a: i32 = iter.next().unwrap().parse().unwrap();
  let b: i32 = iter.next().unwrap().parse().unwrap();
  let c = match a - b {
    0 => "==",
    x if x < 0 => "<",
    x if x > 0 => ">",
    _ => panic!()
  };
  println!("a {} b", c);
}