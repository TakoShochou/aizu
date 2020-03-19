fn line<T:std::str::FromStr>() -> Vec<T> {
  let mut l = String::new();
  std::io::stdin().read_line(&mut l).ok();
  l.split_whitespace().map(|a| a.parse().ok().unwrap()).collect()
}

fn main () {
  let n:i16 = line()[0];
  let mut i:i16 = 1;
  loop {
    let mut x = i;
    if x % 3 == 0 { print!(" {}", i) }
    else {
      loop {
        if x % 10 == 3 { print!(" {}", i); break }
        x /= 10;
        if x == 0 { break }
      }
    }
    i += 1;
    if i > n { break }
  }
  println!()
}

/* 間違い
fn main () {
  let n:u16 = line()[0];
  let mut x:u16 = 1;
  loop {
    println!("x={}", x);
    if x % 3 == 0 ||
      x % 10 == 3 ||
      (x / 10) % 10 == 3
    {
        println!(" {}", x);
    }
    x += 1;
    if x > n { break }
  }
  println!()
}
*/