use petgraph::algo::{astar, dijkstra};
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use petgraph::{Directed, Graph};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::str::FromStr;

use crate::prelude::*;

// const INPUT_RULES: &str = r#"
// 47|53
// 97|13
// 97|61
// 97|47
// 75|29
// 61|13
// 75|53
// 29|13
// 97|29
// 53|29
// 61|53
// 97|53
// 61|29
// 47|13
// 75|47
// 97|75
// 47|61
// 75|61
// 47|29
// 75|13
// 53|13
// "#;
// const INPUT_UPDATES: &str = r#"
// 75,47,61,53,29
// 97,61,53,29,13
// 75,29,13
// 75,97,47,61,53
// 61,13,29
// 97,13,75,29,47
// "#;

const INPUT_RULES: &str = include_str!("./input/d5_rules.txt");
const INPUT_UPDATES: &str = include_str!("./input/d5_updates.txt");

#[derive(Debug)]
struct Rules {
    graph: Graph<(), (), Directed, u32>,
    // djikstra_cache: BTreeMap<u32, HashMap<NodeIndex, u32>>,
}
impl Rules {
    fn for_nodes(&self, nodes: BTreeSet<u32>) -> Self {
        Self {
            graph: Graph::from_edges(nodes.iter().flat_map(|e| {
                self.graph
                    .edges((*e).into())
                    .filter(|e| nodes.contains(&(e.target().index() as u32)))
                    .map(|e| (e.source(), e.target()))
            })),
            // djikstra_cache: BTreeMap::new(),
        }
    }
}
impl FromStr for Rules {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .filter_map(|l| l.split_once("|"))
            .map(|(l, r)| Ok::<_, Error>((l.parse::<u32>()?, r.parse::<u32>()?)))
            .try_fold(Graph::new(), |mut acc, x| {
                acc.extend_with_edges([x?]);
                Ok(acc)
            })
            .map(|graph| Self {
                graph,
                // djikstra_cache: BTreeMap::new(),
            })
    }
}

fn is_less(r: &Rules, a: u32, b: u32) -> bool {
    // if let Some(d) = r.djikstra_cache.get(&a) {
    //     return d.get(&b.into()).is_some();
    // }
    let dijkstra = dijkstra(&r.graph, a.into(), None, |_| 1);
    // r.djikstra_cache
    //     .insert(a, dijkstra);
    // is_less(r, a, b)
    dijkstra.get(&b.into()).is_some()
}

fn cmp(rules: &Rules, a: u32, b: u32) -> Option<Ordering> {
    let less = is_less(rules, a, b);
    let more = is_less(rules, b, a);
    if less && more {
        dbg!(
            astar::astar(&rules.graph, a.into(), |x| x == b.into(), |_| 1, |_| 0),
            astar::astar(&rules.graph, b.into(), |x| x == a.into(), |_| 1, |_| 0),
        );
        panic!()
    }
    if less {
        Some(Ordering::Less)
    } else if more {
        Some(Ordering::Greater)
    } else {
        None
    }
}

struct Update(Vec<u32>);
impl FromStr for Update {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.split(",").map(|s| s.parse()).collect::<Result<_, _>>()?,
        ))
    }
}
impl Update {
    fn rules(&self, rules: &Rules) -> Rules {
        rules.for_nodes(self.0.iter().copied().collect())
    }
    fn is_correct(&self, rules: &Rules) -> bool {
        self.0
            .is_sorted_by(|a, b| cmp(rules, *a, *b).map_or(true, |c| c == Ordering::Less))
    }

    fn fix(mut self, rules: &Rules) -> Self {
        self.0
            .sort_by(|a, b| cmp(rules, *a, *b).unwrap_or(Ordering::Equal));
        self
    }

    fn mid(&self) -> Option<u32> {
        self.0.get(self.0.len() / 2).copied()
    }
}

fn p1() -> Result<u32, Error> {
    let rules = INPUT_RULES.parse()?;

    INPUT_UPDATES
        .lines()
        .filter(|l| !l.is_empty())
        .map(|s| s.parse::<Update>())
        .map_ok(|u| (u.rules(&rules), u))
        .filter_ok(|(rules, u)| u.is_correct(&rules))
        .filter_map_ok(|(_, u)| u.mid())
        .sum_ok()
}

fn p2() -> Result<u32, Error> {
    let rules = INPUT_RULES.parse()?;

    INPUT_UPDATES
        .lines()
        .filter(|l| !l.is_empty())
        .map(|s| s.parse::<Update>())
        .map_ok(|u| (u.rules(&rules), u))
        .filter_ok(|(rules, u)| !u.is_correct(&rules))
        .map_ok(|(rules, u)| u.fix(&rules))
        .filter_map_ok(|u| u.mid())
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
