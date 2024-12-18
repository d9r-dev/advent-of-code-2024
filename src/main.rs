use std::env;

use advent_of_code::aoc2024::day1::run_day1;
use advent_of_code::aoc2024::day2::run_day2;
use advent_of_code::aoc2024::day3::run_day3;
use advent_of_code::aoc2024::day4::run_day4;
use advent_of_code::aoc2024::day5::run_day5;
use advent_of_code::aoc2024::day6::run_day6;
use advent_of_code::aoc2024::day7::run_day7;
use advent_of_code::aoc2024::day8::run_day8;
use advent_of_code::aoc2024::day9::run_day9;

struct Config {
    day: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let run_config = Config {
        day: args[1].clone(),
    };

    run(&run_config.day);
}

fn run(day: &str) {
    match day {
        "1" => run_day1(),
        "2" => run_day2(),
        "3" => run_day3(),
        "4" => run_day4(),
        "5" => run_day5(),
        "6" => run_day6(),
        "7" => run_day7(),
        "8" => run_day8(),
        "9" => run_day9(),
        _ => (),
    }
}
