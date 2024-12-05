use std::fmt::Error;
use std::fs;

pub fn run_day4() {
    let contents = fs::read_to_string("inputs/day4/input").expect("Could not read file");
    run_part_1(&contents);
    run_part_2(&contents);
}
const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

type Grid = Vec<Vec<char>>;

pub fn run_part_1(input: &str) {
    let matrix: Grid = input.lines().map(|l| l.chars().collect()).collect();
    let start = "XMAS".chars().next().unwrap();

    let result = positions(&matrix)
        .filter(|&(x, y)| char_at(&matrix, x, y) == Some(start))
        .flat_map(|(x, y)| build_words(x, y, "XMAS".len(), &matrix))
        .filter(|word_found| word_found == "XMAS")
        .count();

    println!("Part 1: {}", result);
}

fn build_words(x: isize, y: isize, word_length: usize, matrix: &Grid) -> impl Iterator<Item = String> + '_{
    DIRECTIONS.iter().filter_map(move |&(dx, dy)| {
        (0..word_length as isize)
            .map(|n| char_at(&matrix,x + dx * n, y + dy * n))
            .collect()
    })
}

fn positions(matrix: &Grid) -> impl Iterator<Item = (isize, isize)> {
    let rows = matrix.len() as isize;
    let cols = matrix.first().map_or(0, |row| row.len()) as isize;
    (0..rows).flat_map(move |x| (0..cols).map(move |y| (x, y)))
}

fn char_at(matrix: &Grid, x: isize, y: isize) -> Option<char> {
    matrix.get(x as usize)?.get(y as usize).copied()
}
pub fn run_part_2(input: &str) {}
