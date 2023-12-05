fn main() {
    let input = include_str!("../input.txt");

    let mut groups = input.split("\n\n");

    let seed_ranges: Vec<usize> = groups
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();
    println!("{seed_ranges:?}");
    
    let seeds = seed_ranges
        .chunks(2)
        .flat_map(|slice| {
            let [start, range] = slice[..] else {
                panic!("Bad range: {slice:?}");  
            };
            (start..).take(range)
        });

    let mappings: Vec<Vec<Map>> = groups.map(|s| {
        s.lines().skip(1).map(Map::from_line).collect()
    }).collect();

    let results: Vec<_> = seeds.map(|seed| {
        mappings.iter().fold(seed, |state, mapping| perform_mapping(mapping, state))
    }).collect();

    // println!("{results:?}");

    let minimum = results.into_iter().min().unwrap();
    println!("{minimum}");
}

fn perform_mapping(mapping: &[Map], num: usize) -> usize {
    for map in mapping {
        if let Some(new_num) = map.try_map(num) {
            return new_num;
        }
    }
    num
}

#[derive(Debug, Clone, Copy)]
struct Map {
    source_start: usize,
    destination_start: usize,
    range: usize,
}

impl Map {
    fn from_line(s: &str) -> Self {
        let list: Vec<_> = s.split_whitespace().map(|s| s.parse().unwrap()).collect();
        let [destination_start, source_start, range] = list[..] else {
            panic!("Incorrect mapping: {list:?}");
        };
        Map { source_start, destination_start, range }
    }

    fn try_map(self, num: usize) -> Option<usize> {
        let difference = usize::checked_sub(num, self.source_start)?;
        if difference < self.range {
            Some(self.destination_start + difference)
        } else {
            None
        }
    }
}
