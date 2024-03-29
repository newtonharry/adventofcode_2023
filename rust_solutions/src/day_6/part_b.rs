use std::fs;

use regex::Regex;

use crate::{generate_puzzle_input_test, generate_test_input_test};

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

    // Can half the search space given as the distance record increases,it approaches the maxima of the quadratic function, before becoming impossible
    let (mut low, mut high) = (1, race_time / 2);

    // Perform binary search instead of linear search to find the smallest value which wins
    while low < high {
        let mid = low + (high - low) / 2; // Find the x value we are interested in
        let distance_traveled = (race_time - mid) * mid; // <-- quadratic function (plot on desmos), plug x value in produce distance traveled
        if distance_traveled <= distance_record {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    (race_time - (low * 2)) + 1 // We can multiply the low value by 2 because it is derived from a quadratic function (symmetry)
}

generate_test_input_test!(6, 71503);
generate_puzzle_input_test!(6, 39570185);
