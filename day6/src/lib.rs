fn num_of_winning_races((time, distance): (&usize, &usize)) -> usize {
    (0..=*time).filter(|i| (time - i) * i > *distance).count()
}

pub fn solve_part_1(file_content: &str) -> usize {
    let raw_races = file_content
        .lines()
        .map(str::split_whitespace)
        .map(|line| line.skip(1).map(str::parse).map(Result::unwrap).collect())
        .collect::<Vec<Vec<_>>>();

    let races = raw_races[0].iter().zip(raw_races[1].iter());
    races.map(num_of_winning_races).product()
}

pub fn solve_part_2(file_content: &str) -> usize {
    let race = file_content
        .lines()
        .map(str::split_whitespace)
        .map(|line| line.skip(1).collect::<Vec<_>>().join("").parse())
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    num_of_winning_races((&race[0], &race[1]))
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_DATA: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(SAMPLE_DATA), 288);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2(SAMPLE_DATA), 71503);
    }
}
