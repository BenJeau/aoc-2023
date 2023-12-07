next_day=$(ls -d day* | sort -V | tail -1 | sed 's/day//g' | awk '{print $1+1}')
cargo new day${next_day} --lib

# create input file
touch day${next_day}/input

# create bin files
mkdir day${next_day}/src/bin

generate_script() {
    local part=$1
    echo 'fn main() {
    let answer = day'${next_day}'::solve_part_'${part}'(include_str!("../../input"));

    println!("Answer part '${part}': {answer}");
}' >day${next_day}/src/bin/part_${part}.rs
}

generate_script 1
generate_script 2

# create lib file
echo 'pub fn solve_part_1(file_content: &str) -> usize {
    0
}

pub fn solve_part_2(file_content: &str) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_DATA: &str = r#""#;

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(SAMPLE_DATA), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2(SAMPLE_DATA), 0);
    }
}' >day${next_day}/src/lib.rs
