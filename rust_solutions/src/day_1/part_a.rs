use crate::utils::read_file;

pub fn solve(file: &str) -> i32 {
    let file = read_file(file);
    let combined_values = file.iter().map(|line| {
        let first = line.chars().find(|&x| x.is_numeric()).unwrap_or('0');
        let second = line.chars().rev().find(|&x| x.is_numeric()).unwrap_or('0');
        format!("{}{}", first, second)
    });

    combined_values
        .map(|value| value.parse::<i32>().unwrap())
        .sum()
}
