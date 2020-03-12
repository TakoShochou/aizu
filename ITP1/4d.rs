fn main () {
  let mut input1 = String::new();
  std::io::stdin().read_line(&mut input1).unwrap();
  let mut input2 = String::new();
  std::io::stdin().read_line(&mut input2).unwrap();
  let mut v:Vec<i64> = input2.split_whitespace().map(|x| x.parse().unwrap()).collect();
  v.sort();
  println!("{} {} {}", v.first().unwrap(), v.last().unwrap(), v.iter().fold(0, |acc, x| acc + x));
}
