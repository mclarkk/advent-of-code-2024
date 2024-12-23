use std::fs;

fn part1() {
    let puzzle = Puzzle::from_string(fs::read_to_string("./src/input.txt").unwrap());
    let directions = vec![
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
        Direction::Upleft,
        Direction::Upright,
        Direction::Downleft,
        Direction::Downright,
    ];

    let x_positions = puzzle.get_char_positions('X');

    let mut sum = 0;
    for candidate_start_pos in x_positions {
        // create 8 XMAS sequence candidates
        // if they are present, add to sum
        for direction in &directions {
            let candidate_seq = Sequence {
                starting_position: candidate_start_pos.clone(),
                direction: direction.clone(),
                sequence_vals: vec!['X', 'M', 'A', 'S'],
            };
            if candidate_seq.is_present(&puzzle) {
                sum += 1;
            }
        }
    }
    println!("Num of XMAS: {}", sum);
}

fn part2() {
    let puzzle = Puzzle::from_string(fs::read_to_string("./src/input.txt").unwrap());

    let a_positions = puzzle.get_char_positions('A');

    let mut sum = 0;
    for candidate_center_pos in a_positions {
        // check that all diagonal positions are present (e.g. not on edge or corrners)
        if candidate_center_pos
            .next_position(Direction::Upleft)
            .is_some()
            && candidate_center_pos
                .next_position(Direction::Downright)
                .is_some()
            && candidate_center_pos
                .next_position(Direction::Upright)
                .is_some()
            && candidate_center_pos
                .next_position(Direction::Downleft)
                .is_some()
        {
            let diagonal1_forward = Sequence {
                starting_position: candidate_center_pos.next_position(Direction::Upleft).unwrap(),
                direction: Direction::Downright,
                sequence_vals: vec!['M', 'A', 'S'],
            };
            let diagonal1_backward = Sequence {
                starting_position: candidate_center_pos.next_position(Direction::Upleft).unwrap(),
                direction: Direction::Downright,
                sequence_vals: vec!['S', 'A', 'M'],
            };
            // Only bother checking for the second diagonal if the first one exists
            if diagonal1_forward.is_present(&puzzle) || diagonal1_backward.is_present(&puzzle) {
                let diagonal2_forward = Sequence {
                    starting_position: candidate_center_pos.next_position(Direction::Upright).unwrap(),
                    direction: Direction::Downleft,
                    sequence_vals: vec!['M', 'A', 'S'],
                };
                let diagonal2_backward = Sequence {
                    starting_position: candidate_center_pos.next_position(Direction::Upright).unwrap(),
                    direction: Direction::Downleft,
                    sequence_vals: vec!['S', 'A', 'M'],
                };
                if diagonal2_forward.is_present(&puzzle) || diagonal2_backward.is_present(&puzzle) {
                    sum += 1;
                }
            }
        }
    }
    println!("Num of X-MAS: {}", sum);
}

fn main() {
    part1();
    part2();
}

struct Puzzle {
    rows: Vec<Vec<char>>,
    row_len: usize,
    col_len: usize,
}

impl Puzzle {
    fn from_string(s: String) -> Puzzle {
        let row_strings: Vec<String> = s.split("\n").map(|c| c.to_string()).collect();
        let rows: Vec<Vec<char>> = row_strings
            .iter()
            .map(|row_str| row_str.chars().collect())
            .collect();
        let row_len = *(&rows[0].len());
        let col_len = rows.len();
        Puzzle {
            rows,
            row_len,
            col_len,
        }
    }

    fn get_val(&self, pos: Position) -> Option<char> {
        if (pos.row as isize) < 0
            || pos.row >= self.col_len
            || (pos.col as isize) < 0
            || pos.col >= self.row_len
        {
            None
        } else {
            Some(self.rows[pos.row as usize][pos.col as usize])
        }
    }

    fn get_char_positions(&self, target_char: char) -> Vec<Position> {
        let mut positions = Vec::<Position>::new();
        for row in 0..self.col_len {
            for col in 0..self.row_len {
                if self.rows[row][col] == target_char {
                    positions.push(Position {
                        row,
                        col,
                        row_len: self.row_len,
                        col_len: self.col_len,
                    });
                }
            }
        }
        positions
    }
}

#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Upright,
    Upleft,
    Downright,
    Downleft,
}

#[derive(Debug, Clone)]
struct Position {
    row: usize,
    col: usize,
    row_len: usize,
    col_len: usize,
}

impl Position {
    fn next_position(&self, direction: Direction) -> Option<Position> {
        let row = self.row as isize;
        let col = self.col as isize;
        match direction {
            Direction::Up => self.from_row_col(row - 1, col),
            Direction::Down => self.from_row_col(row + 1, col),
            Direction::Left => self.from_row_col(row, col - 1),
            Direction::Right => self.from_row_col(row, col + 1),
            Direction::Upleft => self.from_row_col(row - 1, col - 1),
            Direction::Upright => self.from_row_col(row - 1, col + 1),
            Direction::Downleft => self.from_row_col(row + 1, col - 1),
            Direction::Downright => self.from_row_col(row + 1, col + 1),
        }
    }

    fn from_row_col(&self, row: isize, col: isize) -> Option<Position> {
        if row < 0 || row >= self.col_len as isize || col < 0 || col >= self.row_len as isize {
            None
        } else {
            Some(Position {
                row: row as usize,
                col: col as usize,
                col_len: self.col_len,
                row_len: self.row_len,
            })
        }
    }
}

struct Sequence {
    starting_position: Position,
    direction: Direction,
    sequence_vals: Vec<char>,
}

impl Sequence {
    fn is_present(&self, vals: &Puzzle) -> bool {
        if vals.get_val(self.starting_position.clone()) == Some(self.sequence_vals[0]) {
            if self.sequence_vals.len() == 1 {
                true
            } else {
                match self.starting_position.next_position(self.direction.clone()) {
                    None => false,
                    Some(position) => {
                        let subsequence = Sequence {
                            starting_position: position,
                            direction: self.direction.clone(),
                            sequence_vals: self.sequence_vals[1..].to_vec(),
                        };
                        subsequence.is_present(vals)
                    }
                }
            }
        } else {
            false
        }
    }
}
