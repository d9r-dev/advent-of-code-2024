use std::fs;

pub fn run_day2() {
    let contents = fs::read_to_string("inputs/day2/input1").expect("Could not read file");
    run_part_1(&contents);
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
