fn main () {
  loop {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    if input.contains("0 0") { break }
    let mut iter = input.split_whitespace();
    let a:usize = iter.next().unwrap().parse().unwrap();
    let b:usize = iter.next().unwrap().parse().unwrap();
    for _ in 0 .. a {
      println!("{}", "#".repeat(b));
    }
    println!();
  }
}