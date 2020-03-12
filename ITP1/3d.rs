fn line<T:std::str::FromStr>() -> T {
  let mut l = String::new();
  std::io::stdin().read_line(&mut l).ok();
  l.trim().parse().ok().unwrap()
}

fn line_vec<T:std::str::FromStr>() -> Vec<T> {
  let mut l = String::new();
  std::io::stdin().read_line(&mut l).ok();
  l.split_whitespace().map(|a| a.parse().ok().unwrap()).collect()
}

fn main () {
  let v = line_vec::<i16>();
  println!("{}",(v[0] .. v[1] + 1).filter(|a| v[2] % a == 0).count());
}

/*
fn main () {
  let args = line_vec::<i16>();
  let (a, b, c) = (args[0], args[1], args[2]);
  let mut divs: Vec<i16> = vec!();
  for x in a .. b + 1 {
    if c % x == 0 { divs.push(b) }
  }
  println!("{}", divs.len());
}
*/
