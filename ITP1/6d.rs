fn read<T:std::str::FromStr>() -> T {
  let mut line = String::new();
  std::io::stdin().read_line(&mut line).ok();
  line.trim().parse().ok().unwrap()
}

fn split_whitespace<T:std::str::FromStr>(line: String) -> Vec<T> {
  line.split_whitespace().map(|a| a.parse().ok().unwrap()).collect()
}

fn read_nm () -> (usize, usize) {
  let line: String = read();
  let nm: Vec<usize> = split_whitespace(line);
  (nm[0], nm[1])
}

fn read_matrix (n: usize) -> Vec<Vec<u64>> {
  let mut result: Vec<Vec<u64>> = vec![vec![0; 0]; 0];
  for _ in 0 .. n {
    let row: Vec<u64> = split_whitespace(read::<String>());
    result.push(row);
  }
  result
}

fn read_vector (m: usize) -> Vec<u64> {
  let mut result: Vec<u64> = Vec::new();
  for _ in 0 .. m {
    result.push(read::<u64>())
  }
  result
}

fn product (xs: Vec<u64>, ys: Vec<u64>) -> u64 {
  xs.iter().zip(ys.iter()).fold(0, |acc, (a, b)| acc + a * b)
}

fn calc (matrix: Vec<Vec<u64>>, vector: Vec<u64>) -> Vec<u64> {
  matrix.iter().map(|m| product(m.to_vec(), vector.as_slice().to_vec())).collect()
}

fn main () {
  let (n, m) = read_nm();
  let matrix = read_matrix(n);
  let vector = read_vector(m);
  for a in calc(matrix, vector) {
    println!("{}", a);
  }
}

/** 手本
use std::io::*;
use std::str::FromStr;

struct Scanner<R: Read> {
    reader: R,
}

#[allow(dead_code)]
impl<R: Read> Scanner<R> {
    fn new(reader: R) -> Scanner<R> {
        Scanner { reader: reader }
    }

    fn safe_read<T: FromStr>(&mut self) -> Option<T> {
        let token = self.reader.by_ref().bytes().map(|c| c.unwrap() as char)
            .skip_while(|c| c.is_whitespace())
            .take_while(|c| !c.is_whitespace())
            .collect::<String>();
        if token.is_empty() {
            None
        } else {
            token.parse::<T>().ok()
        }
    }

    fn read<T: FromStr>(&mut self) -> T {
        if let Some(s) = self.safe_read() {
            s
        } else {
            writeln!(stderr(), "Terminated with EOF").unwrap();
            std::process::exit(0);
        }
    }
}

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    let n = sc.read();
    let m = sc.read();
    let a: Vec<Vec<i64>> = (0..n).map(|_| (0..m).map(|_| sc.read()).collect()).collect();
    let b: Vec<i64> = (0..m).map(|_| sc.read()).collect();
    for i in 0..n {
        println!("{}", (0..m).fold(0, |acc, j| acc + a[i][j] * b[j]));
    }
}


Compile Error Logs:

ステータス
Judge: 18/18	Rust	CPU: 00:00 sec	Memory: 5048 KB	Length: 1215 B	2018-02-05 00:37
テストケースの判定結果
Case #	Verdict	CPU Time	Memory	In	Out	Case Name
Case #1	: Accepted	00:00	5028	36	6	00_sample_00.in
Case #2	: Accepted	00:00	5028	36	6	00_sample_01.in
Case #3	: Accepted	00:00	5032	16	5	01_small_00.in
Case #4	: Accepted	00:00	5032	12	3	01_small_01.in
Case #5	: Accepted	00:00	5032	44	9	01_small_02.in
Case #6	: Accepted	00:00	5036	88	17	01_small_03.in
Case #7	: Accepted	00:00	5028	8	2	02_corner_00.in
Case #8	: Accepted	00:00	5028	8	2	02_corner_01.in
Case #9	: Accepted	00:00	5028	28	6	02_corner_02.in
Case #10	: Accepted	00:00	5032	14	8	02_corner_03.in
Case #11	: Accepted	00:00	5032	122	20	03_rand_00.in
Case #12	: Accepted	00:00	5028	314	26	03_rand_01.in
Case #13	: Accepted	00:00	5032	1566	100	03_rand_02.in
Case #14	: Accepted	00:00	5032	4374	240	03_rand_03.in
Case #15	: Accepted	00:00	5032	1566	100	04_large_00.in
Case #16	: Accepted	00:00	5032	4374	240	04_large_01.in
Case #17	: Accepted	00:00	5048	39299	900	05_maximum_00.in
Case #18	: Accepted	00:00	5048	50508	1000	05_maximum_01.in

< prev |		/		| next >	 		
 
Judge Input #  ( | )		Judge Output #  ( | )

*/