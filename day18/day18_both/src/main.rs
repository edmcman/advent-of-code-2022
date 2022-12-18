use std::num;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point(usize, usize, usize);

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
    println!("Max dimension: {max_dimension}");

    let iter = cubes.iter().map(|cube| {
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
        dbg!(count);
        (cube, 6 - count)
    });

    let surface_area: usize = iter.map(|(_,count)| count).sum::<usize>();

    println!("Surface area: {surface_area}");

}
