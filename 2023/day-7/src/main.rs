use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let mut hands: Vec<(Hand, u32)> = input.lines().map(parse_line).collect();
    hands.sort_by_key(|(hand, _)| *hand);
    let total: u32 = hands
        .into_iter()
        .enumerate()
        .map(|(i, (_, bid))| (i+1) as u32 * bid)
        .sum();
    println!("{total}");
}

fn parse_line(s: &str) -> (Hand, u32) {
    let strings: Vec<_> = s.split_whitespace().collect();
    let strings: [&str;2] = strings.try_into().unwrap(); 
    let [hand, bid] = strings;

    let hand = Hand::parse(hand);
    let bid = bid.parse().unwrap();

    (hand, bid)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, Hash)]
struct Hand([CamelCard;5]);

impl Hand {
    fn parse(s: &str) -> Self {
        let chars: Vec<_> = s.chars().collect();
        let chars: [char;5] = chars.try_into().unwrap();
        let cards: [CamelCard;5] = chars.map(CamelCard::from);
        Hand(cards)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let left = HandType::from(self);
        let right = HandType::from(other);

        if let ord @ (Ordering::Less | Ordering::Greater) = left.cmp(&right) {
            return Some(ord);
        }

        for (self_card, other_card) in std::iter::zip(&self.0, &other.0) {
            if let ord @ (Ordering::Less | Ordering::Greater) = self_card.cmp(&other_card) {
                 return Some(ord);
            }
        }

        Some(Ordering::Equal)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum CamelCard {
    Joker,
    Number(u8),
    Ten,
    //Jack,
    Queen,
    King,
    Ace,
}

impl From<char> for CamelCard {
    fn from(c: char) -> Self {
        match String::from(c).parse() {
            Ok(i @ 2..=9) => Self::Number(i),
            Ok(i) => unreachable!("Bad number: {i}"),
            Err(_) => match c {
                'J' => Self::Joker,
                'T' => Self::Ten,
                //'J' => Self::Jack,
                'Q' => Self::Queen,
                'K' => Self::King,
                'A' => Self::Ace,
                _ => panic!("Bad character: {c}"),
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<&Hand> for HandType {
    fn from(hand: &Hand) -> Self {
        let mut grouped: HashMap<CamelCard, u8> = HashMap::new();
        let mut jokers = 0;

        for card in hand.0 {
            match card {
                CamelCard::Joker => jokers += 1,
                _ => {
                    let entry = grouped.entry(card).or_insert(0);
                    *entry += 1;
                }
            }
        }

        let mut values: Vec<_> = grouped.values().copied().collect();
        values.sort();

        if let Some(last) = values.last_mut() {
            *last += jokers;
        } else {
            assert_eq!(jokers, 5);
            return HandType::FiveOfAKind;
        }

        match values[..] {
            [5] => HandType::FiveOfAKind,
            [.., 4] => HandType::FourOfAKind,
            [2, 3] => HandType::FullHouse,
            [.., 3] => HandType::ThreeOfAKind,
            [.., 2, 2] => HandType::TwoPairs,
            [.., 2] => HandType::OnePair,
            [..] => HandType::HighCard,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_hand_type() {
        let hand = Hand::parse("A9AA9");
        let typ = HandType::from(&hand);
        assert_eq!(typ, HandType::FullHouse);

        let hand = Hand::parse("143K1");
        let typ = HandType::from(&hand);
        assert_eq!(typ, HandType::OnePair);
    }
}
