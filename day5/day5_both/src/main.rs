use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

/*
Day 5

Wow, this one was annoying.  The lack of group_by and sorted on Rust's iterator's is annoying.  The group_by in itertools has a terrible and confusing interface.  I ended up using a mutable map to compute the moves.  I guess it's ok, but this one really showed Rust's immutable weaknesses.
*/

#[derive(Debug)]
struct Move {
    num: usize,
    src: usize,
    dst: usize,
}

impl Move {
    fn from_moveline(l: &str) -> Option<Self> {
        lazy_static! {
            static ref re: Regex =
                Regex::new(r"^move (?P<num>\d+) from (?P<src>\d+) to (?P<dst>\d+)$").unwrap();
        }

        let g = re.captures(l)?;

        let to_num = |o: regex::Match| o.as_str().parse::<usize>().unwrap();

        let do_group = |n: &str| to_num(g.name(n).unwrap());

        let t = Self {
            num: do_group("num"),
            src: do_group("src"),
            dst: do_group("dst"),
        };

        Some(t)
    }
}

fn parse_stackline(l: &str) -> Vec<(usize, char)> {
    l.chars()
        .enumerate()
        .filter_map(|t| match t {
            (i, v) if i % 4 == 1 && v != ' ' => Some((i / 4 + 1, v)),
            (_, _) => None,
        })
        .collect()
}

fn process_move(m: &mut std::collections::HashMap<usize, Vec<char>>, mov: &Move, part_one: bool) {
    dbg!("Processing {}", mov);

    let src = m.get_mut(&mov.src).expect("Source stack");
    let srclen = src.len();
    let elements = src.drain(srclen - mov.num..);
    let mut moved_guys: Vec<_> = if part_one {
        elements.rev().collect()
    } else {
        elements.collect()
    };

    let dst = m.get_mut(&mov.dst).expect("Dest stack");

    dbg!(&moved_guys);
    dst.append(&mut moved_guys);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let part_one = match args.get(1).map(|s| s.as_str()) {
        Some("first") => true,
        Some("second") => false,
        Some(_) => panic!("expected first or second"),
        None => true,
    };

    let input: Vec<String> = std::io::stdin().lines().map(|o| o.unwrap()).collect();

    // Parse the stacks
    let mut stack_lines: Vec<&String> = input.iter().take_while(|s| **s != "").collect();

    stack_lines.pop();

    let m = stack_lines
        .iter()
        .map(|s| parse_stackline(s))
        .inspect(|h| println!("ha {:?}", h))
        .flatten()
        .sorted_by_key(|(k, _)| *k)
        .group_by(|(i, c)| *i);

    let m = m.into_iter();

    // The groupby interface is so awkward!

    let mut stackmap: HashMap<usize, Vec<_>> = m
        .map(|(key, group)| {
            let v: Vec<_> = group.map(|(a, b)| b).collect();
            // Reverse
            let v: Vec<_> = v.iter().rev().copied().collect();
            (key, v)
        })
        .collect();

    println!("initial stackmap: {:?}", stackmap);

    let moves: Vec<_> = input
        .iter()
        .filter_map(|s| Move::from_moveline(s))
        .collect();

    println!("moves: {:?}", moves);

    moves
        .iter()
        .for_each(|mov| process_move(&mut stackmap, mov, part_one));

    println!("final stackmap: {:?}", stackmap);

    let top = stackmap
        .keys()
        .sorted()
        .map(|i| stackmap.get(i).unwrap().last().unwrap());
    let top_str: String = top.collect();

    println!("top: {top_str}");
}
