use std::error::Error;
use std::io::prelude::*;
use std::time::Instant;

use nom::{
    branch::alt,
    character::complete::{char, newline, space1, usize},
    combinator::{map, value},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};

use itertools::{iproduct, Itertools};

use microlp::{ComparisonOp, OptimizationDirection, Problem, Solution};
use ndarray::Array;

use rayon::prelude::*;

#[inline]
fn cell(input: &str) -> IResult<&str, bool> {
    alt((value(false, char('.')), value(true, char('#')))).parse(input)
}

#[inline]
fn row(input: &str) -> IResult<&str, [bool; 3]> {
    map((cell, cell, cell), |(a, b, c)| [a, b, c]).parse(input)
}
fn parse_present(input: &str) -> IResult<&str, Present> {
    preceded(
        (usize, char(':'), newline),
        map(
            (row, newline, row, newline, row, newline),
            |(a, _, b, _, c, _)| Present([a, b, c]),
        ),
    )
    .parse(input)
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct PackingProblem {
    width: usize,
    height: usize,
    constraints: Vec<usize>,
}
fn parse_problem(input: &str) -> IResult<&str, PackingProblem> {
    map(
        separated_pair(
            separated_pair(usize, char('x'), usize),
            (char(':'), space1),
            separated_list1(space1, usize),
        ),
        |((width, height), constraints)| PackingProblem {
            width,
            height,
            constraints,
        },
    )
    .parse(input)
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Present>, Vec<PackingProblem>)> {
    separated_pair(
        separated_list1(newline, parse_present),
        newline,
        separated_list1(newline, parse_problem),
    )
    .parse(input)
}

// Each present is a 3x3 boolean matrix where true means that cell is covered by the present
// Since rotating and flipping is allowed, we need some functions for that
#[derive(Hash, Copy, Clone, Eq, PartialEq)]
struct Present([[bool; 3]; 3]);

// reflect a present around the vertical axis
#[inline]
fn reflect(Present(p): &Present) -> Present {
    Present([
        [p[0][2], p[0][1], p[0][0]],
        [p[1][2], p[1][1], p[1][0]],
        [p[2][2], p[2][1], p[2][0]],
    ])
}

// rotate a present clockwise
#[inline]
fn rotate(Present(p): &Present) -> Present {
    Present([
        [p[0][2], p[1][2], p[2][2]],
        [p[0][1], p[1][1], p[2][1]],
        [p[0][0], p[1][0], p[2][0]],
    ])
}

// return the image of a present under the dihedral group
#[inline]
fn all_rotations(p: &Present) -> Vec<Present> {
    (0..=3)
        .scan(*p, |prev, _| {
            *prev = rotate(prev);
            Some(*prev)
        })
        .collect()
}

// return the non-equivalent rotations and reflections of a present
fn rotate_and_reflect(p: &Present) -> Vec<Present> {
    [p, &reflect(p)]
        .into_iter()
        .map(all_rotations)
        .flatten()
        .unique()
        .collect()
}

fn pack(
    presents: &Vec<Present>,
    PackingProblem {
        height,
        width,
        constraints,
    }: &PackingProblem,
) -> Result<Solution, microlp::Error> {
    let mut prob = Problem::new(OptimizationDirection::Minimize);
    let xmax = width.saturating_sub(2);
    let ymax = height.saturating_sub(2);

    // The variables in the problem are, for every type of present and every cell, a decision variable whether to place a present of that type with its top left corner there.
    // In an N×M grid this is only possible if (x, y) < (N-2, M-2) [zero-indexed].
    // First we rotate and reflect all the presents and count the number of types
    let dihedral: Vec<_> = presents.iter().map(rotate_and_reflect).collect();
    let n_presents = dihedral.iter().map(Vec::len).sum();
    // Now we can create the variables
    let placed = Array::from_shape_vec(
        (n_presents, xmax, ymax),
        iproduct![0..n_presents, 0..xmax, 0..ymax]
            .map(|_| prob.add_binary_var(0.0))
            .collect(),
    )
    // This unwrap is safe because we control the shape of the array
    .unwrap();

    // The simplest constraints are the usage constraints
    //     \sum_{j ~ k} placed[j] = presents[k]
    // where presents[k] is the number of presents of each type to be placed, and
    //     j ~ k
    // means that j is a rotation and or reflection of k.

    let mut idx: usize = 0;
    for (n, class) in dihedral.iter().enumerate() {
        let lhs = iproduct![idx..(idx + class.len()), 0..(*width - 2), 0..(*height - 2)]
            .map(|vidx| (placed[vidx], 1.0));
        prob.add_constraint(lhs, ComparisonOp::Eq, constraints[n] as f64);
        idx += class.len();
    }
    let n_present_constraints = constraints.len();

    // The number of presents covering a cell is
    //     covered[x][y] = \sum_k (shape[k] * placed[k]) <= 1
    // where * is convolution (standard interpretation of convolution as sliding a window over an image).
    let presents: Vec<_> = dihedral.into_iter().flatten().collect();
    for (x, y) in iproduct![0..*width, 0..*height] {
        let lhs = iproduct![0..n_presents, 0..3, 0..3]
            .filter_map(|(k, u, v)| {
                if u <= x && v <= y && x - u < xmax && y - v < ymax {
                    Some((placed[(k, x - u, y - v)], presents[k].0[u][v] as u8 as f64))
                } else {
                    None
                }
            })
            .collect_vec();
        prob.add_constraint(lhs, ComparisonOp::Le, 1.0);
    }
    let n_packing_constraints = height * width;

    // Print some diagnostics
    println!(
        "Solving {width}×{height} problem with {n_presents} presents ({} dihedral orbits): {} vars, {} + {} = {} constraints",
        presents.len(),
        n_presents * xmax * ymax,
        n_present_constraints,
        n_packing_constraints,
        n_packing_constraints + n_present_constraints
);
    let now = Instant::now();
    let out = prob.solve();
    println!(
        "Solved a {width}×{height} problem in {:6.2} s: {}",
        now.elapsed().as_secs_f32(),
        if out.is_ok() {
            "feasible"
        } else {
            "infeasible"
        }
    );
    out
}

fn n_occupied(Present(p): &Present) -> usize {
    p.iter().flatten().map(|&b| b as usize).sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();
    let mut input = String::new();

    stdin.read_to_string(&mut input)?;
    let (_, (presents, problems)) =
        parse_input(&input).map_err(|e| format!("Invalid input: {e}"))?;

    // Prune problems where there is not sufficient area even with perfect packing
    let prune_infeasible = false;
    // short-circuit when the problem is trivially solvable by putting each present in its
    // own 3×3 box
    let short_circuit_trivial = false;
    let answer = problems
        .into_par_iter()
        .filter(|p| {
            !prune_infeasible
                || p.width * p.height
                    >= Iterator::zip(p.constraints.iter(), presents.iter().map(n_occupied))
                        .map(|(c, n)| c * n)
                        .sum()
        })
        .filter(|p| {
            (short_circuit_trivial && (p.width / 3) * (p.height / 3) >= p.constraints.iter().sum())
                || pack(&presents, &p).is_ok()
        })
        .count();

    println!("Number of feasible problems: {answer}");

    return Ok(());
}
