use itertools::{Itertools, MinMaxResult};

fn main() {
    let test_input = std::fs::read_to_string("input.txt").unwrap();
    let answer: usize = test_input.lines().map(find_calibration_value).sum();
    println!("{answer}");
}

const NUMBERS: &[&str] = &[
    "1", "2", "3", "4", "5", "6", "7", "8", "9",
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn parse_num(s: &str) -> usize {
    match s {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => s.parse().unwrap(),
    }
}

fn find_calibration_value(content: &str) -> usize {
    let minmax = NUMBERS.iter()
        .flat_map(|num| content.match_indices(num))
        .map(|(i, num)| (i, parse_num(num)))
        .minmax_by_key(|(i, _)| *i);

    let result = match minmax {
        MinMaxResult::NoElements => panic!("No digit found"),
        MinMaxResult::OneElement((_,single)) => format!("{single}{single}"),
        MinMaxResult::MinMax((_,first), (_,last)) => format!("{first}{last}"),
    };

    result.parse().unwrap()
}

fn find_calibration_value(content: &str) -> usize {
    let minmax = NUMBERS.iter()
        .flat_map(|num| content.match_indices(num))
        .map(|(i, num)| (i, parse_num(num)))
        .minmax_by_key(|(i, _)| *i);

    let result = match minmax {
        MinMaxResult::NoElements => panic!("No digit found"),
        MinMaxResult::OneElement((_,single)) => format!("{single}{single}"),
        MinMaxResult::MinMax((_,first), (_,last)) => format!("{first}{last}"),
    };

    result.parse().unwrap()
}
