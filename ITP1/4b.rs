fn main () {
  let mut l = String::new();
  std::io::stdin().read_line(&mut l).ok();
  let r = l.trim().parse::<f64>().ok().unwrap();
  println!(
    "{:.6} {:.6}",
    r * r * std::f64::consts::PI,
    2. * r * std::f64::consts::PI
  )
}