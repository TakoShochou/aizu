fn line<T:std::str::FromStr>() -> Vec<T> {
  let mut l = String::new();
  std::io::stdin().read_line(&mut l).ok();
  l.split_whitespace().map(|a| a.parse().ok().unwrap()).collect()
}

fn main () {
  loop {
    let arg: Vec<i16> = line();
    let (h, w) = (arg[0], arg[1]);
    println!("{}", match (h, w) {
      (0, 0) => break,
      (x, y) => solve(x, y),
    });
  }
}

fn solve (h: i16, w: i16) -> String {
  let mut result = String::new();
  for i in 0 .. h {
    if i % 2 == 0 {
      result.push_str(&solve2("#".to_string(), ".".to_string(), w));
      continue
    }
    result.push_str(&solve2(".".to_string(), "#".to_string(), w));
  } // outer for
  result
}

fn solve2 (a: String, b: String, w: i16) -> String {
  let mut result = String::new();
  for j in 0 .. w {
    if j % 2 == 0 {
      result.push_str(&a);
    } else {
      result.push_str(&b);
    }
  }
  result.push_str("\n");
  result
}

/** 見本
use std::io;
use std::io::BufRead;

fn main() {
    let board_line = "#.".repeat(151);
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let v: Vec<usize> = line.unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        if v == [0, 0] {
            return;
        }
        for (a, b) in (0..2).cycle().map(|a| (a, a + v[1])).take(v[0]) {
            println!("{}", &board_line[a..b]);
        }
        println!("");
    }
}
*/