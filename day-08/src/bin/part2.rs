use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
};

use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

use nom::{
    bytes::complete::tag,
    character::complete::{self, alphanumeric1, newline, space1},
    multi::count,
    sequence::{separated_pair, terminated, tuple},
    IResult,
};

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> usize {
    let (remaining, navigation_instructions) = parse_navigation_instructions(input).unwrap();

    let maps: HashMap<&str, (&str, &str)> = remaining
        .lines()
        .map(|line| {
            let (_, (key, (left, right))) = parse_map(line).unwrap();

            (key, (left, right))
        })
        .fold(HashMap::new(), |mut acc, (key, map)| {
            acc.insert(key, map);

            acc
        });

    let starting_nodes: Vec<&str> = maps
        .iter()
        .filter_map(|(k, _)| {
            if k.ends_with("A") {
                return Some(k.clone());
            }

            None
        })
        .collect();

    let chars: Vec<char> = navigation_instructions.chars().collect();
    let all_steps: Vec<usize> = starting_nodes
        .into_par_iter()
        .map(|starting_node| {
            let mut num_steps = 0;
            let mut current_index = 0;
            let mut current_node = starting_node;

            while !current_node.ends_with("Z") {
                num_steps += 1;
                let next_step = chars.get(current_index).unwrap();
                let (left, right) = maps.get(current_node).unwrap();

                match next_step {
                    'L' => {
                        current_node = left;
                    }
                    'R' => {
                        current_node = right;
                    }
                    _ => panic!("this should never happen"),
                }

                if current_index == navigation_instructions.len() - 1 {
                    current_index = 0;
                } else {
                    current_index += 1;
                }
            }

            num_steps
        })
        .collect();

    all_steps
        .into_iter()
        .reduce(|acc, step| lcm(acc, step))
        .unwrap()
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    return gcd(b, a % b);
}

fn lcm(a: usize, b: usize) -> usize {
    return (a * b) / gcd(a, b);
}

fn parse_navigation_instructions(input: &str) -> IResult<&str, &str> {
    terminated(alphanumeric1, count(newline, 2))(input)
}

fn parse_map(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    let (remaining, (key, _, _, _)) = tuple((alphanumeric1, space1, tag("="), space1))(input)?;
    let (remaining, (_, left, _, _, right, _)) = tuple((
        tag("("),
        alphanumeric1,
        tag(","),
        space1,
        alphanumeric1,
        tag(")"),
    ))(remaining)?;

    Ok((remaining, (key, (left, right))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        );
        assert_eq!(result, 6);
    }
}
