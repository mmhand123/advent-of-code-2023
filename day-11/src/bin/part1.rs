use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alphanumeric1, newline, space1},
    combinator::map_res,
    multi::{count, many1, separated_list1},
    sequence::{separated_pair, terminated, tuple},
    IResult,
};

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> i32 {
    let (_, mut image) = parse_lines(input).unwrap();

    dbg!(&image);

    let mut empty_rows: Vec<usize> = Vec::new();
    let mut empty_cols: Vec<usize> = Vec::new();
    image.iter().enumerate().for_each(|(row_index, row)| {
        let is_row_empty = row.iter().all(|col| match col {
            DataType::EmptySpace => true,
            DataType::Galaxy => false,
        });

        if is_row_empty {
            empty_rows.push(row_index);
        }

        if row_index == 0 {
            row.iter().enumerate().for_each(|(col_index, col)| {
                if matches!(col, DataType::EmptySpace) {
                    // check cols
                    let empty_col = (1..image.len()).all(|row_i| {
                        let other_col = image.get(row_i).unwrap().get(col_index).unwrap();
                        dbg!(row_i);
                        dbg!(other_col);
                        dbg!(col_index);

                        matches!(other_col, DataType::EmptySpace)
                    });

                    if empty_col {
                        empty_cols.push(col_index);
                    }
                }
            });
        }
    });

    let mut distances: Vec<i32> = Vec::new();
    let galaxies: Vec<(usize, usize)> = image
        .iter()
        .enumerate()
        .flat_map(|(row_index, row)| {
            let row_offset: Vec<&usize> =
                empty_rows.iter().filter(|&&row| row < row_index).collect();

            // borrow checker is defeating me, this is bad
            let cols = empty_cols.clone();

            row.iter().enumerate().filter_map(move |(col_index, col)| {
                let col_offset: Vec<&usize> = cols.iter().filter(|&&col| col < col_index).collect();
                match col {
                    DataType::Galaxy => {
                        Some((row_index + row_offset.len(), col_index + col_offset.len()))
                    }
                    _ => None,
                }
            })
        })
        .collect();

    for (i, (galaxy_x, galaxy_y)) in galaxies.iter().enumerate() {
        (i + 1..galaxies.len()).into_iter().for_each(|other_i| {
            let (other_galaxy_x, other_galaxy_y) = galaxies.get(other_i).unwrap();
            let diff_x = *other_galaxy_x as i32 - *galaxy_x as i32;
            let diff_y = *other_galaxy_y as i32 - *galaxy_y as i32;

            distances.push(diff_x.abs() + diff_y.abs())
        })
    }

    dbg!(&image);
    dbg!(&galaxies);

    dbg!(&empty_rows);
    dbg!(&empty_cols);

    // test();

    distances.iter().sum::<i32>()
}

fn parse_lines(input: &str) -> IResult<&str, Vec<Vec<DataType>>> {
    many1(terminated(parse_line, newline))(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<DataType>> {
    many1(map_res(alt((tag("."), tag("#"))), to_data_type))(input)
}

fn to_data_type(input: &str) -> Result<DataType, ParseError> {
    match input {
        "#" => Ok(DataType::Galaxy),
        "." => Ok(DataType::EmptySpace),
        _ => Err(ParseError::InvalidInput),
    }
}

#[derive(Debug)]
enum ParseError {
    InvalidInput,
    // Add other error types here if needed
}

#[derive(Debug)]
enum DataType {
    EmptySpace,
    Galaxy,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
",
        );
        assert_eq!(result, 374);
    }
}
