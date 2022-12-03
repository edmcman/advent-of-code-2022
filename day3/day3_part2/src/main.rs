use itertools::Itertools;
use std::collections::{HashMap, HashSet};

struct ElfGroup(String, String, String);

impl ElfGroup {
    fn common(&self) -> char {
        let toset = |s: &str| s.chars().collect::<HashSet<_>>();

        let i = toset(&self.0);
        let i = i.intersection(&toset(&self.1))
            .copied()
            //.map(|v| *v)
            .collect::<HashSet<_>>();
        let i = i.intersection(&toset(&self.2))
            .copied()
            //.map(|v| *v)
            .collect::<Vec<_>>();

        assert!(i.len() == 1);

        i[0]
    }
}

fn char_to_priority(c: char) -> u32 {

    let r = ('a'..='z').chain('A'..='Z');
    let d = 1..=52;

    let m: HashMap<_, _> = r.zip(d).collect();

    *m.get(&c).unwrap()
}

fn main() {
    let sum:u32 = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .tuples()
        .map(|(s1, s2, s3)| ElfGroup(s1, s2, s3))
        .map(|eg| eg.common())
        .map(char_to_priority)
        .sum();

    println!("sum: {sum}");
}
