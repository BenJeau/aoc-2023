use std::borrow::Cow;

const STRING_NUMBERS: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

const REVERSE_STRING_NUMBERS: &[&str] = &[
    "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin",
];

fn get_leftmost_digit(mut chars: impl Iterator<Item = char>) -> usize {
    chars
        .find(|c| c.is_digit(10))
        .unwrap_or_default()
        .to_digit(10)
        .unwrap() as usize
}

fn calculate_sum_leftmost_and_rightmost(line: &str) -> usize {
    (get_leftmost_digit(line.chars()) * 10) + get_leftmost_digit(line.chars().rev())
}

fn replace_first_leftmost_string_number(line: &str, string_numbers: &[&str]) -> String {
    let mut a = vec![];

    let mut index = 0;
    while index < line.len() {
        let mut found = false;
        for (i, &n) in string_numbers.iter().enumerate() {
            if n.len() + index > line.len() {
                continue;
            }
            if n == &line[index..(n.len() + index)] {
                a.push((i + 1).to_string());
                index += n.len();
                found = true;
                break;
            }
        }
        if !found {
            a.push(line[index..(index + 1)].to_string());
            index += 1;
        } else {
            a.push(line[index..].to_string());
            break;
        }
    }

    a.join("")
}

fn reverse_string<'a>(s: impl Into<Cow<'a, str>>) -> String {
    s.into().chars().rev().collect()
}

fn solver(file_content: &str, f: fn(&str) -> usize) -> usize {
    file_content.split_whitespace().map(f).sum()
}

pub fn solve_part_1(file_content: &str) -> usize {
    solver(file_content, calculate_sum_leftmost_and_rightmost)
}

pub fn solve_part_2(file_content: &str) -> usize {
    solver(file_content, |s| {
        let first_part = replace_first_leftmost_string_number(s, STRING_NUMBERS);

        let reversed_s = reverse_string(s);
        let b = replace_first_leftmost_string_number(&reversed_s, REVERSE_STRING_NUMBERS);
        let second_part = reverse_string(b);

        calculate_sum_leftmost_and_rightmost(&format!("{first_part}{second_part}"))
    })
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_DATA_1: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

    const SAMPLE_DATA_2: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
eighthree
sevenine"#;

    #[test]
    fn test_get_leftmost_digit() {
        let result = get_leftmost_digit("1abc2".chars());
        assert_eq!(result, 1);

        let result = get_leftmost_digit("pqr3stu8vwx".chars());
        assert_eq!(result, 3);

        let result = get_leftmost_digit("a1b2c3d4e5f".chars());
        assert_eq!(result, 1);

        let result = get_leftmost_digit("treb7uchet".chars());
        assert_eq!(result, 7);
    }

    #[test]
    fn test_calculate_sum_leftmost_and_rightmost() {
        let result = calculate_sum_leftmost_and_rightmost("1abc2");
        assert_eq!(result, 12);

        let result = calculate_sum_leftmost_and_rightmost("pqr3stu8vwx");
        assert_eq!(result, 38);

        let result = calculate_sum_leftmost_and_rightmost("a1b2c3d4e5f");
        assert_eq!(result, 15);

        let result = calculate_sum_leftmost_and_rightmost("treb7uchet");
        assert_eq!(result, 77);
    }

    #[test]
    fn test_solve_part_1() {
        let result = solve_part_1(SAMPLE_DATA_1);
        assert_eq!(result, 142);
    }

    #[test]
    fn test_solve_part_2() {
        let result = solve_part_2(SAMPLE_DATA_2);
        assert_eq!(result, 281 + 83 + 79);
    }
}
