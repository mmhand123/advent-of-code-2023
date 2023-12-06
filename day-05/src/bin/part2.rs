use rayon::prelude::*;
use std::{collections::BTreeMap, ops::Range};

use indicatif::ParallelProgressIterator;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use nom::{
    character::complete::{self, multispace0, newline, space0, space1},
    multi::{count, many1, separated_list1},
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult, Parser,
};
use nom_supreme::{tag::complete::tag, ParserExt};

fn main() {
    let input = include_str!("./input2.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> u64 {
    let (_, almanac) = parse(input).unwrap();
    let locations = almanac
        .seeds
        .par_iter()
        .progress_count(almanac.seeds.len() as u64)
        .flat_map(|range| range.clone())
        .map(|seed| almanac.seed_to_location(seed))
        .min()
        .unwrap();

    locations
}

fn parse(input: &str) -> IResult<&str, Almanac> {
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
        Almanac {
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

fn parse_seeds(input: &str) -> IResult<&str, Vec<Range<u64>>> {
    let seed_parser = tag("seeds: ").precedes(separated_list1(
        space1,
        separated_pair(complete::u64, tag(" "), complete::u64)
            .map(|(start, offset)| start..(start + offset)),
    ));
    terminated(seed_parser, count(newline, 2))(input)
}

fn parse_map<'a>(
    input: &'a str,
    separator: &str,
) -> IResult<&'a str, Vec<(Range<u64>, Range<u64>)>> {
    let (remaining, _) = terminated(tag(separator), newline)(input)?;
    let (remaining, ranges) =
        terminated(separated_list1(newline, parse_range), count(newline, 2))(remaining)?;

    Ok((remaining, ranges))
}

fn parse_range(input: &str) -> IResult<&str, (Range<u64>, Range<u64>)> {
    let (remaining, (destination_range_start, _, source_range_start, _, range_length)) =
        tuple((complete::u64, space1, complete::u64, space1, complete::u64))(input)?;

    Ok((
        remaining,
        (
            source_range_start..(source_range_start + range_length),
            destination_range_start..destination_range_start + range_length,
        ),
    ))
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<Range<u64>>,
    seed_to_soil_map: Vec<(Range<u64>, Range<u64>)>,
    soil_to_fertilizer_map: Vec<(Range<u64>, Range<u64>)>,
    fertilizer_to_water_map: Vec<(Range<u64>, Range<u64>)>,
    water_to_light_map: Vec<(Range<u64>, Range<u64>)>,
    light_to_temperature_map: Vec<(Range<u64>, Range<u64>)>,
    temperature_to_humidity_map: Vec<(Range<u64>, Range<u64>)>,
    humidity_to_location_map: Vec<(Range<u64>, Range<u64>)>,
}

impl Almanac {
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

fn value_from_ranges(seed: u64, ranges: &Vec<(Range<u64>, Range<u64>)>) -> u64 {
    let valid_mapping = ranges
        .iter()
        .find(|(source_range, _)| source_range.contains(&seed));

    let Some((source_range, destination_range)) =
            valid_mapping
        else {
            return seed;
        };

    let offset = seed - source_range.start;

    destination_range.start + offset
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
        assert_eq!(result, 46);
    }
}
