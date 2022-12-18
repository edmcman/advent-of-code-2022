#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point(isize, isize, isize);

// return inside & outside points
fn flood(start: &Point, cubes: &Vec<Point>, max_dimension: isize) -> Vec<Point> {
    //println!("flood {:?}", start);

    let mut queue: Vec<Point> = vec![start.clone()];

    let mut outside_points = vec![];

    while let Some(next_point) = queue.pop() {
        //dbg!(&next_point);

        // Six directions...
        [
            (1, 0, 0),
            (0, 1, 0),
            (0, 0, 1),
            (-1, 0, 0),
            (0, -1, 0),
            (0, 0, -1),
        ]
        .map(|delta| {
            Point(
                next_point.0 + delta.0,
                next_point.1 + delta.1,
                next_point.2 + delta.2,
            )
        })
        .into_iter()
        // Only go where there are no cubes
        .filter(|newp| !cubes.contains(newp))
        .filter(|newp| {
            [newp.0, newp.1, newp.2]
                .iter()
                .all(|idx| *idx >= -1 && *idx <= max_dimension)
        })
        .filter(|p| !outside_points.contains(p))
        .for_each(|p| {
            if !queue.contains(&p) {
                //println!("Queueing {:?}", p);
                queue.push(p)
            }
        });

        outside_points.push(next_point);
    }

    outside_points.sort();

    outside_points
}

fn surface_area(cubes: &Vec<Point>) -> usize {
    cubes
        .iter()
        .map(|cube| {
            // count how many adjacent cubes there are
            let count = cubes
                .iter()
                .filter(|othercube| cube != *othercube)
                .filter(|othercube| {
                    cube.0.abs_diff(othercube.0)
                        + cube.1.abs_diff(othercube.1)
                        + cube.2.abs_diff(othercube.2)
                        == 1
                })
                .count();
            (cube, 6 - count)
        })
        .map(|(_, count)| count)
        .sum::<usize>()
}

fn main() {
    let cubes: Vec<Point> = std::io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|s| match s.split(",").collect::<Vec<_>>().as_slice() {
            [a, b, c] => Point(
                a.parse().expect("int"),
                b.parse().expect("int"),
                c.parse().expect("int"),
            ),
            _ => panic!("uh oh"),
        })
        .collect();

    let max_dimension = cubes
        .iter()
        .map(|p| [p.0, p.1, p.2].iter().max().unwrap().to_owned())
        .max()
        .unwrap();
    let min_dimension = cubes
        .iter()
        .map(|p| [p.0, p.1, p.2].iter().min().unwrap().to_owned())
        .min()
        .unwrap();
    println!("Min/max dimension: {min_dimension}/{max_dimension}");

    let p1_surface_area = surface_area(&cubes);

    println!("Part 1: surface area = {p1_surface_area}");

    let outside_point = Point(max_dimension + 1, max_dimension + 1, max_dimension + 1);

    let outside_points = flood(&outside_point, &cubes, max_dimension + 1);

    let p2: usize = cubes
        .iter()
        .map(|next_point| {
            // Six directions...
            [
                (1, 0, 0),
                (0, 1, 0),
                (0, 0, 1),
                (-1, 0, 0),
                (0, -1, 0),
                (0, 0, -1),
            ]
            .map(|delta| {
                Point(
                    next_point.0 + delta.0,
                    next_point.1 + delta.1,
                    next_point.2 + delta.2,
                )
            })
            .into_iter()
            // Only go where there are no cubes
            .filter(|newp| outside_points.contains(newp))
            .count()
        })
        .sum();

    println!("p2 {p2}");
}
