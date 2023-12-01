fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut sum: usize = 0;
    for line in input.lines() {
        let line_number = parse_number(line);

        sum += line_number;
    }

    dbg!(sum);

    return sum.to_string();
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
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        assert_eq!(result, "142".to_string());
    }
}
