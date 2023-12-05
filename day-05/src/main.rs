use std::{collections::HashMap, fs::read_to_string, ops::Range};

fn main() {
    let input = read_to_string("./input.txt").expect("File should exist");
    // let input = read_to_string("./limited-input.txt").expect("File should exist");
    // let input = read_to_string("./smaller-input.txt").expect("File should exist");
    // let input = read_to_string("./test-input.txt").expect("File should exist");
    println!("Part one result is {}", part_one(&input));
    println!("Part two result is {}", part_two(&input));
}

fn part_one(input: &str) -> u64 {
    let seeds = parse_seeds(input.lines().next().unwrap());
    let seed_soil = parse_map(input, "seed-to-soil");
    let soil_fert = parse_map(input, "soil-to-fertilizer");
    let fert_water = parse_map(input, "fertilizer-to-water");
    let water_light = parse_map(input, "water-to-light");
    let light_temp = parse_map(input, "light-to-temperature");
    let temp_hum = parse_map(input, "temperature-to-humidity");
    let hum_loc = parse_map(input, "humidity-to-location");

    let loc = seeds
        .iter()
        .map(|seed| {
            let loc = find_location_from_seed(
                seed,
                &seed_soil,
                &soil_fert,
                &fert_water,
                &water_light,
                &light_temp,
                &temp_hum,
                &hum_loc,
            );
            return loc;
        })
        .min();

    return loc.unwrap();
}

fn part_two(input: &str) -> u64 {
    let seed_ranges = parse_seed_ranges(input.lines().next().unwrap());
    let seed_soil = parse_map(input, "seed-to-soil");
    let soil_fert = parse_map(input, "soil-to-fertilizer");
    let fert_water = parse_map(input, "fertilizer-to-water");
    let water_light = parse_map(input, "water-to-light");
    let light_temp = parse_map(input, "light-to-temperature");
    let temp_hum = parse_map(input, "temperature-to-humidity");
    let hum_loc = parse_map(input, "humidity-to-location");

    let mut min_loc = 10_000_000_000;
    let mut min_seed = 0;
    for (j, seed_range) in seed_ranges.into_iter().enumerate() {
        dbg!(&seed_range);
        for (i, seed) in seed_range.into_iter().enumerate() {
            if j == 4 || i == 0 || i % 50000 == 0 {
                let loc = find_location_from_seed(
                    seed,
                    &seed_soil,
                    &soil_fert,
                    &fert_water,
                    &water_light,
                    &light_temp,
                    &temp_hum,
                    &hum_loc,
                );
                if loc < min_loc {
                    println!(
                        "Location of seed {} is {}, it belongs to range starting at {}",
                        seed, loc, j
                    );
                    min_loc = loc;
                    min_seed = seed;
                }
            }
        }
    }

    return min_loc;
}

fn find_location_from_seed(
    seed: u64,
    seed_soil: &RangeList,
    soil_fert: &RangeList,
    fert_water: &RangeList,
    water_light: &RangeList,
    light_temp: &RangeList,
    temp_hum: &RangeList,
    hum_loc: &RangeList,
) -> u64 {
    let soil = seed_soil.find_in_ranges(&seed).unwrap_or(seed);
    // println!("Looking for soil by seed found soil {}", soil);
    let fert = soil_fert.find_in_ranges(&soil).unwrap_or(soil);
    // println!("Looking for fert by soil found fert {}", fert);
    let water = fert_water.find_in_ranges(&fert).unwrap_or(fert);
    // println!("Looking for water by fert found water {}", water);
    let light = water_light.find_in_ranges(&water).unwrap_or(water);
    // println!("Looking for light by water found light {}", light);
    let temp = light_temp.find_in_ranges(&light).unwrap_or(light);
    // println!("Looking for temp by light found temp {}", temp);
    let hum = temp_hum.find_in_ranges(&temp).unwrap_or(temp);
    // println!("Looking for hum by temp found hum {}", temp);
    let loc = hum_loc.find_in_ranges(&hum).unwrap_or(hum);
    // println!("Looking for loc by hum found loc {}", loc);
    return loc;
}

fn parse_seeds(row: &str) -> Vec<u64> {
    let seeds = row
        .split_once(':')
        .unwrap()
        .1
        .split(' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    return seeds;
}

fn parse_seed_ranges(row: &str) -> Vec<Range<u64>> {
    let mut raw_seeds = row
        .split_once(':')
        .unwrap()
        .1
        .split(' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap())
        .into_iter();

    let mut seed_ranges: Vec<Range<u64>> = Vec::new();

    loop {
        let start = raw_seeds.next();
        let count = raw_seeds.next();
        if start.is_none() || count.is_none() {
            break;
        }
        seed_ranges.push(start.unwrap()..(start.unwrap() + count.unwrap()));
    }
    return seed_ranges;
}

fn parse_map(input: &str, key: &str) -> RangeList {
    let mut found = false;
    let mut vec: Vec<ValRange> = Vec::new();
    for line in input.lines() {
        if !found {
            found = line.find(key).is_some();
        } else if found && line.is_empty() {
            break;
        } else if found {
            let r = parse_data_row(line);
            vec.push(r);
        }
    }

    let range_list = RangeList { list: vec };

    return range_list;
}

fn parse_data_row(row: &str) -> ValRange {
    let columns: Vec<u64> = row.split(' ').map(|s| s.parse::<u64>().unwrap()).collect();

    return ValRange {
        dest_start: *columns.get(0).unwrap(),
        source_start: *columns.get(1).unwrap(),
        range_length: *columns.get(2).unwrap(),
    };
}

#[derive(Debug)]
struct RangeList {
    list: Vec<ValRange>,
}

impl RangeList {
    fn find_in_ranges(&self, key: &u64) -> Option<u64> {
        self.list
            .iter()
            .find(|r| r.contains(key))
            .map(|r| r.get(key))
            .flatten()
    }
}
#[derive(Debug)]
struct ValRange {
    dest_start: u64,
    source_start: u64,
    range_length: u64,
}

impl ValRange {
    fn contains(&self, key: &u64) -> bool {
        return (self.source_start..self.source_start + self.range_length).contains(key);
    }

    fn get(&self, key: &u64) -> Option<u64> {
        let idx = key - self.source_start;
        if self.contains(key) {
            // println!(
            //     "Found {} in range {}, {}, {}, returning {}",
            //     key,
            //     self.dest_start,
            //     self.source_start,
            //     self.range_length,
            //     self.dest_start + idx,
            // );
            return Some(self.dest_start + idx);
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use crate::{parse_data_row, parse_map};

    #[test]
    fn test_parse_map() {
        // Given
        let input = read_to_string("./test-input.txt").expect("File should exist");

        // When
        let result = parse_map(&input, "seed-to-soil");

        // Then
        assert_eq!(result.find_in_ranges(&50u64).unwrap(), 98u64);
        assert_eq!(result.find_in_ranges(&51u64).unwrap(), 99u64);
        assert_eq!(result.find_in_ranges(&99u64).unwrap(), 97u64);
    }
}
