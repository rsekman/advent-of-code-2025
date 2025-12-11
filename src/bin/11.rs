use std::error::Error;
use std::io::prelude::*;

use std::collections::BTreeMap;

use nom::{
    character::complete::{alpha1, char, newline, space1},
    combinator::map,
    multi::separated_list1,
    IResult, Parser,
};

type Tree<'a> = BTreeMap<&'a str, Vec<&'a str>>;

fn parse_input<'a>(input: &'a str) -> IResult<&'a str, Tree<'a>> {
    map(
        separated_list1(
            newline,
            map(
                (alpha1, char(':'), space1, separated_list1(space1, alpha1)),
                |(name, _, _, nodes)| (name, nodes),
            ),
        ),
        |v| v.into_iter().collect(),
    )
    .parse(input)
}

fn transpose_tree<'a>(tree: &Tree<'a>) -> Tree<'a> {
    let mut out = Tree::new();
    for (name, outgoing) in tree.iter() {
        for o in outgoing {
            out.entry(o).or_default().push(name);
        }
    }
    out
}

fn dfs<'a>(start: &'a str, tree: &Tree<'a>, paths: &mut BTreeMap<&'a str, usize>) -> usize {
    match paths.get(start) {
        Some(n) => *n,
        None => {
            let p = tree
                .get(start)
                .map(|v| v.iter().map(|node| dfs(node, tree, paths)).sum())
                .unwrap_or(0);
            paths.insert(start, p);
            p
        }
    }
}

fn paths_between<'a>(start: &'a str, end: &'a str, tree: &Tree<'a>) -> usize {
    let mut paths: BTreeMap<&str, usize> = BTreeMap::new();
    paths.insert(start, 1);
    dfs(end, tree, &mut paths)
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();

    let mut input = String::new();
    stdin.read_to_string(&mut input)?;
    let (_, tree) = parse_input(&input).map_err(|e| format!("Invalid input: {e}"))?;

    let transposed = transpose_tree(&tree);
    let answer = paths_between(&"you", &"out", &transposed);

    println!("# paths from you to out: {answer}");

    let svr_to_fft = paths_between(&"svr", &"fft", &transposed);
    println!("svr to fft: {svr_to_fft}");
    let svr_to_dac = paths_between(&"svr", &"dac", &transposed);
    println!("svr to dac: {svr_to_dac}");

    let dac_to_fft = paths_between(&"dac", &"fft", &transposed);
    println!("dac to fft: {dac_to_fft}");
    let dac_to_out = paths_between(&"dac", &"out", &transposed);
    println!("dac to out: {dac_to_out}");

    let fft_to_dac = paths_between(&"fft", &"dac", &transposed);
    println!("fft to dac: {fft_to_dac}");
    let fft_to_out = paths_between(&"fft", &"out", &transposed);
    println!("fft to out: {fft_to_out}");

    let answer = svr_to_fft * fft_to_dac * dac_to_out + svr_to_dac * dac_to_fft * fft_to_out;

    println!("# paths from svr to out visiting both dac and fft: {answer}");

    return Ok(());
}
