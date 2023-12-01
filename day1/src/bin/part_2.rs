use day1::solve_part_2;

fn main() {
    let file_content = include_str!("../../input_2");
    let answer = solve_part_2(file_content);

    println!("Answer part 2: {answer}");
}
