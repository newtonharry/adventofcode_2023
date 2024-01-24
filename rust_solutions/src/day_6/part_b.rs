use std::fs;

use regex::{Captures, Regex};

fn get_number<'a>(text: &str) -> u64 {
    let numbers_regex = Regex::new(r"\d+").unwrap();
    numbers_regex
        .find_iter(text)
        .filter_map(|mat| mat.as_str().into())
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .unwrap()
}

pub fn solve(file: &str) -> u64 {
    let data = fs::read_to_string(file).expect("Input needs to exist");
    let lines = data.lines().collect::<Vec<&str>>();
    let race_time = get_number(lines[0]);
    let distance_record = get_number(lines[1]);
    let mut lower_limit = 0;

    for time_button_held in 1..race_time {
        let distance_traveled = (race_time - time_button_held) * time_button_held;
        if distance_traveled > distance_record {
            // We have found the lowest limit
            lower_limit = time_button_held;
            break;
        }
    }
    (race_time - (lower_limit * 2)) + 1
}
