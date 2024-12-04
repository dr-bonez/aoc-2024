use std::cmp::min;

use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum XMAS {
    X,
    M,
    A,
    S,
    Other,
}
use itertools::iproduct;
use XMAS::*;
impl From<char> for XMAS {
    fn from(value: char) -> Self {
        match value {
            'x' | 'X' => X,
            'm' | 'M' => M,
            'a' | 'A' => A,
            's' | 'S' => S,
            _ => Other,
        }
    }
}

fn parse_input<A: FromIterator<B>, B: FromIterator<XMAS>>() -> Result<A, Error> {
    BufReader::new(File::open("./input/d4.txt")?)
        .lines()
        .map_ok(|s| s.chars().map(XMAS::from).collect())
        .map(|l| l.map_err(Error::from))
        .collect()
}

fn max_width(map: &Vec<Vec<XMAS>>) -> usize {
    map.iter().map(|b| b.len()).max().unwrap_or(0)
}

fn get_2d(map: &Vec<Vec<XMAS>>, (row, col): (usize, usize)) -> XMAS {
    map.get(row)
        .and_then(|row| row.get(col).copied())
        .unwrap_or(Other)
}

fn count_xmas(map: &Vec<Vec<XMAS>>, coordinates: impl Iterator<Item = (usize, usize)>) -> usize {
    coordinates
        .fold((Other, 0), |(prev, c), x| {
            let x = get_2d(map, x);
            match (prev, x) {
                (_, X) | (X, M) | (M, A) => (x, c),
                (A, S) => (Other, c + 1),
                _ => (Other, c),
            }
        })
        .1
}

fn count_xmases(
    map: &Vec<Vec<XMAS>>,
    coordinates: impl Iterator<Item = impl DoubleEndedIterator<Item = (usize, usize)> + Clone>,
) -> usize {
    coordinates
        .map(|coordinates| {
            count_xmas(map, coordinates.clone()) + count_xmas(map, coordinates.rev())
        })
        .sum()
}

fn count_xmaseses(
    map: &Vec<Vec<XMAS>>,
    coordinates: impl IntoIterator<
        Item = impl Iterator<Item = impl DoubleEndedIterator<Item = (usize, usize)> + Clone>,
    >,
) -> usize {
    coordinates
        .into_iter()
        .map(|coordinates| count_xmases(map, coordinates))
        .sum()
}

fn is_x_mas(map: &Vec<Vec<XMAS>>, (row, col): (usize, usize)) -> bool {
    fn check_ms(l: XMAS, r: XMAS) -> bool {
        (l == M && r == S) || (l == S && r == M)
    }

    if get_2d(map, (row + 1, col + 1)) == A {
        check_ms(get_2d(map, (row, col)), get_2d(map, (row + 2, col + 2)))
            && check_ms(get_2d(map, (row + 2, col)), get_2d(map, (row, col + 2)))
    } else {
        false
    }
}

fn p1() -> Result<usize, Error> {
    let map: Vec<Vec<_>> = parse_input()?;
    let max_height = map.len();
    let max_width = max_width(&map);
    Ok(count_xmaseses(
        &map,
        [
            (0..max_height)
                .map(|r| {
                    (0..max_width)
                        .map(move |c| (r, c))
                        .collect_vec()
                        .into_iter()
                })
                .boxed(), // rows
            (0..max_width)
                .map(|c| {
                    (0..max_height)
                        .map(move |r| (r, c))
                        .collect_vec()
                        .into_iter()
                })
                .boxed(), // columns
            (0..max_height)
                .map(|r| {
                    (0..(max_width - r))
                        .map(move |c| (r + c, c))
                        .collect_vec()
                        .into_iter()
                })
                .boxed(), // rows diag
            (1..max_width)
                .map(|c| {
                    (0..(max_height - c))
                        .map(move |r| (r, r + c))
                        .collect_vec()
                        .into_iter()
                })
                .boxed(), // columns diag
            (0..max_height)
                .map(|r| {
                    (0..min(r + 1, max_width))
                        .map(move |c| (r - c, c))
                        .collect_vec()
                        .into_iter()
                })
                .boxed(), // rows
            (1..max_width)
                .map(|c| {
                    (0..usize::saturating_sub(max_height, c))
                        .map(move |r| (max_height - r - 1, c + r))
                        .collect_vec()
                        .into_iter()
                })
                .boxed(), // columns
        ],
    ))
}

fn p2() -> Result<usize, Error> {
    let map: Vec<Vec<_>> = parse_input()?;
    let max_height = map.len();
    let max_width = max_width(&map);

    Ok(iproduct!(0..(max_width - 2), 0..(max_height - 2))
        .filter(|(row, col)| is_x_mas(&map, (*row, *col)))
        .count())
}

#[test]
fn test_p1() {
    handle_res(p1());
}

#[test]
fn test_p2() {
    handle_res(p2());
}
