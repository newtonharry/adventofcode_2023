use super::part_a;
use super::part_b;

#[test]
fn test_part_a() {
    assert_eq!(
        part_a::solve("../problem_inputs/problem_1_part_a_test_input.txt"),
        142
    );
}

#[test]
fn test_part_b() {
    assert_eq!(
        part_b::solve("../problem_inputs/problem_1_part_b_test_input.txt"),
        281
    );
}
