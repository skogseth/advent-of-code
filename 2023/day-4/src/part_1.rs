use std::collections::BTreeSet;

pub fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let total: u32 = content
        .lines()
        .map(|line| parse_card(line))
        //.inspect(|x| {dbg!(&x);})
        .map(|(winnings, numbers)| calculate_score(&winnings, &numbers))
        .sum();
    println!("{total}");
}

fn parse_card(s: &str) -> (BTreeSet<u32>, BTreeSet<u32>) {
    let (_, card) = s.split_once(':').unwrap();
    let (winnings, numbers) = card.split_once('|').unwrap();
    let winnings: BTreeSet<u32> = winnings
        .trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        //.inspect(|x| { dbg!(&x); })
        .map(|s| s.parse().unwrap())
        .collect();
    let numbers: BTreeSet<u32> = numbers
        .trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        //.inspect(|x| { dbg!(&x); })
        .map(|s| s.parse().unwrap())
        .collect();
    (winnings, numbers)
}

fn calculate_score(winnings: &BTreeSet<u32>, numbers: &BTreeSet<u32>) -> u32 {
    let i = BTreeSet::intersection(winnings, numbers).count();
    dbg!(&i);
    if i == 0 {
        0
    } else {
        2u32.pow(i as u32 - 1)
    }
}
