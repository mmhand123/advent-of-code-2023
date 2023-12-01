use aho_corasick::{AhoCorasick, Match, MatchKind, PatternID};

fn main() {
    let input = include_str!("./input2.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut sum: usize = 0;
    for line in input.lines() {
        dbg!(line);
        let converted = convert_to_numbers(line);
        dbg!(&converted);
        let line_number = parse_number(&converted);
        dbg!(&line_number);

        sum += line_number;
    }

    return sum.to_string();
}

fn convert_to_numbers(input: &str) -> String {
    let mut converted = String::from(input);
    let patterns = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let replacements = &["1e", "2o", "3e", "4", "5e", "6", "7", "8t", "9e"];
    let ac = AhoCorasick::new(patterns).unwrap();
    let matches: Vec<Match> = ac.find_overlapping_iter(input).collect();

    matches.iter().for_each(|_m| {
        converted = ac.replace_all(&converted, replacements);
    });

    return converted;
}

fn parse_number(line: &str) -> usize {
    let line_string_nums: Vec<char> = line
        .chars()
        .filter(|char| {
            return match char.to_digit(10) {
                Some(_digit) => true,
                None => false,
            };
        })
        .collect();

    dbg!(&line_string_nums);

    let mut line_string_num = String::from(line_string_nums[0]);

    line_string_num.push(line_string_nums[line_string_nums.len() - 1]);

    return line_string_num.parse().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        assert_eq!(result, "281".to_string());
    }

    #[test]
    fn hmm() {
        let result = parse_number(&convert_to_numbers("zoneight234"));
        let result2 = parse_number(&convert_to_numbers("4nineeightseven2"));
        assert_eq!(result, 14);
        assert_eq!(result2, 42);
    }
}
