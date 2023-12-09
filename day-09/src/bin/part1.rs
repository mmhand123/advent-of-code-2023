use nom::{
    bytes::complete::tag,
    character::complete::{self, alphanumeric1, newline, space1},
    multi::{count, separated_list1},
    sequence::{separated_pair, terminated, tuple},
    IResult,
};

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> i32 {
    let summed_predictions: i32 = input
        .lines()
        .map(|line| {
            let (_, history) = parse_line(line).unwrap();

            let mut last_values: Vec<i32> = Vec::from([*history.last().unwrap()]);
            let mut differences: Vec<i32> = history.clone();

            while differences.len() > 0 && !differences.iter().all(|diff| diff == &0) {
                differences = difference(differences);

                if differences.len() > 0 {
                    last_values.push(*differences.last().unwrap());
                }
            }

            last_values.iter().sum::<i32>()
        })
        .sum();

    summed_predictions
}

fn difference(vec: Vec<i32>) -> Vec<i32> {
    vec.iter()
        .zip(vec.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect()
}

fn parse_line(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(space1, complete::i32)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
        );
        assert_eq!(result, 114);
    }
}
