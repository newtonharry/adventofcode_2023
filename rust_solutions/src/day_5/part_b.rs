use regex::Regex;
use std::fs;

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
    let data = fs::read_to_string(file).expect("Input needs to exist");
    let segments = data.split("\n\n").collect::<Vec<&str>>();
    let mut mappers: Vec<Mapper> = vec![Default::default(); 8];
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

    let mut intervals = seeds
        .chunks(2)
        .map(|x| Interval {
            level: 1,
            start: x[0],
            end: x[0] + (x[1] - 1),
        })
        .collect::<Vec<Interval>>();

    let segment_regex = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();
    for (index, segment) in segments.iter().enumerate() {
        for cap in segment_regex.captures_iter(segment) {
            let destination = cap[1].parse::<i64>().unwrap();
            let source = cap[2].parse::<i64>().unwrap();
            let range = cap[3].parse::<i64>().unwrap();

            mappers[index].destinations.push(destination);
            mappers[index].sources.push(source);
            mappers[index].ranges.push(range);
        }
    }

    let mut min_location = i64::MAX;
    while let Some(mut interval) = intervals.pop() {
        if interval.level == 8 {
            min_location = min_location.min(interval.start);
            continue;
        }

        let mapper = &mappers[interval.level as usize];
        let mut processed = false;

        for (index, source) in mapper.sources.iter().enumerate() {
            let source = *source;
            let destination = mapper.destinations[index];
            let range_end = source + mapper.ranges[index];
            let diff = destination - source;

            if interval.end <= source || interval.start >= range_end {
                continue;
            }

            if interval.start < source {
                intervals.push(Interval {
                    start: interval.start,
                    end: source,
                    level: interval.level,
                });
                interval.start = source;
            }

            if interval.end > range_end {
                intervals.push(Interval {
                    start: range_end,
                    end: interval.end,
                    level: interval.level,
                });
                interval.end = range_end;
            }

            intervals.push(Interval {
                start: destination + (interval.start - source),
                end: interval.end + diff,
                level: interval.level + 1,
            });

            processed = true;
            break;
        }

        if !processed {
            intervals.push(Interval {
                start: interval.start,
                end: interval.end,
                level: interval.level + 1,
            });
        }
    }

    min_location
}
