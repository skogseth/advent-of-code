use std::collections::BTreeSet;

pub fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();

    let num_wins: Vec<usize> = content.lines().map(|line| parse_card(line)).collect();
    let mut num_cards: Vec<usize> = num_wins.iter().map(|_| 1).collect();

    let mut n_wins = &num_wins[..];
    let mut n_cards = &mut num_cards[..];
    while let ([c_wins, r_wins @ ..], [c_cards, r_cards @ ..]) = (n_wins, n_cards) {
        let end = std::cmp::min(c_wins, r_cards.len());
        for num_card in &mut num_cards[..end] {
            *num_card += current_cards;
        }
    }

    let total: usize = num_cards.into_iter().sum();
    println!("{total}");
    assert_eq!(total, 9997537);
}

fn parse_card(s: &str) -> usize {
    let (_, card) = s.split_once(':').unwrap();
    let (winnings, numbers) = card.split_once('|').unwrap();

    let to_numbers = |s: &str| -> BTreeSet<u32> {
        s.trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            //.inspect(|x| { dbg!(&x); })
            .map(|s| s.parse().unwrap())
            .collect()
    };

    let winnings = to_numbers(winnings);
    let numbers = to_numbers(numbers);

    BTreeSet::intersection(&winnings, &numbers).count()
}
