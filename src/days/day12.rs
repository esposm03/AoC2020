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
                Forward(n) => match rot {
                    Rotation::North => y += n as isize,
                    Rotation::South => y -= n as isize,
                    Rotation::East => x += n as isize,
                    Rotation::West => x -= n as isize,
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

            (x, y, rot)
        });

    (x.abs() + y.abs()) as i64
}

pub fn day12_part2(input: &str) -> i64 {
    let mut wp_x = 10;
    let mut wp_y = 1;
    let mut x = 0;
    let mut y = 0;

    for action in input.lines().map(Action::parse) {
        match action {
            North(n) => wp_y += n as isize,
            South(n) => wp_y -= n as isize,
            East(n) => wp_x += n as isize,
            West(n) => wp_x -= n as isize,
            Forward(n) => {
                x += wp_x*n as isize;
                y += wp_y*n as isize;
            }
            Left(n) => {
                let num = n/90;
                for _ in 0..num {
                    let temp = wp_y;
                    wp_y = wp_x;
                    wp_x = -temp;
                }
            }
            Right(n) => {
                let num = n/90;
                for _ in 0..num {
                    let temp = wp_x;
                    wp_x = wp_y;
                    wp_y = -temp;
                }
            }
        };
    }

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
    assert_eq!(day12_part2("F10\nN3\nF7\nR90\nF11\n"), 286);
}
