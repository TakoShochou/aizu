use std::io::BufRead;

fn main () {
    let stdin = std::io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let hint: Vec<usize> = input.split_whitespace().map(|a| a.parse::<usize>().ok().unwrap()).collect();
    let c = hint[1];
    let mut sum: Vec<usize> = vec![0; c + 1];
    for input in stdin.lock().lines() {
        let mut line: Vec<usize> = input.ok().unwrap()
            .split_whitespace()
            .map(|a| a.parse::<usize>().ok().unwrap())
            .collect();
        // let sum = line.iter().fold(0, |acc, a| acc + a);
        let line_sum: usize = line.iter().sum();
        line.push(line_sum);
        // println!("{}", line.iter().fold("".to_string(), |acc, a| format!("{} {}", acc, a) ));
        println!("{}", line.iter().map(|a| a.to_string()).collect::<Vec<_>>().join(" "));

        for i in 0 .. line.len() {
            sum[i] = sum[i] + line[i];
        }
    }
    println!("{}", sum.iter().map(|a| a.to_string()).collect::<Vec<_>>().join(" "));
}
