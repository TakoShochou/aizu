use std::str::FromStr;
fn line<T:std::str::FromStr>() -> Vec<T> {
  let mut l = String::new();
  std::io::stdin().read_line(&mut l).ok();
  l.split_whitespace().map(|a| a.parse().ok().unwrap()).collect()
}

fn main () {
  loop {
    let input:Vec<String> = line();
    let (a, op, b) = (i32::from_str(&input[0]).unwrap(), &input[1], i32::from_str(&input[2]).unwrap());
    match op.as_ref() {
      "?" => break,
      "+" => println!("{}", a + b),
      "-" => println!("{}", a - b),
      "*" => println!("{}", a * b),
      "/" => println!("{}", a / b),
      _ => break,
    }
  }
}