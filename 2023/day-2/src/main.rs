fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let answer: u64 = content
        .lines()
        .map(Game::parse)
        .inspect(|game| println!("Game: {game:?}"))
        .map(|game| {
            Round {
                red: game.rounds.iter().map(|round| round.red).max().unwrap_or(0),
                green: game.rounds.iter().map(|round| round.green).max().unwrap_or(0),
                blue: game.rounds.iter().map(|round| round.blue).max().unwrap_or(0),
            }
        })
        .map(Round::power)
        .map(|i| i as u64)
        .sum();
    println!("{answer}");
}

const BAG: Round = Round {
    red: 12,
    green: 13,
    blue: 14,
};

#[derive(Debug)]
struct Game {
    id: u16,
    rounds: Vec<Round>,
}

impl Game {
    fn parse(s: &str) -> Self {
        let (id_as_str, rounds_as_str) = s.split_once(": ").unwrap();
        let (_, id) = id_as_str.split_once(' ').unwrap();
        let rounds: Vec<Round> = rounds_as_str
            .split(";")
            .map(|s| s.trim())
            .map(Round::parse)
            .collect();
        Self { id: id.parse().unwrap(), rounds }
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct Round {
    red: u16,
    green: u16,
    blue: u16,
}

impl Round {
    fn parse(s: &str) -> Self {
        let mut round = Round::default();
        for (num, color) in s.split(", ").map(|s| s.split_once(' ').unwrap()) {
            let number = num.parse().unwrap();
            match color {
                "red" => round.red = number,
                "green" => round.green = number,
                "blue" => round.blue = number,
                _ => panic!("Unknown color: {color}"),
            }
        }
        round
    }

    fn power(self) -> u16 {
        self.red * self.green * self.blue
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_round() {
        let input = "3 green, 4 blue, 1 red";
        let round = Round::parse();
        assert_eq!(round, Round { red: 1, green: 3, blue: 4});
        let input = "3 green, 4 blue";
        let round: Round = input.parse().unwrap();
        assert_eq!(round, Round { red: 0, green: 3, blue: 4});
    }
}
