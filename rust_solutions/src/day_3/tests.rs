use super::part_a;

#[test]
fn test_input_part_a() {
    assert_eq!(
        part_a::solve("../problem_inputs/problem_3_test_input.txt"),
        4361
    );
}

#[test]
fn puzzle_input_part_a() {
    let num = part_a::solve("../problem_inputs/problem_3_puzzle_input.txt");
    println!("{}", num);
}
