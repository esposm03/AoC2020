#![allow(non_snake_case)]

use clap::{App, Arg};
use std::io;
use std::io::Read;

mod days;

fn main() {
    let args = App::new("AoC2020")
        .author("Samuele Esposito")
        .about("My solutions for Advent of Code 2020")
        .arg(
            Arg::with_name("DAY")
                .required(true)
                .help("Which day to run")
                .index(1),
        )
        .get_matches();

    let day = args
        .value_of("DAY")
        .unwrap()
        .parse()
        .expect("This doesn't look like a number :(");

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("IO error");

    println!(
        "The result is: {}",
        match day {
            1 => days::day1(&input).unwrap(),
            2 => days::day1_part2(&input).unwrap(),
            3 => days::day2(&input),
            4 => days::day2_part2(&input),
            5 => days::day3(&input),
            6 => days::day3_part2(&input),
            7 => days::day4(&input),
            8 => days::day4_part2(&input),
            9 => days::day5(&input),
            10 => days::day5_part2(&input),
            11 => days::day6(&input),
            12 => days::day6_part2(&input),
            13 => days::day7(&input),
            14 => days::day7_part2(&input),
            15 => days::day8(&input),
            16 => days::day8_part2(&input),
            17 => days::day9(&input, 25),
            18 => days::day9_part2(&input, 25),
            19 => days::day10(&input),
            20 => days::day10_part2(&input),
            21 => days::day11(&input),
            22 => days::day11_part2(&input),
            23 => days::day12(&input),
            _ => panic!("Invalid day provided"),
        }
    );
}
