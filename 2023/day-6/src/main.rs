fn main() {
    let input = include_str!("../input.txt");
    let (time, distance) = parse_input(input);
    let total: usize = num_ways_to_beat((time, distance));
    println!("{total}");
}

fn parse_input(input: &str) -> (usize, usize) {
    let lines: Vec<&str> = input.lines().collect();

    let [times, distances] = &lines[..] else {
        panic!("Bad input: {lines:?}");
    };

    let parse_line = |line: &str| -> usize {
        line.split_whitespace()
            .skip(1)
            .collect::<String>()
            .parse()
            .unwrap()
    };

    (parse_line(times), parse_line(distances))
}

fn num_ways_to_beat((time, distance): (usize, usize)) -> usize {
    (1..time)
        .map(|hold_time| {
            let speed = hold_time;
            let remaining_time = time - hold_time;
            speed * remaining_time
        })
        //.inspect(|d| { println!("{d}"); })
        .filter(|d| *d > distance)
        .count()
}
