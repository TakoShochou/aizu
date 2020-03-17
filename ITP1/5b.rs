fn line<T:std::str::FromStr>() -> Vec<T> {
  let mut l = String::new();
  std::io::stdin().read_line(&mut l).ok();
  l.split_whitespace().map(|a| a.parse().ok().unwrap()).collect()
}

fn main () {
  loop {
    let arg: Vec<i16> = line();
    let (h, w) = (arg[0], arg[1]);
    if h == 0 && w == 0 { return }
    for i in 0 .. h {
      if i == 0 || i == h - 1 {
        println!("{}", "#".repeat(w as usize));
        continue
      }
      println!("#{}#", ".".repeat((w - 2) as usize))
    }
    println!()
  }
}

/** 見本
use std::io;
use std::io::BufRead;

fn main() {
	let stdin = io::stdin();
	for line in stdin.lock().lines() {
		let v: Vec<usize> = line.unwrap().split_whitespace()
			.map(|s| s.parse().unwrap()).collect();
		let (h, w) = (v[0], v[1]);
		let s = "#".repeat(w) + "\n";
		let rect = match (h, w) {
			(0, 0) => break,
			(x, y) if x < 3 || y < 3 => s.repeat(h),
			(_, _) => s + &("#".to_string() + &(".".repeat(w-2)) + "#\n").repeat(h-2) + &("#".repeat(w)) + "\n",
		};
		println!("{}", rect);
	}
}
*/