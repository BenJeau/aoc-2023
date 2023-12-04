use std::collections::BTreeMap;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: isize,
    y: isize,
}

impl From<(isize, isize)> for Point {
    fn from((x, y): (isize, isize)) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Symbol {
    point: Point,
    is_gear: bool,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn contains_point(&self, other: &Point) -> bool {
        self.top_left.x <= other.x
            && self.top_left.y <= other.y
            && self.bottom_right.x >= other.x
            && self.bottom_right.y >= other.y
    }

    fn find_a_symbol<'a>(&'a self, symbols: &'a [Symbol]) -> Option<&'a Symbol> {
        symbols
            .iter()
            .find(|symbol| self.contains_point(&symbol.point))
    }
}

struct Part {
    area: Rectangle,
    value: usize,
}

fn parse_parts_and_symbols(file_content: &str) -> (Vec<Part>, Vec<Symbol>) {
    let mut symbols_points = vec![];

    let potential_parts = file_content
        .lines()
        .map(|line| format!(".{}.", line))
        .enumerate()
        .flat_map(|(row_index, line)| {
            let mut rectangles = vec![];

            let mut current_index = -1;
            let mut current_value = vec![];

            line.chars().enumerate().for_each(|(col_index, char)| {
                if let Some(digit) = char.to_digit(10) {
                    if current_index == -1 {
                        current_index = col_index as isize;
                    }
                    current_value.push(digit);
                } else {
                    if current_index != -1 {
                        rectangles.push(Part {
                            area: Rectangle {
                                top_left: (current_index - 1, row_index as isize - 1).into(),
                                bottom_right: (col_index as isize, row_index as isize + 1).into(),
                            },
                            value: current_value
                                .iter()
                                .fold(0, |acc, digit| acc * 10 + *digit as usize),
                        });

                        current_index = -1;
                        current_value.clear();
                    }

                    if char != '.' {
                        symbols_points.push(Symbol {
                            point: (col_index as isize, row_index as isize).into(),
                            is_gear: char == '*',
                        });
                    }
                }
            });

            rectangles
        })
        .collect();

    (potential_parts, symbols_points)
}

pub fn solve_part_1(file_content: &str) -> usize {
    let (potential_parts, symbols_points) = parse_parts_and_symbols(file_content);

    potential_parts
        .into_iter()
        .flat_map(|part| part.area.find_a_symbol(&symbols_points).map(|_| part.value))
        .sum()
}

pub fn solve_part_2(file_content: &str) -> usize {
    let (potential_parts, symbols_points) = parse_parts_and_symbols(file_content);

    let gears = symbols_points
        .into_iter()
        .filter(|s| s.is_gear)
        .collect::<Vec<_>>();

    potential_parts
        .iter()
        .flat_map(|part| {
            part.area
                .find_a_symbol(&gears)
                .map(|point| (point, part.value))
        })
        .fold(BTreeMap::new(), |mut acc, (point, value)| {
            acc.entry(point).or_insert(vec![]).push(value);
            acc
        })
        .values()
        .filter(|values| values.len() > 1)
        .map(|values| values.iter().product::<usize>())
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_DATA: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    const MODIFIED_SAMPLE_DATA: &str = r#"467..114..
...0......
..35..633
......#...
617*......
.....+.58.+
..592.....
......755.
...$.*....
.664.598.."#;

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(SAMPLE_DATA), 4361);
    }

    #[test]
    fn test_part_1_modified() {
        assert_eq!(solve_part_1(MODIFIED_SAMPLE_DATA), 4361 - 35 - 467);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2(SAMPLE_DATA), 467835);
    }

    #[test]
    fn test_part_2_modified() {
        assert_eq!(solve_part_2(MODIFIED_SAMPLE_DATA), 467835 - (35 * 467));
    }
}
