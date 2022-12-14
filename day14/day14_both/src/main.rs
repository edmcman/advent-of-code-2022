#[derive(Debug, Clone)]
struct Point {
    r: usize,
    c: usize,
}

impl Point {
    fn from_str(s: &str) -> Option<Self> {
        let (cstr, rstr) = s.split_once(',')?;
        Some(Self {
            r: rstr.parse().ok()?,
            c: cstr.parse().ok()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum GridPoint {
    Air,
    Rock,
    Sand,
}

#[derive(Debug)]
struct Grid(Vec<Vec<GridPoint>>);

impl Grid {
    // Create grid of air
    fn new(r: usize, c: usize) -> Self {
        let row = vec![GridPoint::Air; c];
        let g = vec![row; r];
        Self(g)
    }

    fn get(&self, r: usize, c: usize) -> &GridPoint {
        //dbg!((r,c));
        self.0.get(r).unwrap().get(c).unwrap()
    }

    fn getp(&self, p: &Point) -> &GridPoint {
        self.get(p.r, p.c)
    }

    fn get_mut(&mut self, r: usize, c: usize) -> &mut GridPoint {
        self.0.get_mut(r).unwrap().get_mut(c).unwrap()
    }
}

fn main() {
    let lines: Vec<_> = std::io::stdin().lines().map(|l| l.unwrap()).collect();
    let z: Vec<Vec<Point>> = lines
        .iter()
        .map(|l| {
            l.split(" -> ")
                .map(|point| Point::from_str(point).unwrap())
                .collect()
        })
        .collect();

    let max_r = z.iter().flatten().max_by_key(|p| p.r).unwrap().r;
    let max_c = z.iter().flatten().max_by_key(|p| p.c).unwrap().c;

    let grid = Grid::new(max_r + 10, max_c + 10);

    let grid: Grid = z.iter().fold(grid, |grid, segment| {
        segment.windows(2).fold(grid, |grid, pair| {
            let (a, b) = match pair {
                [a, b] => (a, b),
                _ => panic!("uh oh"),
            };

            println!("I should put Rocks between {:?} and {:?}", a, b);

            let range_helper = |a: usize, b: usize| if a < b { a..=b } else { b..=a };

            if a.r == b.r {
                let range = range_helper(a.c, b.c);
                range.fold(grid, |mut grid, c| {
                    //dbg!((a.r, c));
                    *grid.get_mut(a.r, c) = GridPoint::Rock;
                    grid
                })
            } else if a.c == b.c {
                let range = range_helper(a.r, b.r);
                range.fold(grid, |mut grid, r| {
                    //dbg!((r, a.c));
                    *grid.get_mut(r, a.c) = GridPoint::Rock;
                    grid
                })
            } else {
                panic!("diagonal???");
            }
        })
    });

    let sand_start_point = Point { r: 0, c: 500 };

    let mut grid = grid;

    let mut num_sand = -1;

    'outer: loop {
        let mut sand_point = sand_start_point.clone();
        num_sand += 1;

        loop {
            // A unit of sand always falls down one step if possible. If the tile
            // immediately below is blocked (by rock or sand), the unit of sand
            // attempts to instead move diagonally one step down and to the left. If
            // that tile is blocked, the unit of sand attempts to instead move
            // diagonally one step down and to the right. Sand keeps moving as long
            // as it is able to do so, at each step trying to move down, then
            // down-left, then down-right. If all three possible destinations are
            // blocked, the unit of sand comes to rest and no longer moves, at which
            // point the next unit of sand is created back at the source.
            let r = sand_point.r;
            let c = sand_point.c;

            //dbg!(r);

            let down = Point { r: r + 1, c: c };
            let down_left = Point { r: r + 1, c: c - 1 };
            let down_right = Point { r: r + 1, c: c + 1 };

            let next_point = [down, down_left, down_right].into_iter().find_map(|p| {
                if *grid.getp(&p) == GridPoint::Air {
                    Some(p)
                } else {
                    None
                }
            });

            match next_point {
                Some(next_point) => sand_point = next_point,
                None => {
                    // Cool, this is where the sand lands.
                    *grid.get_mut(sand_point.r, sand_point.c) = GridPoint::Sand;
                    break;
                }
            };

            if sand_point.r > max_r {
                println!("We done! {num_sand}");
                break 'outer;
            }

        }
    }

    //dbg!(&grid);

    //let max_r = z.iter().max_by_key(|row|

    //let z = z.chunks(2);
    //let z = z.map(|segment| )
}
