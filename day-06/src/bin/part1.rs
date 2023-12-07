use nom::{
    bytes::complete::tag,
    character::complete::{self, multispace1, newline},
    multi::separated_list1,
    sequence::{preceded, terminated, tuple},
    IResult,
};

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> usize {
    let (_, races) = parse(input).unwrap();

    let margin = races.into_iter().map(|race| score_race(race)).product();

    margin
}

fn parse(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    let (remaining, times) = preceded(tuple((tag("Time:"), multispace1)), parse_numbers)(input)?;
    let (remaining, distances) =
        preceded(tuple((tag("Distance:"), multispace1)), parse_numbers)(remaining)?;

    Ok((
        remaining,
        times
            .iter()
            .enumerate()
            .map(|(i, time)| (*time, *distances.get(i).unwrap()))
            .collect(),
    ))
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    terminated(separated_list1(multispace1, complete::u32), newline)(input)
}

fn score_race(race: (u32, u32)) -> usize {
    let (race_time, distance) = race;

    (1..race_time)
        .filter_map(|speed| {
            let time = race_time - speed;

            let distance_traveled = time * speed;

            if distance_traveled > distance {
                return Some(distance_traveled);
            }

            None
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "Time:      7  15   30
Distance:  9  40  200
",
        );
        assert_eq!(result, 288);
    }
}
