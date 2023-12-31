use nom::{
    branch::alt, bytes::complete::take_while1, character::complete::digit1, multi::many0, IResult,
};
use nom_locate::LocatedSpan;
type Span<'a> = LocatedSpan<&'a str>;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> usize {
    let matrix: Vec<EngineSchematic> = input
        .lines()
        .map(|line| {
            let span = Span::from(line);
            let (_, tokens) = parse_number_or_period_or_symbol(span).unwrap();
            let mut numbers: Vec<PartNumber> = Vec::new();
            let mut symbols: Vec<Symbol> = Vec::new();

            tokens.into_iter().for_each(|token| {
                let num_parse_result = token.value.parse();

                match num_parse_result {
                    Ok(num) => numbers.push(PartNumber {
                        value: num,
                        start: token.start_pos,
                        end: token.end_pos,
                    }),
                    Err(_) => {
                        if token.value.contains('.') {
                            return;
                        }

                        symbols.push(Symbol {
                            value: token.value.to_string(),
                            start: token.start_pos,
                            end: token.end_pos,
                        })
                    }
                }
            });

            EngineSchematic { numbers, symbols }
        })
        .collect();

    let sum: usize = matrix
        .iter()
        .enumerate()
        .filter_map(|(row_index, row)| {
            let row_value: usize = row
                .numbers
                .iter()
                .filter_map(|number| {
                    let is_adjacent_to_symbols_on_row =
                        is_adjacent_to_symbols(&number, &row.symbols);
                    let previous_row = if row_index > 0 {
                        matrix.get(row_index - 1)
                    } else {
                        None
                    };
                    let next_row = matrix.get(row_index + 1);

                    if is_adjacent_to_symbols_on_row {
                        return Some(number.value);
                    }

                    if let Some(previous_row) = previous_row {
                        if is_adjacent_to_symbols(&number, &previous_row.symbols) {
                            return Some(number.value);
                        }
                    }

                    if let Some(next_row) = next_row {
                        if is_adjacent_to_symbols(&number, &next_row.symbols) {
                            return Some(number.value);
                        }
                    }

                    None
                })
                .sum();

            Some(row_value)
        })
        .sum();

    dbg!(sum);

    sum
}

fn parse_number_or_period_or_symbol<'a>(input: Span<'a>) -> IResult<Span<'a>, Vec<Token<'a>>> {
    many0(alt((parse_number, parse_symbol, parse_period)))(input)
}

fn parse_number<'a>(input: Span<'a>) -> IResult<Span<'a>, Token> {
    let start_offset = input.location_offset();
    let (remaining, number) = digit1(input)?;
    let end_offset = remaining.location_offset();

    Ok((
        remaining,
        Token {
            value: number.fragment(),
            start_pos: start_offset,
            end_pos: end_offset,
        },
    ))
}

fn parse_symbol<'a>(input: Span<'a>) -> IResult<Span<'a>, Token> {
    let start_offset = input.location_offset();
    let (remaining, symbol) = take_while1(|c: char| !c.is_alphanumeric() && c != '.')(input)?;
    let end_offset = remaining.location_offset();

    Ok((
        remaining,
        Token {
            value: symbol.fragment(),
            start_pos: start_offset,
            end_pos: end_offset,
        },
    ))
}

fn parse_period<'a>(input: Span<'a>) -> IResult<Span<'a>, Token> {
    let start_offset = input.location_offset();
    let (remaining, periods) = take_while1(|c: char| c == '.')(input)?;
    let end_offset = remaining.location_offset();

    Ok((
        remaining,
        Token {
            value: periods.fragment(),
            start_pos: start_offset,
            end_pos: end_offset,
        },
    ))
}

fn is_adjacent(num_start: usize, num_end: usize, symbol_start: usize, symbol_end: usize) -> bool {
    // between number
    if symbol_start >= num_start && symbol_end <= num_end {
        return true;
    }

    // before number
    if num_start != 0 && symbol_start == num_start - 1 {
        return true;
    }

    // after number
    if symbol_end == num_end + 1 {
        return true;
    }

    false
}

fn is_adjacent_to_symbols(number: &PartNumber, symbols: &Vec<Symbol>) -> bool {
    symbols
        .iter()
        .any(|symbol| is_adjacent(number.start, number.end, symbol.start, symbol.end))
}

#[derive(Debug)]
struct Token<'a> {
    value: &'a str,
    start_pos: usize,
    end_pos: usize,
}

#[derive(Debug)]
struct EngineSchematic {
    numbers: Vec<PartNumber>,
    symbols: Vec<Symbol>,
}

#[derive(Debug)]
struct PartNumber {
    value: usize,
    start: usize,
    end: usize,
}

#[derive(Debug)]
struct Symbol {
    start: usize,
    end: usize,
    value: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(result, 4361);
    }
}
