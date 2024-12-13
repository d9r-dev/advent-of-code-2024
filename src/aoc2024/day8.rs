use std::collections::HashMap;
use std::fs;

pub fn run_day8() {
    println!("Running day 8...");
    let contents = fs::read_to_string("inputs/day8/input").expect("Could not read file");
    part1(&contents);
}

pub fn parse_input(input: &String) -> Vec<Vec<char>> {
    let lines = input.lines().collect::<Vec<&str>>();
    lines.iter().map(|line| line.chars().collect()).collect()
}
fn part1(input: &String) {
    let positions = parse_input(input);
    let antennas: HashMap<char, Vec<(isize, isize)>> = get_antennas(&positions);
    let pairs: HashMap<char, Vec<((isize, isize), (isize, isize))>> = get_pairs(&antennas);
    let x_max = positions.get(0).unwrap().len();
    let y_max = positions.len();
    let fields = get_fields(&pairs, x_max as isize, y_max as isize);

    println!("Result: {:?}", fields)
}

fn get_fields(
    pairs: &HashMap<char, Vec<((isize, isize), (isize, isize))>>,
    x_max: isize,
    y_max: isize,
) -> i32 {
    let mut fields: Vec<(isize, isize)> = Vec::new();
    for (_, v) in pairs {
        for pair in v {
            if pair.0 == pair.1 {
                continue;
            }
            fields.push(pair.0);
            fields.push(pair.1);
            let distance = (pair.1 .0 - pair.0 .0, pair.1 .1 - pair.0 .1);
            let mut point = pair.1;
            loop {
                let new_point = (point.0 + distance.0, point.1 + distance.1);
                if new_point.0 < 0
                    || new_point.1 < 0
                    || new_point.0 >= x_max
                    || new_point.1 >= y_max
                {
                    break;
                } else {
                    fields.push(new_point);
                }
                point = new_point;
            }
        }
    }
    fields.sort();
    fields.dedup();
    fields.len() as i32
}

fn get_pairs(
    positions: &HashMap<char, Vec<(isize, isize)>>,
) -> HashMap<char, Vec<((isize, isize), (isize, isize))>> {
    let mut pairs = HashMap::new();
    for (k, v) in positions.iter() {
        let mut new_pairs = vec![];
        for (_, pos) in v.iter().enumerate() {
            for other_pos in v {
                new_pairs.push((*pos, *other_pos));
            }
        }

        let _ = pairs.insert(*k, new_pairs);
    }
    pairs
}

fn get_antennas(positions: &Vec<Vec<char>>) -> HashMap<char, Vec<(isize, isize)>> {
    let mut antennas: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    for (y, line) in positions.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if let '.' = char {
                continue;
            } else {
                match antennas.get_mut(&char) {
                    Some(v) => v.push((x as isize, y as isize)),
                    None => {
                        let _ = antennas.insert(*char, vec![(x as isize, y as isize)]);
                    }
                }
            }
        }
    }
    antennas
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse() {
        let test_input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
            .to_string();

        let map = get_antennas(&parse_input(&test_input));

        assert_eq!(*map.get(&'A').unwrap(), vec![(6, 5), (8, 8), (9, 9)]);
    }

    #[test]
    fn test_get_pairs() {
        let mut positions = HashMap::new();
        positions.insert('A', vec![(6, 5), (8, 8)]);
        positions.insert('B', vec![(6, 2), (3, 8), (7, 9)]);

        let map = get_pairs(&positions);

        assert_eq!(
            map.get(&'A').unwrap(),
            &vec![
                ((6, 5), (6, 5)),
                ((6, 5), (8, 8)),
                ((8, 8), (6, 5)),
                ((8, 8), (8, 8))
            ]
        );

        assert_eq!(
            map.get(&'B').unwrap(),
            &vec![
                ((6, 2), (6, 2)),
                ((6, 2), (3, 8)),
                ((6, 2), (7, 9)),
                ((3, 8), (6, 2)),
                ((3, 8), (3, 8)),
                ((3, 8), (7, 9)),
                ((7, 9), (6, 2)),
                ((7, 9), (3, 8)),
                ((7, 9), (7, 9))
            ]
        );
    }
}
