use std::collections::BTreeMap;

use nom::{
    bytes::complete::tag,
    character::complete::{self, multispace0, newline, space0, space1},
    multi::{count, many1, separated_list1},
    sequence::{preceded, terminated, tuple},
    IResult,
};

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> u64 {
    let (_, almanac) = parse(input).unwrap();
    let locations = almanac
        .seeds
        .iter()
        .map(|seed| almanac.seed_to_location(*seed))
        .min()
        .unwrap();

    locations
}

fn parse(input: &str) -> IResult<&str, Alamanac> {
    let (remaining, seeds) = parse_seeds(input)?;
    let (remaining, seed_to_soil_map) = parse_map(remaining, "seed-to-soil map:")?;
    let (remaining, soil_to_fertilizer_map) = parse_map(remaining, "soil-to-fertilizer map:")?;
    let (remaining, fertilizer_to_water_map) = parse_map(remaining, "fertilizer-to-water map:")?;
    let (remaining, water_to_light_map) = parse_map(remaining, "water-to-light map:")?;
    let (remaining, light_to_temperature_map) = parse_map(remaining, "light-to-temperature map:")?;
    let (remaining, temperature_to_humidity_map) =
        parse_map(remaining, "temperature-to-humidity map:")?;
    let (remaining, humidity_to_location_map) = parse_map(remaining, "humidity-to-location map:")?;

    Ok((
        remaining,
        Alamanac {
            seeds,
            seed_to_soil_map,
            soil_to_fertilizer_map,
            fertilizer_to_water_map,
            water_to_light_map,
            light_to_temperature_map,
            temperature_to_humidity_map,
            humidity_to_location_map,
        },
    ))
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    let seed_parser = preceded(
        tuple((tag("seeds:"), multispace0)),
        separated_list1(space1, complete::u64),
    );
    terminated(seed_parser, count(newline, 2))(input)
}

fn parse_map<'a>(input: &'a str, separator: &str) -> IResult<&'a str, Vec<Range>> {
    let (remaining, _) = terminated(tag(separator), newline)(input)?;
    let (remaining, ranges) =
        terminated(separated_list1(newline, parse_range), count(newline, 2))(remaining)?;

    Ok((remaining, ranges))
}

fn parse_range(input: &str) -> IResult<&str, Range> {
    let (remaining, (destination_range_start, _, source_range_start, _, range_length)) =
        tuple((complete::u64, space1, complete::u64, space1, complete::u64))(input)?;

    Ok((
        remaining,
        Range {
            destination_range_start,
            source_range_start,
            range_length,
        },
    ))
}

#[derive(Debug)]
struct Alamanac {
    seeds: Vec<u64>,
    seed_to_soil_map: Vec<Range>,
    soil_to_fertilizer_map: Vec<Range>,
    fertilizer_to_water_map: Vec<Range>,
    water_to_light_map: Vec<Range>,
    light_to_temperature_map: Vec<Range>,
    temperature_to_humidity_map: Vec<Range>,
    humidity_to_location_map: Vec<Range>,
}

impl Alamanac {
    fn seed_to_location(&self, seed: u64) -> u64 {
        let soil = value_from_ranges(seed, &self.seed_to_soil_map);
        let fertilizer = value_from_ranges(soil, &self.soil_to_fertilizer_map);
        let water = value_from_ranges(fertilizer, &self.fertilizer_to_water_map);
        let light = value_from_ranges(water, &self.water_to_light_map);
        let temp = value_from_ranges(light, &self.light_to_temperature_map);
        let humidity = value_from_ranges(temp, &self.temperature_to_humidity_map);
        let location = value_from_ranges(humidity, &self.humidity_to_location_map);

        location
    }
}

fn value_from_ranges(seed: u64, ranges: &Vec<Range>) -> u64 {
    let value_in_range: Vec<u64> = ranges
        .into_iter()
        .filter_map(|range| range.value_in_range(seed))
        .collect();

    if value_in_range.len() > 0 {
        return *value_in_range.first().unwrap();
    }

    seed
}

#[derive(Debug)]
struct Range {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

impl Range {
    fn value_in_range(&self, seed: u64) -> Option<u64> {
        if seed >= self.source_range_start && seed < (self.source_range_start + self.range_length) {
            return Some(seed - self.source_range_start + self.destination_range_start);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4

",
        );
        assert_eq!(result, 35);
    }

    #[test]
    fn test_range() {
        let (_, result) = parse_range("50 98 2\n").unwrap();

        assert_eq!(result.destination_range_start, 50);
        assert_eq!(result.source_range_start, 98);
        assert_eq!(result.range_length, 2);
    }
}
