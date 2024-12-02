use std::fs;

pub fn run_day2() {
    let contents = fs::read_to_string("inputs/day2/input1").expect("Could not read file");
    run_part_1(&contents);
    run_part_2(&contents);
}

pub fn run_part_1(input: &str) {
    let list = input.lines().collect::<Vec<&str>>();
    let mut result: i32 = 0;
    list.iter().for_each(|line| {
        let mut forward: bool = true;
        let mut valid: bool = true;
        let mut before: i32 = 0;
        for (i, number) in line.split_whitespace().enumerate() {
            let num = number.parse::<i32>().unwrap();
            if i == 1 {
                if num == before {
                    valid = false;
                    break;
                }
                if num < before {
                    forward = false;
                }
            }
            if i > 0 {
                if forward && num > before && (num - before).abs() <= 3 {
                    before = num;
                    continue;
                } else if !forward && num < before && (num - before).abs() <= 3 {
                    before = num;
                    continue;
                } else {
                    valid = false;
                    break;
                }
            }
            before = num;
        }
        result = match valid {
            true => result + 1,
            false => result,
        }
    });
    println!("Day 2 Part 1: {}", result);
}

pub fn run_part_2(input: &str) {
    let list = input.lines().collect::<Vec<&str>>();
    let mut result: i32 = 0;
    list.iter().for_each(|line| {
        let levels = line
            .split_whitespace()
            .map(|char| char.parse::<i32>().unwrap())
            .collect();
        let valid = check_validity(&levels);
        result = match valid {
            true => result + 1,
            false => {
                let mut temp = 0;
                for id_to_drop in 0..levels.len() {
                    let mut new_levels = levels.clone();
                    new_levels.remove(id_to_drop);
                    let valid = check_validity(&new_levels);
                    match valid {
                        true => {
                            temp = 1;
                            break;
                        }
                        false => temp = 0,
                    }
                }
                result + temp
            }
        }
    });
    println!("Day 2 Part 1: {}", result);
}

fn check_validity(line: &Vec<i32>) -> bool {
    let mut forward: bool = true;
    let mut valid: bool = true;
    let mut before: i32 = 0;
    for (i, num) in line.into_iter().enumerate() {
        if i == 1 {
            if *num == before {
                valid = false;
                break;
            }
            if *num < before {
                forward = false;
            }
        }
        if i > 0 {
            if forward && *num > before && (num - before).abs() <= 3 {
                before = *num;
                continue;
            } else if !forward && *num < before && (num - before).abs() <= 3 {
                before = *num;
                continue;
            } else {
                valid = false;
                break;
            }
        }
        before = *num;
    }
    valid
}
