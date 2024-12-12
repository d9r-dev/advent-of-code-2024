use std::fs;

#[derive(Debug, PartialEq)]
pub struct Calc {
    sum: u64,
    nums: Vec<u64>,
}

type Calculations = Vec<Calc>;

pub fn run_day7() {
    let contents = fs::read_to_string("inputs/day7/input").expect("Could not read file");
    part1(&contents);
    part2(&contents);
}

fn part2(input: &String) {
    let calculations = parse_input(input);
    let result: u64 = calculations.iter().filter(|&calc| calc.check_validity_part2(&calc.nums[0], 1)).map(|calc| calc.sum).sum();

    println!("result part2: {}", result);

}

impl Calc {
    fn check_validity(&self, a: &u64, b: u64) -> bool {
        if b >= self.nums.len() as u64 {
            return false;
        }
        let sum = a + self.nums[b as usize];
        let mult = a * self.nums[b as usize];

        if sum == self.sum || mult == self.sum {
            true
        } else {
            self.check_validity(&sum, b + 1) ||
            self.check_validity(&mult, b + 1)
        }
    }

    fn check_validity_part2(&self, a: &u64, b: u64) -> bool {
        if b >= self.nums.len() as u64 {
            return false;
        }

        let sum = a + self.nums[b as usize];
        let mult = a * self.nums[b as usize];
        let concat = (a.to_string() + &*self.nums[b as usize].to_string()).parse::<u64>().unwrap();

        if sum == self.sum || mult == self.sum || concat == self.sum {
            true
        } else {
            self.check_validity_part2(&sum, b + 1) || self.check_validity_part2(&mult, b + 1) || self.check_validity_part2(&concat, b + 1)
        }
    }
}
fn part1(input: &String) {
    let calculations = parse_input(input);
    let result: u64 = calculations.iter().filter(|&calc| calc.check_validity(&calc.nums[0], 1)).map(|calc| calc.sum).sum();

    println!("result part1: {}", result);
}

pub fn parse_input(input: &String) -> Calculations {
    input
        .lines()
        .map(|line| {
            let (sum, rest) = line.split_once(':').unwrap();
            let sum = sum.parse::<u64>().unwrap();
            let nums = rest
                .trim()
                .split(' ')
                .map(|x| x.parse::<u64>().unwrap())
                .collect();
            Calc { sum, nums }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse() {
        let test_input = "10: 36 420 69
20: 12 20"
            .to_string();

        let mut calculations = vec![];
        calculations.push(Calc {
            sum: 10,
            nums: vec![36, 420, 69],
        });
        calculations.push(Calc {
            sum: 20,
            nums: vec![12, 20],
        });

        let output = parse_input(&test_input);

        assert_eq!(output[0], calculations[0]);
        assert_eq!(output[1], calculations[1]);
    }

    #[test]
    fn should_be_valid() {
        //292: 11 6 16 20
        let calc = Calc{
           sum: 292,
            nums: vec![11, 6, 16, 20],
        };

        assert_eq!(calc.check_validity(&calc.nums[0], 1), true);
    }

    #[test]
    fn should_be_invalid() {
        //161011: 16 10 13
        let calc = Calc{
            sum: 161011,
            nums: vec![16, 0, 13],
        };

        assert_eq!(calc.check_validity(&calc.nums[0], 1), false);
    }

    #[test]
    fn should_be_valid_part2() {
        // 7290: 6 8 6 15
        let calc = Calc{
            sum: 7290,
            nums: vec![6, 8, 6, 15],
        };

        assert_eq!(calc.check_validity_part2(&calc.nums[0], 1), true);
    }

    #[test]
    fn should_be_invalid_part2() {
        // 21037: 9 7 18 13
        let calc = Calc{
            sum: 21037,
            nums: vec![9, 7, 18, 13],
        };

        assert_eq!(calc.check_validity_part2(&calc.nums[0], 1), false);
    }
}
