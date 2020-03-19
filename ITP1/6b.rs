use std::io::BufRead;

type Card = (char, u8);

fn main () {
  let stdin = std::io::stdin();
  let mut input2:Vec<Card> = vec![];
  for line in stdin.lock().lines().skip(1) { // LOOK: skip(1)
    let l = line.unwrap();
    let mut iter = l.split_whitespace();
    let s:char = iter.next().unwrap().chars().next().unwrap();
    let r = iter.next().unwrap().parse::<u8>().ok().unwrap();
    input2.push((s, r));
  }
  
  let cards = make_cards();
  let missing_cards: Vec<Card> = cards.iter().filter(|c| !input2.contains(c)).cloned().collect();
  for c in missing_cards {
    println!("{} {}", c.0, c.1);
  }
}

fn make_cards () -> Vec<Card> {
  let suits = vec!['S', 'H', 'C', 'D'];
  let ranks:Vec<u8> = (1..14).collect();

  let mut cards:Vec<Card> = vec![];
  for s in suits {
    for r in &ranks {
      cards.push((s, *r));
    }
  }
  cards
}

/*見本
use std::io::BufRead;
use std::collections::HashSet;

fn main() {
    let mut set = HashSet::new();
    let stdin = std::io::stdin();
    for line in stdin.lock().lines().skip(1) {
        set.insert(String::from(line.unwrap().trim()));
    }

    for s in ["S", "H", "C", "D"].iter() {
        for n in 1..14 {
            let s = format!("{} {}", s, n);
            if !set.contains(&s) {
                println!("{}", s);
            }
        }
    }
}
*/