fn line<T:std::str::FromStr>() -> T {
  let mut l = String::new();
  std::io::stdin().read_line(&mut l).ok();
  l.trim().parse().ok().unwrap()
}

fn main () {
  let input = line::<String>();
  let a: Vec<i8> = input.split_whitespace().map(|x| x.parse::<i8>().unwrap()).collect();
  println!(
    "{}",
    if
      a[2] - a[4] < 0 ||
      a[2] + a[4] > a[0] ||
      a[3] - a[4] < 0 ||
      a[3] + a[4] > a[1]
    { "No" } else { "Yes" }
  );
}
