use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    // x=2, y=18
    fn from_str(s: &str) -> Option<Self> {
        let s = s
            .split_once(", ")
            .map(|(s1, s2)| [s1, s2].map(|s| s.split_once('=').map(|o| o.1)));
        match s {
            Some([Some(l), Some(r)]) => Some(Self {
                x: l.parse().ok()?,
                y: r.parse().ok()?,
            }),
            _ => None,
        }
    }

    fn to_tup(&self) -> (isize, isize) {
        (self.x, self.y)
    }

    fn distance(&self, o: &Point) -> isize {
        (o.y - self.y).abs() + (o.x - self.x).abs()
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Sensor {
    loc: Point,
    nearest_beacon: Point,
}

impl Sensor {
    fn from_str(s: &str) -> Option<Self> {
        // Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        let sp: Vec<_> = s.split([' ', ':']).collect();

        let sensor = &sp[2..4].join(" ");
        let beacon = &sp[9..11].join(" ");

        Some(Self {
            loc: Point::from_str(sensor)?,
            nearest_beacon: Point::from_str(beacon)?,
        })
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
enum GridPoint {
    Beacon,
    Sensor,
    Unknown,
    NotBeacon,
}

#[derive(Clone)]
struct Grid {
    m: HashMap<Point, GridPoint>,
}

impl Grid {
    fn count_notbeacon_in_row(&self, r: isize) -> usize {
        self.m
            .iter()
            .filter(|(k, v)| k.y == r && **v == GridPoint::NotBeacon)
            .count()
    }
}

fn main() {
    let s: Option<Vec<_>> = std::io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| Sensor::from_str(&l))
        .collect();
    let sensors = s.expect("Parsable");

    //let max_ref_x = s.iter().flat_map(|s| [s.loc.x, s.nearest_beacon.x]).max().expect("there are sensors");
    //let max_ref_y = s.iter().flat_map(|s| [s.loc.y, s.nearest_beacon.y]).max().expect("there are sensors");

    let grid = Grid { m: HashMap::new() };

    dbg!(&sensors);

    // Put beacons and sensors in the grid
    /*
    let grid = sensors.iter().fold(grid, |mut grid, sensor| {
        let e = grid.m.insert(sensor.loc.clone(), GridPoint::Sensor);
        assert!(dbg!(e).is_none());
        let e = grid.m.insert(sensor.nearest_beacon.clone(), GridPoint::Beacon);
        // Can be a duplicate beacon
        //assert!(dbg!(e).is_none());
        grid
    });*/

    // Mark where beacons cannot be
    let grid_p1 = sensors.iter().fold(grid.clone(), |grid, sensor| {
        let distance = sensor.loc.distance(&sensor.nearest_beacon);
        // We know there are no beacons of that distance or closer
        // I'm lazy so I'll loop over the box
        let yr = ((sensor.loc.y - distance)..(sensor.loc.y + distance));
        dbg!(&yr);
        if !yr.contains(&2000000) {
            dbg!("Skipping");
            grid
        } else {
            dbg!("Not skipping");
            ((sensor.loc.x - distance)..(sensor.loc.x + distance)).fold(grid, |grid, x| {
                (2000000..=2000000).fold(grid, |mut grid, y| {
                    //dbg!(&y);
                    let new_point = Point { x, y };
                    let new_point_dist = new_point.distance(&sensor.loc);
                    if new_point_dist <= distance
                        && new_point != sensor.loc
                        && new_point != sensor.nearest_beacon
                    {
                        let old_value = grid.m.insert(new_point, GridPoint::NotBeacon);
                        match old_value {
                            Some(GridPoint::Beacon) => panic!("how is this a beacon"),
                            Some(GridPoint::Sensor) => panic!("how is this a sensor"),
                            _ => (),
                        };
                        grid
                    } else {
                        grid
                    }
                })
            })
        }
    });

    //println!("grid row 10 {}", grid.count_notbeacon_in_row(10));
    println!("grid row 2000000 {}", grid_p1.count_notbeacon_in_row(2000000));
}
