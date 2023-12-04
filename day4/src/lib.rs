use std::collections::{BTreeMap, HashSet};

fn create_hash_set_from_line(line: &str) -> HashSet<usize> {
    let mut numbers = HashSet::new();

    for number in line.split_whitespace() {
        numbers.insert(number.parse::<usize>().unwrap());
    }

    numbers
}

fn get_number_of_matches(line: &str) -> usize {
    let (card_winning, card_numbers) = line.split_once(":").unwrap().1.split_once("|").unwrap();

    let winning_numbers = create_hash_set_from_line(card_winning);
    let numbers = create_hash_set_from_line(card_numbers);

    winning_numbers.intersection(&numbers).count()
}

pub fn solve_part_1(file_content: &str) -> usize {
    file_content
        .lines()
        .map(get_number_of_matches)
        .map(|number_of_matches| {
            if number_of_matches == 0 {
                return 0;
            }

            2usize.pow((number_of_matches - 1) as u32) as usize
        })
        .sum()
}

pub fn solve_part_2(file_content: &str) -> usize {
    let mut buffer = BTreeMap::new();

    file_content
        .lines()
        .map(get_number_of_matches)
        .enumerate()
        .fold(0, |acc, (index, number_of_matches)| {
            let count_of_index_in_buffer = buffer.get(&index).copied().unwrap_or(0) + 1;

            for i in (index)..(index + number_of_matches) {
                for _ in 0..count_of_index_in_buffer {
                    buffer.entry(i + 1).or_insert(0);
                    *buffer.get_mut(&(i + 1)).unwrap() += 1;
                }
            }

            acc + count_of_index_in_buffer
        })
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_DATA: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(SAMPLE_DATA), 13);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2(SAMPLE_DATA), 30);
    }
}
