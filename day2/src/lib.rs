use std::collections::BTreeMap;

const RED: &str = "red";
const GREEN: &str = "green";
const BLUE: &str = "blue";

fn common(file_content: &str, solver: fn(&str, usize) -> Option<usize>) -> usize {
    file_content
        .lines()
        .enumerate()
        .flat_map(|(index, line)| {
            let (_, content) = line.split_once(": ").unwrap();
            solver(content, index)
        })
        .sum()
}

fn split_round(game: &str) -> impl Iterator<Item = (&str, usize)> {
    game.split(", ").map(|card| {
        let (count, color) = card.split_once(" ").unwrap();
        (color, count.parse::<usize>().unwrap())
    })
}

fn part_1_solver(content: &str, index: usize) -> Option<usize> {
    let constraints = BTreeMap::from([(RED, 12), (GREEN, 13), (BLUE, 14)]);

    let satisfies_contraints = content
        .split("; ")
        .flat_map(split_round)
        .all(|(color, count)| count <= *constraints.get(color).unwrap());

    if satisfies_contraints {
        Some(index + 1)
    } else {
        None
    }
}

pub fn solve_part_1(file_content: &str) -> usize {
    common(file_content, part_1_solver)
}

fn part_2_solver(content: &str, _: usize) -> Option<usize> {
    let answer = content
        .split("; ")
        .map(split_round)
        .map(Iterator::collect::<BTreeMap<_, _>>)
        .fold(
            BTreeMap::from([(RED, 0), (GREEN, 0), (BLUE, 0)]),
            |mut acc, game| {
                for (color, count) in game {
                    if let Some(x) = acc.get_mut(color) {
                        *x = count.max(*x);
                    }
                }

                acc
            },
        )
        .values()
        .product::<usize>();

    Some(answer)
}

pub fn solve_part_2(file_content: &str) -> usize {
    common(file_content, part_2_solver)
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_DATA: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(SAMPLE_DATA), 8);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2(SAMPLE_DATA), 2286);
    }
}
