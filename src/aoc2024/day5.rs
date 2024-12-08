use std::fs;

#[derive(Debug)]
struct Rule {
    pub first: usize,
    pub last: usize,
}

#[derive(Debug, Clone)]
struct Update {
    pub steps: Vec<usize>,
}

#[derive(Debug)]
struct Puzzle {
    pub rules: Vec<Rule>,
    pub updates: Vec<Update>,
}

impl Rule {
    fn new(first: usize, last: usize) -> Rule {
        Rule { first, last }
    }
}

impl Puzzle {
    fn new(contents: String) -> Puzzle {
        let (rules, updates) = Puzzle::parse_input(contents);
        Puzzle { rules, updates }
    }

    fn parse_input(input: String) -> (Vec<Rule>, Vec<Update>) {
        let (rules_string, updates_string) = input.split_once("\n\n").unwrap();
        let rules = rules_string
            .split("\n")
            .map(|string| {
                let (first, last) = string.split_once("|").unwrap();
                Rule::new(first.parse().unwrap(), last.parse().unwrap())
            })
            .collect();

        let mut updates = Vec::new();

        updates_string
            .split("\n")
            .map(|string| {
                let mut steps = Vec::new();
                string
                    .split(",")
                    .map(|str| str.parse::<usize>().unwrap())
                    .for_each(|step| steps.push(step));
                Update { steps }
            })
            .for_each(|update| updates.push(update));

        (rules, updates)
    }

    fn run_part_1(self: &Puzzle) -> usize {
        let failed = self.get_failed();
        self.updates
            .iter()
            .enumerate()
            .filter_map(|(p, update)| {
                if failed[p] {
                    return None;
                }
                let mid = update.steps[update.steps.len() / 2];
                Some(mid)
            })
            .sum()
    }

    fn run_part2(self: &Puzzle) -> usize {
        let failed = self.get_failed();
        let failed_pages: Vec<_> = self
            .updates
            .iter()
            .enumerate()
            .filter_map(|p| {
                if failed[p.0] {
                    return Some(p.1.clone());
                }
                None
            })
            .collect();

        let mut sum = 0;
        for page in failed_pages {
            let mut steps = page.steps.clone();
            loop {
                let mut swapped = false;

                'outer: for rule in self.rules.iter() {
                    for i in 0..steps.len() {
                        if steps[i] == rule.first {
                            continue 'outer;
                        }
                        if steps[i] == rule.last {
                            for j in i..steps.len() {
                                if steps[j] == rule.first {
                                    steps.swap(i, j);
                                    swapped = true;
                                    continue 'outer;
                                }
                            }
                        }
                    }
                }
                if !swapped {
                    sum += steps[steps.len() / 2];
                    break;
                }
            }
        }
        sum
    }

    fn get_failed(self: &Self) -> Vec<bool> {
        let len = self.updates.len();
        let mut failed = vec![false; len];

        for rule in self.rules.iter() {
            for update in 0..len {
                if failed[update] {
                    continue;
                }

                if let Some(s) = self.updates[update]
                    .steps
                    .iter()
                    .position(|&x| x == rule.last)
                {
                    if self.updates[update].steps[s..].contains(&rule.first) {
                        failed[update] = true;
                    }
                }
            }
        }
        failed
    }
}
pub fn run_day5() {
    let contents = fs::read_to_string("inputs/day5/input").expect("Could not read file");
    let puzzle = Puzzle::new(contents);
    let result1 = puzzle.run_part_1();
    let result2 = puzzle.run_part2();

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);
}
