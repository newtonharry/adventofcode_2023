use super::part_a;
use super::part_b;

#[test]
fn test_input_part_a() {
    assert_eq!(
        part_a::solve("../problem_inputs/problem_4_test_input.txt"),
        13
    );
}

#[test]
fn puzzle_input_part_a() {
    let sum = part_a::solve("../problem_inputs/problem_4_puzzle_input.txt");
    println!("{}", sum);
}

#[test]
fn test_input_part_b() {
    let sum = part_b::solve("../problem_inputs/problem_4_puzzle_input.txt");
    println!("{}", sum);
    // assert_eq!(
    //     part_b::solve("../problem_inputs/problem_4_test_input.txt"),
    //     13
    // );
}
