use std::fs;

type Grid = Vec<Vec<char>>;

enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

struct Puzzle {
    positions: Grid,
}

impl Puzzle {
    fn new(contents: String) -> Puzzle {
        let lines = contents.split("\n").collect::<Vec<&str>>();
        let mut positions = vec![];
        lines
            .iter()
            .for_each(|&l| positions.push(Vec::from(l.chars().collect::<Vec<char>>())));
        Puzzle { positions }
    }

    fn run_part1(&self) -> usize {
        let mut current_pos = self.find_start().unwrap();
        let mut current_dir = Direction::Top;
        let mut visited_pos = vec![];
        loop {
            let next_position = Puzzle::get_next_position(current_pos, &current_dir);

            match self.get_char(next_position.0, next_position.1) {
                Some('#') => {
                    current_dir = match current_dir {
                        Direction::Top => Direction::Right,
                        Direction::Bottom => Direction::Left,
                        Direction::Left => Direction::Top,
                        Direction::Right => Direction::Bottom,
                    };
                    if !visited_pos.contains(&current_pos) {
                        visited_pos.push(current_pos);
                    }
                    current_pos = Self::get_next_position(current_pos, &current_dir);
                }
                Some('.') | Some('^') => {
                    if !visited_pos.contains(&current_pos) {
                        visited_pos.push(current_pos);
                    }
                    current_pos = next_position;
                }
                Some('\n') | None => break,
                _ => break,
            }
        }

        visited_pos.len() + 1
    }

    fn get_next_position(current_pos: (isize, isize), current_dir: &Direction) -> (isize, isize) {
        match current_dir {
            Direction::Top => (current_pos.0, current_pos.1 - 1),
            Direction::Bottom => (current_pos.0, current_pos.1 + 1),
            Direction::Left => (current_pos.0 - 1, current_pos.1),
            Direction::Right => (current_pos.0 + 1, current_pos.1),
        }
    }
    fn get_char(&self, x: isize, y: isize) -> Option<char> {
        self.positions.get(y as usize)?.get(x as usize).copied()
    }
    fn find_start(&self) -> Option<(isize, isize)> {
        for (y, line) in self.positions.iter().enumerate() {
            for (x, char) in line.iter().enumerate() {
                if *char == '^' {
                    return Some((x as isize, y as isize));
                }
            }
        }
        None
    }
}

pub fn run_day6() {
    let contents = fs::read_to_string("inputs/day6/input").expect("Could not read file");
    let puzzle = Puzzle::new(contents);
    let result1 = puzzle.run_part1();

    println!("Part 1: {}", result1);
}
