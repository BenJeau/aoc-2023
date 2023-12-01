use day1::solve_part_1;

fn main() {
    let file_content = include_str!("../../input_1");
    let answer = solve_part_1(file_content);

    println!("Answer part 1: {answer}");
}
