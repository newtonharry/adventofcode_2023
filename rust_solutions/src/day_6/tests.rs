use super::part_a;
use super::part_b;

#[test]
fn test_input_part_a() {
    println!(
        "{}",
        part_a::solve("../problem_inputs/problem_6_test_input.txt")
    );
}

#[test]
fn puzzle_input_part_a() {
    let sum = part_a::solve("../problem_inputs/problem_6_puzzle_input.txt");
    println!("{}", sum);
}

#[test]
fn test_input_part_b() {
    let sum = part_b::solve("../problem_inputs/problem_6_test_input.txt");
    println!("{}", sum);
}

#[test]
fn puzzle_input_part_b() {
    let sum = part_b::solve("../problem_inputs/problem_6_puzzle_input.txt");
    println!("{}", sum);
}
