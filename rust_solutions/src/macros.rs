// Macro to generate a test for a given problem and day
#[macro_export]
macro_rules! generate_test_input_test {
    ($input:tt, $answer:literal) => {
        #[test]
        fn test_input() {
            assert_eq!(
                self::solve(&format!(
                    "../problem_inputs/problem_{}_test_input.txt",
                    $input
                )),
                $answer
            );
        }
    };
}

#[macro_export]
macro_rules! generate_test_input_custom_input {
    ($input:tt, $answer:literal) => {
        #[test]
        fn test_input() {
            assert_eq!(self::solve($input), $answer);
        }
    };
}

#[macro_export]
macro_rules! generate_puzzle_input_test {
    ($input:tt, $answer:literal) => {
        #[test]
        fn puzzle_input() {
            assert_eq!(
                self::solve(&format!(
                    "../problem_inputs/problem_{}_puzzle_input.txt",
                    $input
                )),
                $answer
            );
        }
    };
}

#[macro_export]
macro_rules! generate_puzzle_input_custom_input {
    ($input:tt, $answer:literal) => {
        #[test]
        fn puzzle_input() {
            assert_eq!(self::solve($input), $answer);
        }
    };
}
