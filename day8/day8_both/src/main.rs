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

fn view_distance_direction(f: &Forrest, d: &Direction, p: &Point) -> i32 {
    let look = look_in_direction(f, d, p);
    let height = f.get(p.r).unwrap().get(p.c).unwrap();

    let view: Vec<i32> = look.iter().take_while(|oh| *oh < height).copied().collect();
    // is there another item? after the view?  if so add one
    let score = if view.len() < look.len() { view.len() + 1 } else { view.len() };

    score.try_into().unwrap()
}

fn view_score(f: &Forrest, p: &Point) -> i32 {
    [
        Direction::Left,
        Direction::Right,
        Direction::Up,
        Direction::Down,
    ]
    .iter()
    .map(|d| view_distance_direction(f, d, p))
    .product()
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

    let vis: Vec<_> = point_iterator.clone()
    .inspect(|p| println!("p {:?} {}", p, is_visible(&f, &p)))
    .map(|p| is_visible(&f, &p))
        .collect();

    let nvis = vis.iter().filter(|b| **b).count();

    println!("part1: {:?}", nvis);

    let vis_score = point_iterator.clone().map(|p| view_score(&f, &p)).max().unwrap();

    println!("part 2: {vis_score}");
}
