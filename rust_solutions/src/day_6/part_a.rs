use regex::Regex;
use std::fs;

fn get_numbers(text: &str) -> Vec<usize> {
    let numbers_regex = Regex::new(r"\d+").unwrap();
    numbers_regex
        .find_iter(text)
        .filter_map(|mat| mat.as_str().parse::<usize>().ok())
        .collect()
}

pub fn solve(file: &str) -> usize {
    let data = fs::read_to_string(file).expect("Input needs to exist");
    let lines = data.lines().collect::<Vec<&str>>();
    let times = get_numbers(lines[0]);
    let distances = get_numbers(lines[1]);

    let times_dist = times
        .iter()
        .zip(distances.iter())
        .collect::<Vec<(&usize, &usize)>>();

    let mut num_of_beats = vec![];

    for (time, record_dist) in times_dist {
        let mut lower_limit = 0;
        for time_held in 1..*time {
            let distance_traveled = (*time - time_held) * time_held;
            if distance_traveled > *record_dist {
                lower_limit = time_held;
                break; // Break as soon as a valid lower limit is found
            }
        }
        let wins = if lower_limit == 0 {
            0
        } else {
            *time - (lower_limit * 2) + 1
        };
        num_of_beats.push(wins);
    }

    num_of_beats.iter().product()
}
