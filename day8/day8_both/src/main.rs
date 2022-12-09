type Forrest = Vec<Vec<i32>>;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Point {
    r: usize,
    c: usize,
}

fn get_height(f: &Forrest, p: &Point) -> Option<i32> {
    f.get(p.r)?.get(p.c).copied()
}

fn look_in_direction(f: &Forrest, d: &Direction, p: &Point) -> Vec<i32> {
    let g = |r: usize, c: usize| f.get(r).unwrap().get(c).unwrap();

    match d {
        Direction::Up => (0..(p.r)).rev().map(|newr| *g(newr, p.c)).collect(),
        Direction::Down => ((p.r + 1)..(f.len())).map(|newr| *g(newr, p.c)).collect(),
        Direction::Left => (0..(p.c)).rev().map(|newc| *g(p.r, newc)).collect(),
        Direction::Right => ((p.c + 1)..(f.get(0).unwrap().len()))
            .map(|newc| *g(p.r, newc)).collect(),
    }
}

fn get_highest_in_direction(f: &Forrest, d: &Direction, p: &Point) -> i32 {
    let highest = look_in_direction(f, d, p).iter().max().copied();
    let highest = highest.unwrap_or(-1);
    highest
}

fn is_visible(f: &Forrest, p: &Point) -> bool {
    let h = get_height(f, p).unwrap();
    [
        Direction::Left,
        Direction::Right,
        Direction::Up,
        Direction::Down,
    ]
    .iter()
    .any(|d| h > get_highest_in_direction(f, &d, p))
}

fn main() {
    let l = std::io::stdin().lines().map(|l| l.unwrap());
    let l = l.map(|l| {
        l.chars()
            .map(|c| c.to_digit(10).unwrap())
            .map(|u| i32::try_from(u).unwrap())
            .collect::<Vec<_>>()
    });
    let f: Forrest = l.collect();

    let numr = f.len();
    let numc = f.get(0).unwrap().len();

    let point_iterator = (0..numr).flat_map(|r| (0..numc).map(move |c| Point { r, c }));

    let vis: Vec<_> = point_iterator
    .inspect(|p| println!("p {:?} {}", p, is_visible(&f, &p)))
    .map(|p| is_visible(&f, &p))
        //.filter(|b| *b)
        //.enumerate()
        .collect();
        //count();

    let nvis = vis.iter().filter(|b| **b).count();

    println!("part1: {:?}", nvis)
}
