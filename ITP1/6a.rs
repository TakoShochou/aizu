use std::io::BufRead;
fn main () {
  let stdin = std::io::stdin();
  let line = stdin.lock().lines().nth(1).unwrap().unwrap();
  println!("{}", line.split_whitespace().rev().collect::<Vec<&str>>().join(" "));
}

/*
fn main () {
  let mut _in1 = String::new();
  std::io::stdin().read_line(&mut _in1).unwrap();

  let mut in2 = String::new();
  std::io::stdin().read_line(&mut in2).unwrap();
  let mut v:Vec<u16> = in2.split_whitespace().map(|x| x.parse().unwrap()).collect();
  v.reverse();
  for (i, a) in v.iter().enumerate() {
    if i == v.len() - 1 {
      print!("{}", a);
      continue;
    }
    print!("{} ", a);
  }
  println!();
}
*/