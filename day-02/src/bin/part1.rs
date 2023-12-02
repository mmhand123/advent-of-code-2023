use nom::{
    character::complete::{alpha1, char, digit1, multispace0, space1},
    combinator::map_res,
    multi::separated_list0,
    sequence::{separated_pair, tuple},
    IResult,
};

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> usize {
    let valid_games: Vec<Game> = input
        .lines()
        .map(|line| {
            let (rest_of_line, game_id) = parse_game_id(line);
            let raw_rounds = rest_of_line.trim().split(';');

            let rounds: Vec<Round> = raw_rounds
                .map(|raw_round| {
                    let round = parse_round(raw_round);

                    return round;
                })
                .collect();

            return Game {
                id: game_id,
                rounds,
            };
        })
        .filter(|game| game.rounds.iter().all(|round| is_valid_round(round)))
        .collect();

    return valid_games.iter().map(|game| game.id).sum();
}

fn parse_game_metadata(input: &str) -> IResult<&str, (&str, &str, &str, char)> {
    return tuple((alpha1, space1, digit1, char(':')))(input);
}

#[derive(Debug)]
struct Game {
    id: usize,
    rounds: Vec<Round>,
}

#[derive(Debug)]
struct Round {
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Debug)]
struct ColorCount {
    number: usize,
    color: String,
}

fn parse_game_id(input: &str) -> (&str, usize) {
    let (rest, (_, _, game_number_str, _)) = parse_game_metadata(input).unwrap();
    let game_number: usize = game_number_str.parse().unwrap();

    (rest, game_number)
}

fn parse_round(input: &str) -> Round {
    let (_, rolls) = parse_rolls(input.trim()).unwrap();
    let mut round = Round {
        red: 0,
        green: 0,
        blue: 0,
    };

    for roll in rolls {
        match roll.color.as_str() {
            "red" => round.red = roll.number,
            "green" => round.green = roll.number,
            "blue" => round.blue = roll.number,
            _ => {}
        }
    }

    return round;
}

fn parse_color_count(input: &str) -> IResult<&str, ColorCount> {
    let (input, (number, color)) =
        separated_pair(map_res(digit1, str::parse::<usize>), space1, alpha1)(input)?;

    Ok((
        input,
        ColorCount {
            number,
            color: color.to_string(),
        },
    ))
}

fn parse_rolls(input: &str) -> IResult<&str, Vec<ColorCount>> {
    separated_list0(tuple((char(','), multispace0)), parse_color_count)(input)
}

fn is_valid_round(round: &Round) -> bool {
    round.red <= 12 && round.green <= 13 && round.blue <= 14
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, 8);
    }
}
