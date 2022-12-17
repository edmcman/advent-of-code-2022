// A point is described from the lower left part of the bounding box of the
// shape.  There does not necessarily have to be a point at (0,0).
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// x, y
struct Point(usize, usize);

impl Point {
    fn move_in_dir(&self, d: &Dir) -> Option<Point> {
        match d {
            Dir::Left if self.0 > 0 => Some(Point(self.0 - 1, self.1)),
            Dir::Right if self.0 + 1 < 7 => Some(Point(self.0 + 1, self.1)),
            Dir::Down if self.1 > 0 => Some(Point(self.0, self.1 - 1)),
            _ => None,
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
enum Dir {
    Left,
    Right,
    Down,
}

impl Dir {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '<' => Some(Self::Left),
            '>' => Some(Self::Right),
            _ => None,
        }
    }
}

struct Grid(Vec<[bool; 7]>);

type Shape = Vec<Point>;

impl Grid {
    fn new() -> Self {
        Self(vec![[false; 7]; 21])
    }

    fn get(&self, p: &Point) -> Option<bool> {
        Some(*self.0.get(p.1)?.get(p.0)?)
    }

    fn get_mut(&mut self, p: &Point) -> &mut bool {
        self.0.get_mut(p.1).expect("row").get_mut(p.0).expect("col")
    }

    // Does the shape fit at point
    fn fits(&self, s: &Shape, p: &Point) -> bool {
        //self.0.resize(p.1 + 10, [false; 7]);

        s.iter().all(|sp| {
            let new_point = Point(p.0 + sp.0, p.1 + sp.1);

            if p.0 + sp.0 >= 7 {
                return false;
            }

            //dbg!(&new_point);
            self.get(&new_point)
                .map(|present| !present)
                .unwrap_or(false)
        })
    }

    fn can_move_down(&self, s: &Shape, p: &Point) -> bool {
        p.move_in_dir(&Dir::Down)
            .map(|p| self.fits(s, &p))
            .unwrap_or(false)
    }

    fn add_shape_to_point(&mut self, s: &Shape, base_p: &Point) {
        self.0.resize(self.0.len().max(base_p.1 + 20), [false; 7]);

        //self.0.extend([[false; 7]].iter().copied());

        s.iter().for_each(|p| {
            //dbg!(p);
            let new_point = Point(base_p.0 + p.0, base_p.1 + p.1);
            assert!(self.get(&new_point) == Some(false));
            *self.get_mut(&new_point) = true
        })
    }

    fn add_shape(&mut self, s: &Shape) {
        let next_point = Point(2, self.highest_rock() + 3);
        self.add_shape_to_point(s, &next_point)
    }

    fn find_floor(&self) -> &[[bool; 7]] {
        let ok = (0..7).map(|col| {
            self.0
                .iter()
                .rev()
                .enumerate()
                .find_map(|(i, b)| if *b.get(col).unwrap() { Some(i) } else { None })
                .unwrap()
        });
        //let ok: Vec<_> = ok.collect();
        let num = ok.max().unwrap();
        println!("We need to look at the last {num} rows");
        let slice = self.0.get(self.0.len()-31..).unwrap();
        //dbg!(slice.len());
        assert!(slice.len() == 31);
        slice
    }

    fn highest_rock(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(i, arr)| {
                if arr.iter().any(|b| *b) {
                    Some(i)
                } else {
                    None
                }
            })
            .max()
            .map(|max| max + 1)
            .unwrap_or(0)
    }
}

