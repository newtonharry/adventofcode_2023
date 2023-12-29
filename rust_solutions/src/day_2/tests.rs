use super::part_a;
use super::part_b;

#[test]
fn test_input_part_a() {
    assert_eq!(
        part_a::solve("../problem_inputs/problem_2_test_input.txt"),
        8
    );
}

#[test]
fn puzzle_input_part_a() {
    assert_eq!(
        part_a::solve("../problem_inputs/problem_2_puzzle_input.txt"),
        1931
    );
}

#[test]
fn test_input_part_b() {
    assert_eq!(
        part_b::solve("../problem_inputs/problem_2_test_input.txt"),
        2286
    );
}

#[test]
fn puzzle_input_part_b() {
    assert_eq!(
        part_b::solve("../problem_inputs/problem_2_puzzle_input.txt"),
        83105
    );
}
