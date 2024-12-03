use regex::Regex;
use std::fs;
pub fn run_day3() {
    let contents = fs::read_to_string("inputs/day3/input").expect("Could not read file");
    run_part_1(&contents);
    run_part_2(&contents);
}

fn run_part_1(contents: &String) {
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    let matches: Vec<_> = re.find_iter(contents).map(|m| m.as_str()).collect();
    let mut result: u64 = 0;

    matches.iter().for_each(|&m| println!("{}", m));
    let _ = matches
        .iter()
        .map(|&string| parse_tupels(string))
        .collect::<Vec<_>>()
        .iter()
        .for_each(|tup| result += tup.0 * tup.1);

    println!("{}", result);
}

fn parse_tupels(string: &str) -> (u64, u64) {
    let numbers = string
        .strip_prefix("mul(")
        .unwrap()
        .strip_suffix(')')
        .unwrap()
        .split_once(',')
        .unwrap();

    (numbers.0.parse().unwrap(), numbers.1.parse().unwrap())
}

fn run_part_2(contents: &String) {
    ()
}
