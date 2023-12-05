use std::collections::BTreeSet;

pub fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();

    let mut cards: Vec<_> = content
        .lines()
        .map(|line| Card {
            matches: parse_card(line),
            count: 1,
        })
        .collect();

    let mut ptr_cards = &mut cards[..];

    while let [current_card, remaining_cards @ ..] = ptr_cards {
        let end = std::cmp::min(current_card.matches, remaining_cards.len());

        for card in &mut remaining_cards[..end] {
            card.count += current_card.count;
        }

        ptr_cards = remaining_cards;
    }

    let total: usize = cards.into_iter().map(|card| card.count).sum();
    println!("{total}");
    assert_eq!(total, 9997537);
}

struct Card {
    matches: usize,
    count: usize,
}

fn parse_card(s: &str) -> usize {
    let (_, card) = s.split_once(':').unwrap();
    let (winners, numbers) = card.split_once('|').unwrap();

    let find_numbers = |s: &str| s.split_whitespace().map(|s| s.parse().unwrap()).collect();

    let winners: BTreeSet<u32> = find_numbers(winners);
    let numbers: BTreeSet<u32> = find_numbers(numbers);

    BTreeSet::intersection(&winners, &numbers).count()
}
