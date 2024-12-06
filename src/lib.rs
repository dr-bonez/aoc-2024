#![feature(array_try_from_fn)]
#![feature(extend_one)]
#![feature(try_trait_v2)]

use std::fmt::Debug;
use std::iter::Sum;
use std::ops::{Add, AddAssign, ControlFlow, FromResidual, Try};
use std::process::Output;

use anyhow::{anyhow, ensure, Error};

mod d1;
mod d2;
mod d3;
mod d4;
mod d5;

mod prelude {
    pub use crate::*;
    pub use anyhow::{anyhow, ensure, Error};
    pub use itertools::Itertools;
    pub use std::fs::File;
    pub use std::io::{BufRead, BufReader};
}

pub fn collect_arr<const N: usize, T>(mut i: impl Iterator<Item = T>) -> Result<[T; N], Error> {
    let arr = std::array::try_from_fn(|_| {
        i.next()
            .ok_or_else(|| anyhow!("incorrect number of elements for array: expected {N}"))
    });
    ensure!(i.next().is_none(), "too many elements in array");
    arr
}

pub fn handle_res<T: Debug>(res: Result<T, Error>) {
    match res {
        Ok(a) => {
            println!("{a:?}");
        }
        Err(e) => {
            eprintln!("{e}");
            eprintln!("{e:?}");
            std::process::exit(1);
        }
    }
}

pub trait MoreIterTools: Iterator {
    fn try_unzip<A, B, FromA, FromB>(self) -> Result<(FromA, FromB), Error>
    where
        FromA: Default + Extend<A>,
        FromB: Default + Extend<B>,
        Self: Sized + Iterator<Item = Result<(A, B), Error>>;
    fn try_fold<Acc, E, F>(self, init: Acc, f: F) -> Result<Acc, E>
    where
        F: FnMut(Acc, Self::Item) -> Result<Acc, E>;
    fn sum_ok(self) -> Self::Item
    where
        Self::Item: Try + FromResidual,
        <Self::Item as Try>::Output: Default + AddAssign;
    fn boxed<'a>(self) -> Box<dyn Iterator<Item = Self::Item> + 'a>
    where
        Self: 'a;
}

impl<T: Iterator> MoreIterTools for T {
    fn try_unzip<A, B, FromA, FromB>(mut self) -> Result<(FromA, FromB), Error>
    where
        FromA: Default + Extend<A>,
        FromB: Default + Extend<B>,
        Self: Sized + Iterator<Item = Result<(A, B), Error>>,
    {
        let mut a = FromA::default();
        let mut b = FromB::default();
        if let Some(len) = self.size_hint().1 {
            a.extend_reserve(len);
            b.extend_reserve(len);
        }
        loop {
            if let Some((x_a, x_b)) = self.next().transpose()? {
                a.extend_one(x_a);
                b.extend_one(x_b);
            } else {
                break;
            }
        }
        Ok((a, b))
    }
    fn try_fold<Acc, E, F>(self, init: Acc, mut f: F) -> Result<Acc, E>
    where
        F: FnMut(Acc, Self::Item) -> Result<Acc, E>,
    {
        let mut acc = init;
        for x in self {
            acc = f(acc, x)?;
        }
        Ok(acc)
    }
    fn sum_ok(self) -> Self::Item
    where
        Self::Item: Try + FromResidual,
        <Self::Item as Try>::Output: Default + AddAssign,
    {
        let mut res = <Self::Item as Try>::Output::default();
        for item in self {
            match item.branch() {
                ControlFlow::Continue(item) => res += item,
                ControlFlow::Break(err) => return Self::Item::from_residual(err),
            }
        }
        Self::Item::from_output(res)
    }
    fn boxed<'a>(self) -> Box<dyn Iterator<Item = Self::Item> + 'a>
    where
        Self: 'a,
    {
        Box::new(self)
    }
}
