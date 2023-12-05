fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let matrix = parse_content(&content);

    let mut valid_nums: Vec<u32> = Vec::new();
    let mut current_num: Option<Number> = None;

    let rows = matrix.len();
    let cols = matrix[0].len();
    for i in 0..rows {
        for j in 0..cols {
            match matrix[i][j] {
                Element::Empty | Element::Symbol => {
                    if let Some(num) = current_num.take() {
                        if num.is_valid {
                            let s: String = num.digits.into_iter().collect();
                            let i: u32 = s.parse().unwrap();
                            valid_nums.push(i);
                        }
                    }
                }
                Element::Number(char) => {
                    let n = current_num.get_or_insert(Number::default());
                    n.digits.push(char);
                    if !n.is_valid {
                        n.is_valid = search_for_symbol(&matrix, i, j);
                    }
                }
            }
        }
    }

    let sum: u32 = valid_nums.iter().sum();
    println!("Result: {sum}");
}

fn get_adjacent_indices(row: usize, col: usize, rows: usize, cols: usize) -> Vec<(usize, usize)> {
    let mut indices = Vec::new();
    let rows = rows as i64;
    let cols = cols as i64;

    let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0), (-1, -1), (1, -1), (1, 1), (-1, 1)];

    for (dr, dc) in directions {
        let new_row = row as i64 + dr;
        let new_col = col as i64 + dc;

        // Check if the new indices are within bounds
        if new_row >= 0 && new_row < rows && new_col >= 0 && new_col < cols {
            indices.push((new_row as usize, new_col as usize));
        }
    }

    indices
}

fn search_for_symbol(matrix: &Vec<Vec<Element>>, i: usize, j: usize) -> bool {
    let symbols = get_adjacent_indices(i, j, matrix.len(), matrix[0].len());

    for (i, j) in symbols {
        if matrix[i][j].is_symbol() {
            return true;
        }
    }

    false
}

#[derive(Debug, Clone, Default)]
struct Number {
    digits: Vec<char>,
    is_valid: bool,
}

fn parse_content(s: &str) -> Vec<Vec<Element>> {
    s.lines()
        .map(|line| line.chars().map(Element::from).collect())
        .collect()
}

#[derive(Debug, Clone, Copy)]
enum Element {
    Empty,
    Symbol,
    Number(char),
}

impl Element {
    fn is_symbol(self) -> bool {
        matches!(self, Self::Symbol)
    }
}

impl From<char> for Element {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '+' | '-' | '*' | '/' | '%' | '=' | '@' | '&' | '$' | '#' => Self::Symbol,
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => Self::Number(c),
            _ => panic!("Unknown char: {c}"),
        }
    }
}
