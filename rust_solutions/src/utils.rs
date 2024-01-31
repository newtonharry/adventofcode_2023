use std::fs;

pub fn read_file(file: &str) -> Vec<String> {
    let data = fs::read_to_string(file).expect("Expect test/puzzle input");
    data.lines().map(|line| line.to_string()).collect()
}
