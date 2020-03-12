fn read<T:std::str::FromStr>() -> T {
  let mut line = String::new();
  std::io::stdin().read_line(&mut line).ok();
  line.trim().parse().ok().unwrap()
}

fn main () {
  let heystack:String = read();
  let needle:String = read();
  if heystack.repeat(2).find(&needle).is_some() {
    return println!("Yes");
  }
  println!("No");
}

/*
use std::io::BufRead;

fn main () {
  let stdin = std::io::stdin();
  let mut heystack = stdin.lock().lines().nth(0).unwrap().unwrap().to_string();
  let needle = stdin.lock().lines().nth(0).unwrap().unwrap().to_string();
  for _ in 0 .. needle.len() {
    if heystack.contains(&needle) {
      return println!("Yes")
    }
    let a = heystack.pop().unwrap();
    heystack = a.to_string() + &heystack;
  }
  println!("No")
}
*/
