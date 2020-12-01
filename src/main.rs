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

    println!("The result is: {}", match day {
        1 => days::day1(&input).unwrap(),
        2 => days::day1_part2(&input).unwrap(),
        _ => panic!("Invalid day provided"),
    });
}
