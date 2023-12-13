use std::io::Write;
use std::collections::{BTreeSet, BTreeMap};
use std::str::FromStr;

fn main() {
    let input = include_str!("../input.txt");
    let (turns, map) = parse_input(input);
    let n_steps = walk_map(turns, map);
    println!("{n_steps}");
}

fn walk_map(turns: impl Iterator<Item = Turn>, map: BTreeMap<Id, (Id, Id)>) -> usize {
    let mut current: BTreeSet<Id> = map.keys().copied().filter(|Id([_, _, c])| *c == 'A').collect();
    let end: BTreeSet<Id> = map.keys().copied().filter(|Id([_, _, c])| *c == 'Z').collect();

    let mut lock = std::io::stdout().lock();
    let _ = writeln!(lock, "[0] {current:?}");
    for (i, turn) in turns.enumerate() {
        current = current.into_iter().map(|id| {
            let (left, right) = *map.get(&id).unwrap();
            match turn {
                Turn::Left => left,
                Turn::Right => right,
            }
        }).collect();

        if current.iter().any(|Id([_, _, c])| *c == 'Z') {
            let difference: BTreeSet<_> = BTreeSet::difference(&end, &current).collect();
            let _ = writeln!(lock, "[{}] {difference:?}", i + 1);
        }
        
        if current == end {
            return i + 1;
        }
    }

    unreachable!("For loop should be infinite");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Turn {
    Left,
    Right,
}

impl From<char> for Turn {
    fn from(c: char) -> Self {
        match c {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("Bad character: {c}"),
        }
    }
}

fn parse_input(input: &str) -> (impl Iterator<Item = Turn> + '_, BTreeMap<Id, (Id, Id)>) {
    let mut lines = input.lines();
    let iter = lines.next().unwrap().chars().map(Turn::from).cycle();

    let empty = lines.next().unwrap();
    assert!(empty.is_empty());

    let mapping = lines.map(|line| {
        let (id, turns) = line.split_once('=').unwrap();
        let id = id.trim().parse().unwrap();
        let turns = parse_tuple(turns.trim());
        (id, turns)
    }).collect();
    
    (iter, mapping)
}

fn parse_tuple(input: &str) -> (Id, Id) {
    // Remove leading and trailing parentheses
    let clean_input = input.trim_start_matches('(').trim_end_matches(')');

    // Split the string into parts separated by ','
    let (left, right) = clean_input.split_once(',').unwrap();

    // Attempt to parse the two parts into u32 values
    let left = left.trim().parse().unwrap();
    let right = right.trim().parse().unwrap();

    // If parsing is successful, return the tuple
    (left, right)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Id([char;3]);

impl FromStr for Id {
    type Err = Vec<char>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();
        let chars: [char;3] = chars.try_into()?;
        Ok(Self(chars))
    }
}
