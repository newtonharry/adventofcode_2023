use std::fs;

pub fn solve(file: &str) -> i32 {
    let file = fs::read_to_string(file).expect("Make sure data file is available");
    let combined_values = file.split('\n').map(|line| {
        let first = line.chars().find(|&x| x.is_numeric()).unwrap_or('0');
        let second = line.chars().rev().find(|&x| x.is_numeric()).unwrap_or('0');

        format!("{}{}", first, second)
    });

    combined_values
        .map(|value| value.parse::<i32>().unwrap())
        .sum()
}
