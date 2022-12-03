use std::collections::{HashMap,HashSet};

struct Rucksack(String, String);

impl Rucksack {
    fn from_string(s: &str) -> Self {

        let len = s.len();
        assert!(len % 2 == 0);        

        let s1 = &s[..len/2];
        let s2 = &s[len/2..];

        Self(String::from(s1),
             String::from(s2))
    }

    fn common(&self) -> char {
        let s1 = self.0.chars().collect::<HashSet<_>>();
        let s2 = self.1.chars().collect::<HashSet<_>>();

        let i = s1.intersection(&s2).collect::<Vec<_>>();
        dbg!(&i);
        dbg!(&self.0);
        dbg!(&self.1);
        assert!(i.len() == 1);

        *i[0]
    }
}

fn char_to_priority(c: char) -> u32 {
    let r = ('a'..='z').chain('A'..='Z');
    let d = 1..=52;

    let m : HashMap<_, _> = r.zip(d).collect();

    *m.get(&c).unwrap()
}

fn main() {
    let sum : u32 =
        std::io::stdin().lines()
        .map(Result::unwrap)
        .map(|s| Rucksack::from_string(&s))
        .map(|r| r.common())
        .map(char_to_priority)
        .sum();

    println!("The sum is {sum}")
}
