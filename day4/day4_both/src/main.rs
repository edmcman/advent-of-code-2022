use std::{ops::RangeInclusive, collections::HashSet};

struct Range(u32, u32);


impl Range {
    fn from_string(s: &str) -> Self {
        match s.split_once('-') {
            Some((a,b)) => Self(a.parse().unwrap(), b.parse().unwrap()),
            _ => todo!("uhoh")
        }
    }

    fn range(&self) -> RangeInclusive<u32> {
        self.0 ..= self.1
    }

    fn subset(&self, r: &Range) -> bool {
        let s1 : HashSet<_> = self.range().collect();
        let s2 : HashSet<_> = r.range().collect();
        s1.is_subset(&s2) || s2.is_subset(&s1)
    }

    fn overlap(&self, r: &Range) -> bool {
        let s1 : HashSet<_> = self.range().collect();
        let s2 : HashSet<_> = r.range().collect();
        !s1.is_disjoint(&s2)
    }

}

fn main() {
    let range_pairs = std::io::stdin()
    .lines()
    .map(|o| o.unwrap())
    .map(|l| match l.split_once(',') {
        Some((a,b)) => (Range::from_string(a), Range::from_string(b)),
        _ => todo!("uh oh")
    }).collect::<Vec<_>>();

    // Annoying: can't reuse the iterator...

    let subsets = range_pairs.iter().filter(|(r1,r2)| r1.subset(r2));
    let overlaps = range_pairs.iter().filter(|(r1, r2)| r1.overlap(r2));

    println!("part1: {}", subsets.count());
    println!("part2: {}", overlaps.count());
}
