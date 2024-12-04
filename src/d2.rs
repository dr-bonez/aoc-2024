use crate::prelude::*;

fn parse_input<A: FromIterator<B>, B: FromIterator<usize>>() -> Result<A, Error> {
    BufReader::new(File::open("./input/d2.txt")?)
        .lines()
        .filter_map_ok(|l| {
            let s = l.trim();
            if s.is_empty() {
                None
            } else {
                Some(
                    s.split_ascii_whitespace()
                        .map(|s| s.parse())
                        .collect::<Result<_, _>>(),
                )
            }
        })
        .map(|s| s.map_err(Error::from).and_then(|s| s.map_err(Error::from)))
        .collect()
}

fn safe_step(
    prev: Option<usize>,
    x: usize,
    state: Option<bool>,
    is_safe: bool,
) -> (Option<usize>, Option<bool>, bool) {
    let new_state = prev.map(|prev| prev < x);
    let new_safe = state.map_or(true, |s| Some(s) == new_state)
        && prev.map_or(true, |prev| {
            prev != x
                && match new_state {
                    Some(true) => prev < x && x - prev <= 3,
                    Some(false) => prev > x && prev - x <= 3,
                    None => true,
                }
        });
    (Some(x), new_state, is_safe && new_safe)
}

fn is_safe(a: impl IntoIterator<Item = usize>) -> bool {
    let mut prev = None;
    let mut state = None;
    let mut is_safe = true;
    for x in a {
        (prev, state, is_safe) = safe_step(prev, x, state, is_safe);
        if !is_safe {
            return false;
        }
    }
    is_safe
}

fn is_damp_safe(a: &Vec<usize>, skip: Option<usize>) -> bool {
    let mut prev = None;
    let mut state = None;
    let mut is_safe = true;
    for (idx, x) in a.iter().copied().enumerate() {
        if Some(idx) == skip {
            continue;
        }
        (prev, state, is_safe) = safe_step(prev, x, state, is_safe);
        if !is_safe {
            if skip.is_none() {
                for i in usize::saturating_sub(idx, 2)..=idx {
                    if is_damp_safe(a, Some(i)) {
                        return true;
                    }
                }
            }
            return false;
        }
    }
    is_safe
}

fn p1() -> Result<usize, Error> {
    let a: Vec<Vec<usize>> = parse_input()?;

    Ok(a.into_iter().filter(|a| is_safe(a.iter().copied())).count())
}

fn p2() -> Result<usize, Error> {
    let a: Vec<Vec<usize>> = parse_input()?;

    Ok(a.into_iter().filter(|a| is_damp_safe(a, None)).count())
}

#[test]
fn test_p1() {
    handle_res(p1());
}

#[test]
fn test_p2() {
    handle_res(p2());
}
