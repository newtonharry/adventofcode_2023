use std::fs;

#[derive(Default, Debug)]
struct Mapper {
    destinations: Vec<i64>,
    sources: Vec<i64>,
    ranges: Vec<i64>,
}

impl Mapper {
    fn map(&mut self, input: i64) -> i64 {
        println!("Input is {}", input);
        // Try and find the input in the hashmap
        // Try and find the input in the source ranges
        let mut val = input;
        for (index, source) in self.sources.iter().enumerate() {
            let range = self.ranges[index];
            if input >= *source && input < (*source + range) {
                println!("Found input in source range");
                // Need to perform analytical mapping here
                val = self.destinations[index] + (input - source);
                break;
            }
        }
        return val;
    }

    fn build_mapper(&mut self, lines: &mut std::str::Lines) {
        lines.nth(0);
        while let Some(line) = lines.next() {
            if line.starts_with(|c: char| c.is_ascii_digit()) {
                let numbers = line
                    .trim()
                    .split(' ')
                    .map(|c| c.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>();
                let (destination, source, range) = (numbers[0], numbers[1], numbers[2]);
                self.destinations.push(destination);
                self.sources.push(source);
                self.ranges.push(range);
            } else {
                break;
            }
        }
    }
}

pub fn solve(file: &str) -> i64 {
    let data = fs::read_to_string(file).expect("Input needs to exist");
    let mut lines = data.lines();
    let mut nums = vec![];
    // First get the seed numbers
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

    let mut soil_mapper = Mapper::default();
    let mut fert_mapper = Mapper::default();
    let mut water_mapper = Mapper::default();
    let mut light_mapper = Mapper::default();
    let mut temp_mapper = Mapper::default();
    let mut humidity_mapper = Mapper::default();
    let mut location_mapper = Mapper::default();

    lines.next();

    soil_mapper.build_mapper(&mut lines);
    fert_mapper.build_mapper(&mut lines);
    water_mapper.build_mapper(&mut lines);
    light_mapper.build_mapper(&mut lines);
    temp_mapper.build_mapper(&mut lines);
    humidity_mapper.build_mapper(&mut lines);
    location_mapper.build_mapper(&mut lines);

    for seed in seeds {
        let soil = soil_mapper.map(seed);
        let fert = fert_mapper.map(soil);
        let water = water_mapper.map(fert);
        let light = light_mapper.map(water);
        let temp = temp_mapper.map(light);
        let humidity = humidity_mapper.map(temp);
        let location = location_mapper.map(humidity);
        nums.push(location);
        println!(
            "Seed: {}, Soil: {}, Fert: {}, Water: {}, Light: {}, Temp: {}, Humidity: {}, Location: {}",
            seed, soil, fert, water, light, temp, humidity, location
        );
    }

    // return smallest number from nums
    *(nums.iter().min().unwrap())
}
