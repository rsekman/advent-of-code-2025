use std::error::Error;
use std::io::prelude::*;

use aoclib::grid::{diagonal_neighbors_within_bounds, UPoint};

use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::{char, newline},
    combinator::value,
    multi::{many1, separated_list1},
    IResult, Parser,
};

fn parse_code(input: &str) -> IResult<&str, Vec<Vec<bool>>> {
    separated_list1(
        newline,
        many1(alt((value(true, char('@')), value(false, char('.'))))),
    )
    .parse(input)
}

fn accessible<'a>(grid: &'a Vec<Vec<bool>>) -> Vec<(usize, usize)> {
    let height = grid.len();
    let width = grid[0].len();

    (0..height)
        .cartesian_product(0..width)
        .filter(|&(x, y)| grid[x][y])
        .filter(|&(x, y)| {
            diagonal_neighbors_within_bounds(&UPoint { x, y }, (height - 1, width - 1))
                .into_iter()
                .filter(|v| grid[v.x][v.y])
                .count()
                < 4
        })
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();
    let mut input = String::new();

    stdin.read_to_string(&mut input)?;
    let (_, mut grid) = parse_code(&input).map_err(|e| format!("Invalid input: {e}"))?;

    let mut n_accessible = 0;
    for n in 0.. {
        let idxs = accessible(&grid);
        if idxs.len() == 0 {
            break;
        }
        n_accessible += idxs.len();
        let _ = idxs.iter().map(|&(x, y)| grid[x][y] = false).collect_vec();
        println!("The number of accessible paper rolls (stage {n}) is {n_accessible}");
    }

    return Ok(());
}