// Ugh.  Part 2 needs to return 3 values, but part 1 only needs 1.
fn tower_height(jet_pattern: &Vec<Dir>, nrocks: usize, p2: bool) -> Option<(usize, usize, usize)> {
    let mut grid = Grid::new();

    let jet_pattern_len = jet_pattern.len();
    let mut jet_pattern = jet_pattern.iter().cycle().enumerate();

    let shapes: [Shape; 5] = [
        (0..=3).map(|x| Point(x, 0)).collect(),
        vec![
            Point(0, 1),
            Point(1, 1),
            Point(2, 1),
            Point(1, 0),
            Point(1, 2),
        ],
        vec![
            Point(0, 0),
            Point(1, 0),
            Point(2, 0),
            Point(2, 1),
            Point(2, 2),
        ],
        (0..4).map(|y| Point(0, y)).collect(),
        vec![Point(0, 0), Point(0, 1), Point(1, 0), Point(1, 1)],
    ];

    //let mut jet_counter = 0;

    let mut floor_state = None;

    for i in 0..nrocks {
        dbg!(&i);
        let shape = shapes.get(i % shapes.len()).expect("math");

        let mut shape_position = Point(2, grid.highest_rock() + 3);

        for j in 0.. {
            //dbg!(&shape_position);

            //if !grid.can_move_down(shape, &shape_position) { break; }

            let (jet_counter, jet_dir) = jet_pattern.next().unwrap();
            //jet_counter += 1;

            if i > 0 && p2 && jet_counter % jet_pattern_len == 0
            /*&& shape_position.0 == 2*/
            {
                println!("I am a cycle! rocks={} {}", i, grid.highest_rock());
                let new_floor = grid.find_floor().to_vec();
                match floor_state.clone() {
                    Some((floor_state, old_rocks, old_height)) if new_floor == floor_state => {
                        let delta_rock = i-old_rocks;
                        let delta_height = grid.highest_rock() - old_height;
                        return Some((old_rocks, delta_rock, delta_height));
                    },
                    None => floor_state = Some((new_floor, i, grid.highest_rock())),
                    _ => ()
                };
                //println!("x={}", shape_position.0);
                //return Some(i);
                //return Some(grid.highest_rock());
            }

            let jet_point = shape_position
                .move_in_dir(jet_dir)
                .unwrap_or(shape_position);
            if grid.fits(shape, &jet_point) {
                //dbg!(("jet moved shape", jet_dir, &shape_position, &jet_point));
                shape_position = jet_point;
            } else {
                //dbg!(("jet did not move shape", jet_dir, &shape_position));
            }

            // Ok, move down.
            let down_point = shape_position.move_in_dir(&Dir::Down);
            let down_point_fits = down_point.map(|p| grid.fits(shape, &p)).unwrap_or(false);
            if down_point_fits {
                //dbg!(&shape_position);
                shape_position = down_point.expect("logic");
                //dbg!("fits");
            } else {
                //dbg!("no fits");

                //println!("wtf {jet_counter} {jet_pattern_len} {}", jet_counter % jet_pattern_len);

                break;
            }
        }

        //dbg!("boom");

        //dbg!(&shape_position);

        grid.add_shape_to_point(shape, &shape_position);

        //dbg!(grid.0.len());

/*        grid.0
            .get(0..20)
            .unwrap()
            .iter()
            .enumerate()
            .rev()
            .for_each(|(i, r)| {
                println!(
                    "g {i:0>2} {}",
                    r.map(|b| if b { '#' } else { '.' })
                        .iter()
                        .collect::<String>()
                );
            });
            */
        //        dbg!(&grid.0);
    }

    if p2 {
        None
    } else {
        Some((grid.highest_rock(), 0, 0))
    }
}

fn main() {
    let pattern: Option<Vec<Dir>> = std::io::stdin()
        .lines()
        .next()
        .map(|l| l.unwrap())
        .expect("pattern")
        .chars()
        .map(|c| Dir::from_char(c))
        .collect();
    let pattern = pattern.expect("pattern");

    println!("cool pattern len = {}", pattern.len());

    let (p1_height,_,_) = tower_height(&pattern, 2022, false).expect("p1");
    println!("height: {p1_height}");

    let (start_rock, delta_rock, delta_height) = tower_height(&pattern, pattern.len() * 5, true).unwrap();

    println!("Identified cycle: every {delta_rock} rocks starting at {start_rock} there is an increase of {delta_height} height");

    // 1000000000000 rocks
    // 1000000000000 % delta_rock

    let total_rocks: usize = 1000000000000;
    let remaining = 1000000000000 - start_rock;

    println!("The cycle starts at {start_rock}.  After that there are {remaining} rocks.");
    let cycles = remaining / delta_rock;
    let remainder = remaining % delta_rock;
    println!("In that {remaining} rocks there will be {cycles} complete cycles and {remainder} left over.");

    let (start_height,_,_) = tower_height(&pattern, start_rock, false).expect("start height");

    //let at_cycle = wow - start_rock;

    println!("The cycle starts at {start_rock} which has a height of {start_height}");

    let remainder_rock = start_rock + remainder;
    let (remainder_height,_,_) = tower_height(&pattern, remainder_rock, false).expect("remainder rock");
    let diff = remainder_height - start_height;
    println!("{remainder_rock} has a height of {remainder_height}. After we subtract {start_height} we get {diff}.");
    let total = start_height + cycles * delta_height + diff;
    println!("part2: {total}");

}
