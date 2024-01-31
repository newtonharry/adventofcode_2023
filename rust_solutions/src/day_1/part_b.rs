use crate::{generate_puzzle_input_test, generate_test_input_custom_input, utils::read_file};

pub fn solve(file: &str) -> i32 {
    let file = read_file(file);
    let mut cleansed_data: Vec<String> = Vec::new();
    let mut new_line = String::new();
    for line in file {
        for i in 0..line.len() {
            let slice = &line[i..];
            match slice {
                _ if slice.starts_with("one") => new_line.push('1'),
                _ if slice.starts_with("two") => new_line.push('2'),
                _ if slice.starts_with("three") => new_line.push('3'),
                _ if slice.starts_with("four") => new_line.push('4'),
                _ if slice.starts_with("five") => new_line.push('5'),
                _ if slice.starts_with("six") => new_line.push('6'),
                _ if slice.starts_with("seven") => new_line.push('7'),
                _ if slice.starts_with("eight") => new_line.push('8'),
                _ if slice.starts_with("nine") => new_line.push('9'),
                _ => {
                    // If no match, move to the next character
                    if let Some(val) = line.chars().nth(i) {
                        new_line.push(val);
                    }
                }
            };
        }
        cleansed_data.push(new_line);
        new_line = String::new();
    }

    let combined_values = cleansed_data.iter().map(|line| {
        let first = line.chars().find(|&x| x.is_numeric()).unwrap();
        let second = line.chars().rev().find(|&x| x.is_numeric()).unwrap_or('0');

        format!("{}{}", first, second)
    });

    combined_values
        .map(|value| value.parse::<i32>().unwrap())
        .sum()
}

generate_test_input_custom_input!("../problem_inputs/problem_1_part_b_test_input.txt", 281);
generate_puzzle_input_test!(1, 55413);
