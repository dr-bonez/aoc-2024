use regex::Regex;

use crate::prelude::*;

fn parse_input() -> Result<String, Error> {
    Ok(std::fs::read_to_string("./input/d3.txt")?)
}

fn p1() -> Result<usize, Error> {
    let s = parse_input()?;
    let re = Regex::new("mul\\(([0-9]{1,3}),([0-9]{1,3})\\)")?;
    re.captures_iter(&s)
        .map(|m| {
            Ok::<_, Error>(
                m.get(1).unwrap().as_str().parse::<usize>()?
                    * m.get(2).unwrap().as_str().parse::<usize>()?,
            )
        })
        .sum_ok()
}

fn p2() -> Result<usize, Error> {
    let s = parse_input()?;
    let re = Regex::new("(mul)\\(([0-9]{1,3}),([0-9]{1,3})\\)|(do)\\(\\)|(don't)\\(\\)")?;
    let mut enabled = 1;
    re.captures_iter(&s)
        .map(|m| {
            Ok::<_, Error>(if m.get(1).is_some() {
                enabled
                    * m.get(2).unwrap().as_str().parse::<usize>()?
                    * m.get(3).unwrap().as_str().parse::<usize>()?
            } else if m.get(4).is_some() {
                enabled = 1;
                0
            } else if m.get(5).is_some() {
                enabled = 0;
                0
            } else {
                unreachable!()
            })
        })
        .sum_ok()
}

#[test]
fn test_p1() {
    handle_res(p1());
}

#[test]
fn test_p2() {
    handle_res(p2());
}
