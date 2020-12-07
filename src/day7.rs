// --- Day 7: Handy Haversacks ---

// You land at the regional airport in time for your next flight. In
// fact, it looks like you'll even have time to grab some food: all
// flights are currently delayed due to issues in luggage processing.

// Due to recent aviation regulations, many rules (your puzzle input)
// are being enforced about bags and their contents; bags must be
// color-coded and must contain specific quantities of other
// color-coded bags. Apparently, nobody responsible for these
// regulations considered how long they would take to enforce!

// For example, consider the following rules:

// light red bags contain 1 bright white bag, 2 muted yellow bags.
// dark orange bags contain 3 bright white bags, 4 muted yellow bags.
// bright white bags contain 1 shiny gold bag.
// muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
// shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
// dark olive bags contain 3 faded blue bags, 4 dotted black bags.
// vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
// faded blue bags contain no other bags.
// dotted black bags contain no other bags.

// These rules specify the required contents for 9 bag types. In this
// example, every faded blue bag is empty, every vibrant plum bag
// contains 11 bags (5 faded blue and 6 dotted black), and so on.

// You have a shiny gold bag. If you wanted to carry it in at least
// one other bag, how many different bag colors would be valid for the
// outermost bag? (In other words: how many colors can, eventually,
// contain at least one shiny gold bag?)

// In the above rules, the following options would be available to you:

// A bright white bag, which can hold your shiny gold bag directly.

// A muted yellow bag, which can hold your shiny gold bag directly,
// plus some other bags.

// A dark orange bag, which can hold bright white and muted yellow
// bags, either of which could then hold your shiny gold bag.

// A light red bag, which can hold bright white and muted yellow bags,
// either of which could then hold your shiny gold bag.

// So, in this example, the number of bag colors that can eventually
// contain at least one shiny gold bag is 4.

// How many bag colors can eventually contain at least one shiny gold
// bag? (The list of rules is quite long; make sure you get all of
// it.)

// --- Part Two ---

// It's getting pretty expensive to fly these days - not because of
// ticket prices, but because of the ridiculous number of bags you
// need to buy!

// Consider again your shiny gold bag and the rules from the above
// example:

// faded blue bags contain 0 other bags.

// dotted black bags contain 0 other bags.

// vibrant plum bags contain 11 other bags: 5 faded blue bags and 6
// dotted black bags.

// dark olive bags contain 7 other bags: 3 faded blue bags and 4
// dotted black bags.

// So, a single shiny gold bag must contain 1 dark olive bag (and the
// 7 bags within it) plus 2 vibrant plum bags (and the 11 bags within
// each of those): 1 + 1*7 + 2 + 2*11 = 32 bags!

// Of course, the actual rules have a small chance of going several
// levels deeper than this example; be sure to count all of the bags,
// even if the nesting becomes topologically impractical!

// Here's another example:

// shiny gold bags contain 2 dark red bags.
// dark red bags contain 2 dark orange bags.
// dark orange bags contain 2 dark yellow bags.
// dark yellow bags contain 2 dark green bags.
// dark green bags contain 2 dark blue bags.
// dark blue bags contain 2 dark violet bags.
// dark violet bags contain no other bags.

// In this example, a single shiny gold bag must contain 126 other
// bags.

// How many individual bags are required inside your single shiny gold
// bag?

use petgraph::{algo::all_simple_paths, prelude::DiGraph, Direction};

use std::collections::{HashMap, HashSet};
use std::path::Path;

pub fn part1() -> u32 {
    let path = Path::new("day7-input.txt");
    let input = std::fs::read_to_string(path).expect("read");
    let rules = input.as_str().lines();
    let mut all_nodes = HashSet::new();
    let mut all_edges = Vec::new();
    for (node, mut edges) in rules.map(parse_rule) {
        all_nodes.insert(node);
        all_edges.append(&mut edges);
    }
    let mut nodeids = HashMap::new();
    for (index, node) in all_nodes.iter().enumerate() {
        nodeids.insert(node.to_string(), index as u32);
    }
    let graph = DiGraph::<u32, u32, _>::from_edges(
        all_edges
            .iter()
            .map(|(a, s, w)| (nodeid(&nodeids, a), nodeid(&nodeids, s), w)),
    );
    count_to(graph, nodeid(&nodeids, "shiny gold"))
}

pub fn part2() -> u32 {
    let path = Path::new("day7-input.txt");
    let input = std::fs::read_to_string(path).expect("read");
    let rules = input.as_str().lines();
    let mut all_nodes = HashSet::new();
    let mut all_edges = Vec::new();
    for (node, mut edges) in rules.map(parse_rule) {
        all_nodes.insert(node);
        all_edges.append(&mut edges);
    }
    let mut nodeids = HashMap::new();
    for (index, node) in all_nodes.iter().enumerate() {
        nodeids.insert(node.to_string(), index as u32);
    }
    let graph = DiGraph::<u32, u32>::from_edges(
        all_edges
            .iter()
            .map(|(a, s, w)| (nodeid(&nodeids, a), nodeid(&nodeids, s), w)),
    );
    sum_from(&graph, nodeid(&nodeids, "shiny gold").into(), 1) - 1
}

fn parse_rule(rule: &str) -> (String, Vec<(String, String, u32)>) {
    let a_rest: Vec<_> = rule.split(" bags contain ").collect();
    let (a, rest) = (a_rest[0], a_rest[1]);
    let w_ds: Vec<(u32, String)> = match rest {
        "no other bags." => vec![],
        other => other
            .split(',')
            .map(|w_d| {
                let w_d1_d2_rest: Vec<_> = w_d.split_whitespace().collect();
                let (w, d1, d2, _) = (
                    w_d1_d2_rest[0],
                    w_d1_d2_rest[1],
                    w_d1_d2_rest[2],
                    w_d1_d2_rest[3],
                );
                let mut d = d1.to_string();
                d.push_str(" ");
                d.push_str(d2);
                (w.parse().expect("u32-parse"), d)
            })
            .collect(),
    };
    (
        a.to_string(),
        w_ds.into_iter()
            .map(|(w, d)| (a.to_string(), d, w))
            .collect(),
    )
}

fn count_to(graph: DiGraph<u32, u32>, dindex: u32) -> u32 {
    let mut nodes = HashSet::new();
    for sindex in graph.externals(Direction::Incoming) {
        let paths = all_simple_paths::<Vec<_>, _>(&graph, sindex, dindex.into(), 0, None);
        for path in paths {
            for node in path {
                nodes.insert(node);
            }
        }
    }
    let count = nodes.len();
    if count > 0 {
        (count - 1) as u32
    } else {
        0
    }
}

fn sum_from(graph: &DiGraph<u32, u32>, start: petgraph::prelude::NodeIndex, weight: u32) -> u32 {
    let edges = graph.edges(start);
    let nodes = graph.neighbors(start);
    let mut sum = 0;
    for (edge, node) in edges.zip(nodes) {
        sum += sum_from(&graph, node, *edge.weight());
    }
    weight + weight * sum
}

fn nodeid(nodeids: &HashMap<String, u32>, node: &str) -> u32 {
    *nodeids.get(node).expect("get")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(124, part1())
    }

    #[test]
    fn test_part2() {
        assert_eq!(34862, part2())
    }
}
