use crate::prelude::*;

fn parse_input() -> Result<(), Error> {
    BufReader::new(File::open("./input/d#.txt")?).lines()
}

fn p1() -> Result<(), Error> {
    Ok(())
}

fn p2() -> Result<(), Error> {
    Ok(())
}

#[test]
fn test_p1() {
    handle_res(p1());
}

#[test]
fn test_p2() {
    handle_res(p2());
}
