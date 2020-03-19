use std::io::BufRead;

fn read () -> Vec<(usize, usize, usize, i8)> {
  let stdin = std::io::stdin();
  let mut result: Vec<(usize, usize, usize, i8)> = vec![];
  for line in stdin.lock().lines().skip(1) {
    let l = line.unwrap();
    let mut iter = l.split_whitespace();
    result.push((
      iter.next().unwrap().parse::<usize>().ok().unwrap(),
      iter.next().unwrap().parse::<usize>().ok().unwrap(),
      iter.next().unwrap().parse::<usize>().ok().unwrap(),
      iter.next().unwrap().parse::<i8>().ok().unwrap(),
    ));
  }
  result
}

fn main () {
  let bfr: [[[i8;10] ;3]; 4] = [[[0; 10] ;3];4];
  let mut result = bfr.clone();
  let input = read();

  for (bi, b) in bfr.iter().enumerate() {
    for (fi, f) in b.iter().enumerate() {
      for (ri, _r) in f.iter().enumerate() {
        for &(xb, xf, xr, xv) in &input {
          if bi == xb-1 && fi == xf-1 && ri == xr-1 {
            result[bi][fi][ri] += xv; 
          }
        }
      }
    }
  }

  for (i, b) in result.iter().enumerate() {
    for f in b {
      for r in f {
        print!(" {}", r)
      }
      println!();
    }
    if i != 3 {
      println!("{}", "#".repeat(20));
    }
  }
}

/* 参考
use std::io::BufRead;

fn main() {
    let mut a = [[[0;10];3];4];
    let stdin = std::io::stdin();
    for line in stdin.lock().lines().skip(1) {
        let v: Vec<i64> = line.unwrap().split_whitespace().map(|n| n.parse().unwrap()).collect();
        let (b, f, r, v): (usize, usize, usize, i64) = (v[0] as usize, v[1] as usize, v[2] as usize, v[3]);
        a[b-1][f-1][r-1] += v;
    }
    
    for b in 0..4 {
        for f in 0..3 {
            for r in a[b][f].iter() {
                print!(" {}", r);
            }
            println!("");
        }
        if b != 3 {
            println!("{}", "#".repeat(20));
        }
    }
}

*/