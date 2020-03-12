fn line<T:std::str::FromStr>() -> Vec<T> {
  let mut l = String::new();
  std::io::stdin().read_line(&mut l).ok();
  l.split_whitespace().map(|a| a.parse().ok().unwrap()).collect()
}

fn main () {
  let a:Vec<u32> = line();
  println!(
    "{} {} {:.5}",
    a[0] / a[1],
    a[0] % a[1],
    a[0] as f64 / a[1] as f64
  )
}