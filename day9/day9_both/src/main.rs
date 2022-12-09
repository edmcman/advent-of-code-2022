use std::collections::{HashSet, HashMap};
#[macro_use]
extern crate lazy_static;

// Position relative to starting position
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Position {
    r: isize,
    c: isize,
}

impl Position {
    fn delta(&self, other: &Position) -> (isize, isize) {
        let delta_r = (self.r - other.r);
        let delta_c = (self.c - other.c);
        (delta_r, delta_c)
    }

        fn is_touching(&self, other: &Position) -> bool {
        let (r,c) = self.delta(other);
        [r,c].iter().all(|d| d.abs() <= 1)
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn from_char(c: char) -> Option<Self> {
        lazy_static! {
            static ref M: HashMap<char, Direction> = {
                [
                    ('L', Direction::Left),
                    ('R', Direction::Right),
                    ('U', Direction::Up),
                    ('D', Direction::Down),
                ]
                .into()
            };
        }
        M.get(&c).cloned()
    }

    const DELTA_LIST: [(Direction, Position); 4] = [
        (Self::Left, Position { r: -1, c: 0 }),
        (Self::Right, Position { r: 1, c: 0 }),
        (Self::Up, Position { r: 0, c: 1 }),
        (Self::Down, Position { r: 0, c: -1 }),
    ];

    fn get_delta(&self) -> Position {
        lazy_static! {
            static ref DELTA_MAP: HashMap<Direction, Position> =
                Direction::DELTA_LIST.into();
        }

        DELTA_MAP.get(self).unwrap().clone()
    }
}

struct State {
    head: Position,
    tail: Position,
    tail_visit: HashSet<Position>
}

impl State {
    fn mov(self, d: &Direction) -> Self {
        let delta = d.get_delta();
        let new_head_position = Position {
            r: self.head.r + delta.r,
            c: self.head.c + delta.c,
        };

        let would_be_touching = new_head_position.is_touching(&self.tail);
        // If the tail would not be touching, move it in the same direction

        let new_tail = match would_be_touching {
            true => self.tail,
            false => {
                let (dr, dc) = new_head_position.delta(&self.tail);
                let trick = |d: isize| if d == 0 {0} else { d/d.abs() };
                Position {
                r: self.tail.r + trick(dr),
                c: self.tail.c + trick(dc)
            }},
        };

        let mut new_state = Self {
            head: new_head_position,
            tail: new_tail,
            tail_visit: self.tail_visit
        };

        new_state.tail_visit.insert(new_state.tail.clone());

        new_state

    }
}

fn main() {
    let initial_state = State {
        head: Position { r: 0, c: 0 },
        tail: Position { r: 0, c: 0 },
        tail_visit: HashSet::new()
    };

    let z = std::io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|s| {
            s.split_once(' ')
                .map(|(a, b)| (a.to_string(), b.to_string()))
                .unwrap()
        })
        .map(|(d, n)| (Direction::from_char(d.chars().next().unwrap()).unwrap(), n.parse::<usize>().unwrap()))
        .flat_map(|(d,n)| [d].repeat(n))
        .fold(initial_state, |st, m| st.mov(&m));

    println!("tail visit: {:?}", z.tail_visit);

    println!("part1: {}", z.tail_visit.len())
}
