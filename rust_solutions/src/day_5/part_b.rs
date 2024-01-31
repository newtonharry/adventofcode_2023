use regex::Regex;
use std::fs;

use crate::{generate_puzzle_input_test, generate_test_input_test};

#[derive(Default, Debug, Clone)]
struct Mapper {
    destinations: Vec<i64>,
    sources: Vec<i64>,
    ranges: Vec<i64>,
}

#[derive(Debug)]
struct Interval {
    level: i64,
    start: i64,
    end: i64,
}

pub fn solve(file: &str) -> i64 {
    // Read the input file
    let data = fs::read_to_string(file).expect("Input needs to exist");

    // Split the data into segments
    let segments = data.split("\n\n").collect::<Vec<&str>>();

    // Create a vector of mappers, one for each segment
    let mut mappers: Vec<Mapper> = vec![Default::default(); 8];

    // Split the first line of the data to get the seeds
    let mut lines = data.lines();
    let seeds = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    // Create intervals based on the seeds
    let mut intervals = seeds
        .chunks(2)
        .map(|x| Interval {
            level: 1,
            start: x[0],
            end: x[0] + (x[1] - 1),
        })
        .collect::<Vec<Interval>>();

    // Create a regex pattern to match the segments
    let segment_regex = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();

    // Iterate over each segment and extract the destination, source, and range
    for (index, segment) in segments.iter().enumerate() {
        for cap in segment_regex.captures_iter(segment) {
            let destination = cap[1].parse::<i64>().unwrap();
            let source = cap[2].parse::<i64>().unwrap();
            let range = cap[3].parse::<i64>().unwrap();

            // Add the destination, source, and range to the corresponding mapper
            mappers[index].destinations.push(destination);
            mappers[index].sources.push(source);
            mappers[index].ranges.push(range);
        }
    }

    // Initialize the minimum location to the maximum possible value
    let mut min_location = i64::MAX;

    // Process the intervals
    while let Some(mut interval) = intervals.pop() {
        // If the interval level is 8, update the minimum location and continue
        // The starting interval value will always be the smallest, so we don't need to check any other number within the interval range
        if interval.level == 8 {
            min_location = min_location.min(interval.start);
            continue;
        }

        // Get the mapper for the current interval level
        let mapper = &mappers[interval.level as usize];
        let mut processed = false;

        // Iterate over the sources in the mapper
        for (index, source) in mapper.sources.iter().enumerate() {
            let source = *source;
            let destination = mapper.destinations[index];
            let range_end = source + mapper.ranges[index];
            let diff = destination - source;

            // If the interval is outside the range of the source, skip to the next source
            if interval.end <= source || interval.start >= range_end {
                continue;
            }

            // If the interval starts before the source, split the interval and add the first part
            if interval.start < source {
                intervals.push(Interval {
                    start: interval.start,
                    end: source,
                    level: interval.level,
                });
                // We are setting the start interval here so we can prepare to push the overlapping area to the next level
                interval.start = source;
            }

            // If the interval ends after the range, split the interval and add the second part
            if interval.end > range_end {
                intervals.push(Interval {
                    start: range_end,
                    end: interval.end,
                    level: interval.level,
                });
                // We are setting the end interval here so we can prepare to push the overlapping area to the next level
                interval.end = range_end;
            }

            // Add a new interval based on the destination and the difference between the source and destination
            // Use those variables we set earlier in the two if statments above to push the overlapping area to the next level to be mapped
            // This is essentially hitting two birds with one stone, which is where the elegance of the solution is in my opinion.
            intervals.push(Interval {
                start: destination + (interval.start - source), // This line computes to the same one below but just in a more intuitive fashion (imo)
                end: interval.end + diff,
                level: interval.level + 1,
            });

            processed = true;
            break;
        }

        // If none of the sources were processed, add the interval to the next level
        if !processed {
            intervals.push(Interval {
                start: interval.start,
                end: interval.end,
                level: interval.level + 1,
            });
        }
    }

    // Return the minimum location
    min_location
}

generate_test_input_test!(5, 46);
generate_puzzle_input_test!(5, 60568880);
