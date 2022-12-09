use std::collections::{HashMap, HashSet};
#[macro_use]
extern crate lazy_static;

// Position relative to starting position
#[derive(Hash, PartialEq, Eq, Clone, Debug, Copy)]
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
        let (r, c) = self.delta(other);
        [r, c].iter().all(|d| d.abs() <= 1)
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
        (Self::Left, Position { r: 0, c: -1 }),
        (Self::Right, Position { r: 0, c: 1 }),
        (Self::Up, Position { r: 1, c: 0 }),
        (Self::Down, Position { r: -1, c: 0 }),
    ];

    fn get_delta(&self) -> Position {
        lazy_static! {
            static ref DELTA_MAP: HashMap<Direction, Position> = Direction::DELTA_LIST.into();
        }

        DELTA_MAP.get(self).unwrap().clone()
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct State {
    locs: Vec<Position>,
    tail_visit: HashSet<Position>,
}

impl State {
    // After the head moves, catch the tail up
    fn tail_move(h: &Position, t: &mut Position) -> () {
        let would_be_touching = h.is_touching(&t);
        if would_be_touching {
            return;
        };
        let (dr, dc) = h.delta(t);
        let trick = |d: isize| if d == 0 { 0 } else { d / d.abs() };
        t.r += trick(dr);
        t.c += trick(dc);
    }

    fn head_move(h: &mut Position, d: &Direction) -> () {
        let delta = d.get_delta();
        h.r += delta.r;
        h.c += delta.c;
    }

    fn do_move(mut self, d: &Direction) -> Self {
        self.tail_visit.insert(self.locs.last().unwrap().clone());

        // Step 1: Move the head
        State::head_move(self.locs.first_mut().expect("need a head"), d);

        // Step 2: Move all the tails in sequence
        // Grr: https://internals.rust-lang.org/t/a-windows-mut-method-on-slice/16941/11
        let range_vec: Vec<usize> = (0..(self.locs.len())).collect();
        let index_windows = range_vec.windows(2);
        index_windows.for_each(|v| {
            let index1 = v.get(0).unwrap().clone();
            let index2 = v.get(1).unwrap().clone();
            // Ugh, need to clone here to have references to both the head and tail.
            State::tail_move(
                &self.locs.get(index1).unwrap().clone(),
                self.locs.get_mut(index2).unwrap(),
            );
        });

        // println!("State {:?}", self);

        let tail = self.locs.last().unwrap().clone();
        println!("Head is at {:?}", self.locs.first().unwrap());
        println!("Tail is at {:?}", tail);

        self.tail_visit.insert(tail);

        self
    }
}

fn main() {
    let part1_initial_state = State {
        locs: [Position { r: 0, c: 0 }].repeat(2),
        tail_visit: HashSet::new(),
    };

    let part2_initial_state = State {
        locs: [Position { r: 0, c: 0 }].repeat(10),
        tail_visit: HashSet::new(),
    };

    let moves: Vec<Direction> = std::io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|s| {
            s.split_once(' ')
                .map(|(a, b)| (a.to_string(), b.to_string()))
                .unwrap()
        })
        .map(|(d, n)| {
            (
                Direction::from_char(d.chars().next().unwrap()).unwrap(),
                n.parse::<usize>().unwrap(),
            )
        })
        .flat_map(|(d, n)| [d].repeat(n))
        .collect();

    let part1_final = moves.iter().fold(part1_initial_state, |st, m| st.do_move(&m));
    println!("part1: {}", part1_final.tail_visit.len());

    let part2_final = moves.iter().fold(part2_initial_state, |st, m| st.do_move(&m));
    println!("part2: {}", part2_final.tail_visit.len());
}
