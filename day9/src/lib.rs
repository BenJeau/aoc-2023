fn solver(file_content: &str, reverse: bool) -> i32 {
    file_content
        .lines()
        .map(|line| {
            let mut nums = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            if reverse {
                nums.reverse();
            }

            let mut num = *nums.last().unwrap();

            while nums.windows(2).any(|w| w[0] != w[1]) {
                nums = nums.windows(2).map(|w| w[1] - w[0]).collect();
                num += *nums.last().unwrap();
            }

            num
        })
        .sum()
}

pub fn solve_part_1(file_content: &str) -> i32 {
    solver(file_content, false)
}

pub fn solve_part_2(file_content: &str) -> i32 {
    solver(file_content, true)
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_DATA: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(SAMPLE_DATA), 114);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2(SAMPLE_DATA), 2);
    }
}
