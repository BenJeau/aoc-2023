use std::collections::HashMap;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone)]
enum Move {
    Right,
    Left,
}

impl From<char> for Move {
    fn from(s: char) -> Self {
        match s {
            'R' => Move::Right,
            'L' => Move::Left,
            _ => panic!("Invalid move: {}", s),
        }
    }
}

fn common(file_content: &str) -> (Vec<Move>, HashMap<&str, (&str, &str)>) {
    let (moves, map) = file_content.split_once("\n\n").unwrap();

    let moves = moves.chars().map(Move::from).collect::<Vec<_>>();

    let map = map.lines().fold(HashMap::new(), |mut map, line| {
        let (source, destinations) = line.split_once(" = ").unwrap();

        let (left, right) = destinations.split_once(", ").unwrap();

        let left = left.trim_start_matches('(');
        let right = right.trim_end_matches(')');

        map.insert(source, (left, right));

        map
    });

    (moves, map)
}

pub fn solve_part_1(file_content: &str) -> usize {
    let (moves, map) = common(file_content);

    let mut count = 0;
    let mut current = "AAA";

    loop {
        for m in &moves {
            count += 1;
            let (left, right) = map.get(current).unwrap();

            current = match m {
                Move::Right => right,
                Move::Left => left,
            };

            if current == "ZZZ" {
                return count;
            }
        }
    }
}

pub fn solve_part_2(file_content: &str) -> usize {
    let (moves, map) = common(file_content);

    let mut count: usize = 0;
    let mut currents = map
        .keys()
        .cloned()
        .filter(|k| k.ends_with('A'))
        .collect::<Vec<_>>();

    let mut current_first_z = vec![0; currents.len()];

    loop {
        for m in &moves {
            count += 1;

            for (index, current) in &mut currents.iter_mut().enumerate() {
                let (left, right) = map.get(current).unwrap();

                *current = match m {
                    Move::Right => right,
                    Move::Left => left,
                };

                if current.ends_with('Z') {
                    current_first_z[index] = count;
                }
            }

            if current_first_z.iter().all(|c| *c != 0) {
                return lcm_list(&current_first_z);
            }
        }
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn lcm_list(numbers: &[usize]) -> usize {
    numbers.iter().fold(1, |a, &b| lcm(a, b))
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_DATA: &str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;

    const SAMPLE_DATA_OTHER: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;

    const SAMPLE_DATA_2: &str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(SAMPLE_DATA), 2);
    }

    #[test]
    fn test_part_1_other() {
        assert_eq!(solve_part_1(SAMPLE_DATA_OTHER), 6);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2(SAMPLE_DATA_2), 6);
    }
}
