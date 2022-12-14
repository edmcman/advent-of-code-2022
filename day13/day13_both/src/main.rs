use json;
use std::cmp::Ordering;
use std::str::FromStr;
use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum Packet {
    Int(i32),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Int(l), Self::Int(r)) => l.partial_cmp(&r),
            (Self::List(l), Self::List(r)) => l.partial_cmp(r),
            (l @ Self::Int(_), Self::List(r)) => {
                let v = vec![l.clone()];
                v.partial_cmp(r)
            }
            (Self::List(l), r @ Self::Int(_)) => {
                let v = vec![r.clone()];
                l.partial_cmp(&v)
            }
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Packet {
    fn from_json(j: &json::JsonValue) -> Option<Self> {
        match j {
            json::JsonValue::Number(i) => {
                let f: f64 = (*i).into();
                Some(Self::Int(f.round() as i32))
            }
            json::JsonValue::Array(a) => {
                let t: Option<Vec<Self>> = a
                    .iter()
                    .map(|v| Self::from_json(v))
                    .collect();
                t.map(|o| Self::List(o))
            }
            _ => None,
        }
    }
}

impl FromStr for Packet {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let j = match json::parse(s) {
            Ok(j) => j,
            Err(_) => return Err("json failed"),
        };
        match Self::from_json(&j) {
            Some(r) => Ok(r),
            None => Err("Unable to parse as json"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct PacketPair(Packet, Packet);

impl PacketPair {
    fn from_strs(s: &[String]) -> Self {
        PacketPair(
            s.get(0).unwrap().parse().unwrap(),
            s.get(1).unwrap().parse().unwrap(),
        )
    }

    fn is_sorted(&self) -> bool {
        self.0 <= self.1
    }
}

fn main() {
    let input: Vec<_> = std::io::stdin().lines().map(|l| l.unwrap()).collect();

    let pairs: Vec<_> = input
        .chunks(3)
        .map(|strs| PacketPair::from_strs(strs))
        .collect();

    //dbg!(&z);

    let p1: usize = pairs
        .iter()
        .enumerate()
        .filter_map(|(i, pair)| if pair.is_sorted() { Some(i + 1) } else { None })
        .sum();

    println!("Part 1: {p1}");

    let dividers = ["[[2]]", "[[6]]"].map(|s| Packet::from_str(s).expect("json"));

    // We don't need pairs anymore, so we can own it.
    let p2v: Vec<_> = pairs.into_iter().flat_map(|s| [s.0, s.1]).chain(dividers.clone().into_iter()).sorted().collect();
    //p2v.iter().for_each(|j| println!("{:?}", j));
    let p2: usize = dividers.map(|divider| p2v.iter().enumerate().find_map(|(i,e)| if *e == divider { Some(i+1) } else {None}).unwrap()).iter().product();
    println!("Part 2: {p2}");
}
