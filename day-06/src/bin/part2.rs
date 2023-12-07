use nom::{
    bytes::complete::tag,
    character::complete::{self, digit1, multispace1, newline},
    multi::separated_list1,
    sequence::{preceded, terminated, tuple},
    IResult,
};

fn main() {
    let input = include_str!("./input2.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> usize {
    let (_, race) = parse(input).unwrap();

    let margin = score_race(race);

    margin
}

fn parse(input: &str) -> IResult<&str, (u64, u64)> {
    let (remaining, times) = preceded(tuple((tag("Time:"), multispace1)), parse_numbers)(input)?;
    let (remaining, distances) =
        preceded(tuple((tag("Distance:"), multispace1)), parse_numbers)(remaining)?;

    let time = times.join("").parse::<u64>().unwrap();
    let distance = distances.join("").parse::<u64>().unwrap();

    Ok((remaining, (time, distance)))
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<&str>> {
    terminated(separated_list1(multispace1, digit1), newline)(input)
}

fn score_race(race: (u64, u64)) -> usize {
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
        assert_eq!(result, 71503);
    }
}
