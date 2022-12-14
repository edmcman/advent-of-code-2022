use json;
use std::cmp::Ordering;
use std::str::FromStr;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum Packet {
    Int(i32),
    List(Vec<Box<Packet>>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Int(l), Self::Int(r)) => l.partial_cmp(&r),
            (Self::List(l), Self::List(r)) => l.partial_cmp(r),
            (l @ Self::Int(_), Self::List(r)) => {
                let v = vec![Box::new(l.clone())];
                v.partial_cmp(r)
            }
            (Self::List(l), r @ Self::Int(_)) => {
                let v = vec![Box::new(r.clone())];
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
                let t: Option<Vec<Box<Self>>> = a
                    .iter()
                    .map(|v| Self::from_json(v).map(|o| Box::new(o)))
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
    let z: Vec<_> = std::io::stdin().lines().map(|l| l.unwrap()).collect();

    let z: Vec<_> = z
        .chunks(3)
        .map(|strs| PacketPair::from_strs(strs))
        .collect();

    //dbg!(&z);

    let n: usize = z
        .iter()
        .enumerate()
        .filter_map(|(i, pair)| if pair.is_sorted() { Some(i + 1) } else { None })
        .sum();

    println!("Sorted: {n}");
}
