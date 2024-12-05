use std::{collections::HashMap, fs};

pub fn run_day1() {
    let contents = fs::read_to_string("inputs/day1/input1").expect("Could not read file");
    run_part_1(&contents);
    run_part_2(&contents);
}

pub fn run_part_1(contents: &String) {
    let (mut list1, mut list2) = parse_day1_input(&contents);
    list1.sort();
    list2.sort();
    let result = list1
        .into_iter()
        .zip(list2)
        .map(|(l, r)| l.abs_diff(r))
        .sum::<u32>();

    println!("Result Day1 Part 1: {result}")
}

pub fn run_part_2(contents: &String) {
    let (list1, list2) = parse_day1_input(&contents);
    let mut freq = HashMap::new();
    list2
        .into_iter()
        .for_each(|l2| *freq.entry(l2).or_insert(0) += 1);
    let result: u32 = list1
        .iter()
        .filter_map(|l1| freq.get(l1).map(|f| l1 * f))
        .sum();
    println!("Result Day1 Part 2: {result}")
}

pub fn parse_day1_input(contents: &String) -> (Vec<u32>, Vec<u32>) {
    let mut list1 = vec![];
    let mut list2 = vec![];

    contents.lines().for_each(|line| {
        let tuple = line.trim().split_once(" ").unwrap();
        list1.push(
            tuple
                .0
                .trim()
                .parse::<u32>()
                .expect("Could not parse String"),
        );
        list2.push(
            tuple
                .1
                .trim()
                .parse::<u32>()
                .expect("Could not parse String"),
        );
    });

    (list1, list2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shoud_parse() {
        let test_input = "1234    4567
    6789    9876"
            .to_string();

        let mut control_list1 = Vec::new();
        control_list1.push(1234);
        control_list1.push(6789);

        let mut control_list2 = Vec::new();
        control_list2.push(4567);
        control_list2.push(9876);

        assert_eq!(
            parse_day1_input(&test_input),
            (control_list1, control_list2)
        )
    }
}
