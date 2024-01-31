use regex::Regex;
use std::fs;

use crate::{generate_puzzle_input_test, generate_test_input_test};

fn get_numbers<'a>(regex: &'a Regex, text: &'a str) -> impl Iterator<Item = usize> + 'a {
    regex
        .find_iter(text)
        .filter_map(|mat| mat.as_str().parse::<usize>().ok())
}

pub fn solve(file: &str) -> usize {
    let data = fs::read_to_string(file).expect("Input needs to exist");
    let lines = data.lines().collect::<Vec<&str>>();
    let numbers_regex = Regex::new(r"\d+").unwrap();
    let times = get_numbers(&numbers_regex, lines[0]);
    let distances = get_numbers(&numbers_regex, lines[1]);

    let times_dist = times.zip(distances);

    let mut num_of_beats = vec![];

    for (race_time, record_dist) in times_dist {
        let (mut low, mut high) = (1, race_time / 2);
        // Perform binary search instead of linear search: O(n/2)
        while low < high {
            let mid = low + (high - low) / 2;
            let distance_traveled = (race_time - mid) * mid;
            if distance_traveled <= record_dist {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        let wins = if low == 0 {
            0
        } else {
            race_time - (low * 2) + 1
        };
        num_of_beats.push(wins);
    }

    num_of_beats.iter().product()
}

generate_test_input_test!(6, 288);
generate_puzzle_input_test!(6, 140220);
