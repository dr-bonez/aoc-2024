use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{anyhow, Error};
use itertools::Itertools;

use crate::{collect_arr, handle_res, MoreIterTools};

fn calculate_distance(mut a: Vec<usize>, mut b: Vec<usize>) -> usize {
    a.sort();
    b.sort();
    a.into_iter()
        .zip_eq(b)
        .map(|(a, b)| if a > b { a - b } else { b - a })
        .sum()
}

fn calculate_similarity(mut a: Vec<usize>, mut b: VecDeque<usize>) -> usize {
    a.sort();
    b.make_contiguous().sort();
    a.into_iter()
        .map(|a| {
            let mut res = 0;
            loop {
                match b.get(0) {
                    Some(x_b) if *x_b < a => {
                        b.pop_front();
                    }
                    Some(x_b) if *x_b == a => {
                        b.pop_front();
                        res += 1;
                    }
                    _ => return res * a,
                }
            }
        })
        .sum()
}

fn parse_input<A: Default + Extend<usize>, B: Default + Extend<usize>>() -> Result<(A, B), Error> {
    BufReader::new(File::open("./input/d1.txt")?)
        .lines()
        .filter_map_ok(|s| {
            let s = s.trim();
            if s.is_empty() {
                None
            } else {
                Some((|s: &str| {
                    let [a, b] = collect_arr::<2, &str>(s.split_ascii_whitespace())?;
                    Ok((a.parse()?, b.parse()?))
                })(s))
            }
        })
        .map(|s| s.map_err(Error::from).and_then(|s| s))
        .try_unzip::<_, _, A, B>()
}

fn p1() -> Result<usize, Error> {
    let (a, b) = parse_input()?;

    Ok(calculate_distance(a, b))
}

fn p2() -> Result<usize, Error> {
    let (a, b) = parse_input()?;

    Ok(calculate_similarity(a, b))
}

#[test]
fn test_p1() {
    handle_res(p1());
}

#[test]
fn test_p2() {
    handle_res(p2());
}
