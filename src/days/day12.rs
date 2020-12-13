use Action::*;

pub fn day12(input: &str) -> i64 {
    // Coordinates: we have x, which is the east-west axis, positive when moving towards east
    // we have y, which is the north-south axis, positive when moving towards north
    let (x, y, _) = input
        .lines()
        .map(Action::parse)
        .fold((0isize, 0isize, Rotation::East), |(mut x, mut y, mut rot), elem| {
            match elem {
                North(n) => y += n as isize,
                South(n) => y -= n as isize,
                East(n) => x += n as isize,
                West(n) => x -= n as isize,
                Forward(n) => {
                    match rot {
                        Rotation::North => y += n as isize,
                        Rotation::South => y -= n as isize,
                        Rotation::East => x += n as isize,
                        Rotation::West => x -= n as isize,
                    }
                    /*
                    dbg!((rot%360).abs());
                    match (rot%360).abs() {
                        0 => y += n as isize,
                        90 => x += n as isize,
                        180 => y -= n as isize,
                        270 => x -= n as isize,
                        _ => unreachable!(),
                    }
                    */
                }
                Left(n) => {
                    let mut coord_num = match rot {
                        Rotation::North => 0,
                        Rotation::East => 90,
                        Rotation::South => 180,
                        Rotation::West => 270,
                    };

                    while coord_num - (n as isize) < 0 {
                        coord_num += 360;
                    }

                    rot = match (coord_num - n as isize) % 360 {
                        0 => Rotation::North,
                        90 => Rotation::East,
                        180 => Rotation::South,
                        270 => Rotation::West,
                        _ => unreachable!(),
                    };
                }
                Right(n) => {
                    let coord_num = match rot {
                        Rotation::North => 0,
                        Rotation::East => 90,
                        Rotation::South => 180,
                        Rotation::West => 270,
                    };

                    rot = match (coord_num + n) % 360 {
                        0 => Rotation::North,
                        90 => Rotation::East,
                        180 => Rotation::South,
                        270 => Rotation::West,
                        _ => unreachable!(),
                    };
                }
            };

            println!("With instruction {:?}, ship moves to ({}, {}, {:?})", elem, x, y, rot);

            (x, y, rot)
        });

    (x.abs() + y.abs()) as i64
}

#[derive(Debug, Eq, PartialEq)]
enum Rotation {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Eq, PartialEq)]
enum Action {
    North(usize),
    South(usize),
    East(usize),
    West(usize),
    Forward(usize),
    Left(usize),
    Right(usize),
}

impl Action {
    fn parse(i: &str) -> Action {
        let num = i.chars().skip(1).collect::<String>().parse().unwrap();
        match i.chars().next().unwrap() {
            'N' => North(num),
            'S' => South(num),
            'E' => East(num),
            'W' => West(num),
            'F' => Forward(num),
            'L' => Left(num),
            'R' => Right(num),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
#[test]
fn test() {
    assert_eq!(Action::parse("N32"), North(32));
    assert_eq!(day12("F10\nN3\nF7\nR90\nF11\n"), 25);

    assert_eq!(270 % 360, 270);
    assert_eq!(450 % 360, 90);
    assert_eq!(-90 % 360, -90);
}
