use std::collections::HashMap;
use std::collections::VecDeque;

struct Grid(Vec<Vec<char>>);

fn char_to_elevation(c: &char) -> char {
    match c {
        'S' => 'a',
        'E' => 'z',
        _ => *c,
    }
}

impl Grid {
    fn rows(&self) -> usize {
        self.0.len()
    }

    fn cols(&self) -> usize {
        self.0.get(0).unwrap().len()
    }

    fn get_elevation(&self, p: &Position) -> Option<char> {
        //dbg!(p);
        let r = self.0.get(p.r)?;
        let c = r.get(p.c)?;
        Some(char_to_elevation(c))
    }

    fn find(&self, c: char) -> impl Iterator<Item = Position> + '_ {
        let v: Vec<_> = self
            .0
            .iter()
            .enumerate()
            .flat_map(|(ri, r)| {
                r.iter()
                    .enumerate()
                    .filter_map(|(ci, c2)| if c == *c2 { Some(ci) } else { None })
                    .map(move |ci| (ri, ci))
            })
            .map(|(r, c)| Position {
                r: r.try_into().unwrap(),
                c: c.try_into().unwrap(),
            })
            .collect();
        v.into_iter()
    }

    fn find_start(&self) -> Position {
        self.find('S').next().unwrap()
    }

    fn find_end(&self) -> Position {
        self.find('E').next().unwrap()
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const DELTAS: [(Direction, (isize, isize)); 4] = [
        (Self::Up, (-1, 0)),
        (Self::Down, (1, 0)),
        (Self::Left, (0, -1)),
        (Self::Right, (0, 1)),
    ];
}

#[derive(PartialEq, Debug, Clone, Eq, Hash)]

struct Position {
    r: usize,
    c: usize,
}

impl Position {
    fn mov(&self, d: &Direction) -> Option<Position> {
        let (_, (rd, cd)) = Direction::DELTAS
            .iter()
            .find(|(d2, _deltas)| d == d2)
            .unwrap();
        Some(Position {
            r: usize::try_from(isize::try_from(self.r).ok()? + *rd).ok()?,
            c: usize::try_from(isize::try_from(self.c).ok()? + *cd).ok()?,
        })
    }

    fn next_move(&self, g: &Grid, d: &Direction) -> Option<Position> {
        //dbg!(self);
        let current_elevation = g.get_elevation(&self)?;
        //dbg!(&current_elevation);
        let max_elevation = std::char::from_u32(current_elevation as u32 + 1)?;
        //dbg!(&max_elevation);
        //dbg!(d);
        let next_position = self.mov(d)?;
        //dbg!(&next_position);
        let next_elevation = g.get_elevation(&next_position)?;
        //dbg!(&next_elevation);
        /*        if (current_elevation == 'c') {
                    println!("wtf {current_elevation} {next_elevation} {max_elevation}");
                    println!("{}", next_elevation <= max_elevation);
                }
        */
        //dbg!(&next_position);
        //dbg!((g.rows(),g.cols()));
        if (next_elevation <= max_elevation)
            && (next_position.r < g.rows())
            && (next_position.c < g.cols())
        {
            Some(next_position)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct State {
    p: Position,
    history: Vec<(Position, Direction)>,
}

fn shortest_path(start_position: Position, grid: &Grid) -> Option<usize> {
    let initial_st = State {
        p: start_position,
        history: vec![],
    };

    let mut queue = VecDeque::from([initial_st]);

    let mut best: HashMap<Position, usize> = HashMap::new();

    let dirs = [
        Direction::Left,
        Direction::Right,
        Direction::Up,
        Direction::Down,
    ];

    let end_state = grid.find_end();

    // TODO: Use priority queue
    while let Some(st) = queue.pop_front() {
        //println!("Popped {:?} from queue", st);

        let pos = &st.p;
        let is_the_best = best
            .get(pos)
            .map(|best| st.history.len() < *best)
            .unwrap_or(true);

        if is_the_best {
            //println!("I am the best! {:?}", st.p);
            best.insert(st.p.clone(), st.history.len());
            if st.p == end_state {
                println!("New best for end state! {}", st.history.len());
            }

            let () = dirs
                .iter()
                .flat_map(|d| pos.next_move(&grid, d).map(|new_pos| (new_pos, d)))
                .map(|(new_pos, d)| {
                    let mut new_history = st.history.clone();
                    new_history.push((new_pos.clone(), *d));
                    State {
                        p: new_pos,
                        history: new_history,
                    }
                })
                //.inspect(|st| println!("Queueing state: {:?}", st))
                .for_each(|st| {
                    queue.push_back(st);
                });
        }
    }

    best.get(&end_state).cloned()
}

fn main() {
    let grid: Vec<_> = std::io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect();
    let grid = Grid(grid);

    let part1 = shortest_path(grid.find_start(), &grid).unwrap();
    println!("Part 1: {part1}");

    let part2 = grid
        .find('a')
        .flat_map(|pos| shortest_path(pos.clone(), &grid))
        .min_by_key(|x| *x)
        .unwrap();
    println!("Part 2: {part2}");
}
